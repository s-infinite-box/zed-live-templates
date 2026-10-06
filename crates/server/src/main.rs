mod documents;

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Result;
use engine::{config::Config, context, renderer};
use serde_json::json;
use tokio::sync::{Mutex, Semaphore};
use tower_lsp::{Client, LanguageServer, LspService, Server, jsonrpc, lsp_types::*};

use documents::{Document, byte_offset, trigger_start};

struct State {
    config_path: PathBuf,
    source: Option<String>,
    config: Option<Arc<Config>>,
    documents: HashMap<Url, Document>,
    root: Option<PathBuf>,
    as_is: bool,
}

impl State {
    fn reload(&mut self) {
        let source = match std::fs::read_to_string(&self.config_path) {
            Ok(source) => source,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                self.source = None;
                self.config = None;
                return;
            }
            Err(error) => {
                eprintln!("{}: {error}", self.config_path.display());
                return;
            }
        };
        if self.source.as_ref() == Some(&source) {
            return;
        }
        let directory = self.config_path.parent().unwrap().to_path_buf();
        match Config::parse(&source, directory) {
            Ok(config) => self.config = Some(Arc::new(config)),
            Err(error) => eprintln!("{}: {error:#}", self.config_path.display()),
        }
        self.source = Some(source);
    }
}

struct Backend {
    state: Mutex<State>,
    execution: Semaphore,
    cli_config: bool,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    #[allow(deprecated)]
    async fn initialize(&self, params: InitializeParams) -> jsonrpc::Result<InitializeResult> {
        let mut state = self.state.lock().await;
        if !self.cli_config
            && let Some(path) = params
                .initialization_options
                .as_ref()
                .and_then(|options| options.get("config_path"))
                .and_then(|path| path.as_str())
        {
            state.config_path = absolute(Path::new(path));
        }
        state.root = params
            .workspace_folders
            .as_ref()
            .and_then(|folders| folders.first().map(|folder| &folder.uri))
            .or(params.root_uri.as_ref())
            .and_then(|uri| uri.to_file_path().ok());
        state.as_is = params
            .capabilities
            .text_document
            .as_ref()
            .and_then(|document| document.completion.as_ref())
            .and_then(|completion| completion.completion_item.as_ref())
            .and_then(|item| item.insert_text_mode_support.as_ref())
            .is_some_and(|modes| modes.value_set.contains(&InsertTextMode::AS_IS));
        state.reload();
        Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: "Live Templates".into(),
                version: Some("0.1.0".into()),
            }),
            capabilities: ServerCapabilities {
                position_encoding: Some(PositionEncodingKind::UTF16),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions::default()),
                ..Default::default()
            },
        })
    }

    async fn shutdown(&self) -> jsonrpc::Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let document = params.text_document;
        self.state.lock().await.documents.insert(
            document.uri,
            Document {
                text: document.text,
                language: document.language_id,
                version: document.version,
            },
        );
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Some(document) = self
            .state
            .lock()
            .await
            .documents
            .get_mut(&params.text_document.uri)
        {
            for change in params.content_changes {
                document.text = change.text;
            }
            document.version = params.text_document.version;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.state
            .lock()
            .await
            .documents
            .remove(&params.text_document.uri);
    }

    async fn completion(
        &self,
        params: CompletionParams,
    ) -> jsonrpc::Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        let (config, document, root, as_is) = {
            let mut state = self.state.lock().await;
            state.reload();
            let (Some(config), Some(document)) = (&state.config, state.documents.get(&uri)) else {
                return Ok(None);
            };
            (
                config.clone(),
                document.clone(),
                state.root.clone(),
                state.as_is,
            )
        };
        let Some(line) = document.text.split('\n').nth(position.line as usize) else {
            return Ok(None);
        };
        let Some(offset) = byte_offset(line, position.character) else {
            return Ok(None);
        };
        // 重叠缩写优先使用最长匹配，避免配置顺序影响展开。
        let Some((template, start)) = config
            .templates
            .iter()
            .filter_map(|template| {
                if !template
                    .languages
                    .iter()
                    .any(|language| language.eq_ignore_ascii_case(&document.language))
                {
                    return None;
                }
                trigger_start(line, offset, &template.trigger).map(|start| (template, start))
            })
            .max_by_key(|(template, _)| template.trigger.len())
        else {
            return Ok(None);
        };

        // 一次只展开一个候选，避免补全刷新同时启动大量脚本。
        let _permit = self.execution.acquire().await.unwrap();
        let path = uri.to_file_path().ok();
        let ctx = match context::create(
            &config.formats,
            template,
            json!({
                "uri": uri, "path": path,
                "name": path.as_ref().and_then(|path| path.file_name()).map(|name| name.to_string_lossy()),
                "language": document.language, "version": document.version,
                "line": line.trim_end_matches('\r')
            }),
            json!({ "line": position.line, "character": position.character, "encoding": "utf-16" }),
            root.as_deref(),
        ) {
            Ok(ctx) => ctx,
            Err(error) => {
                eprintln!("context: {error:#}");
                return Ok(None);
            }
        };
        let snippet = match renderer::render(&config, template, ctx).await {
            Ok(snippet) => snippet,
            Err(error) => {
                eprintln!("{error:#}");
                return Ok(None);
            }
        };
        let state = self.state.lock().await;
        if !state
            .documents
            .get(&uri)
            .is_some_and(|current| current.version == document.version)
        {
            return Ok(None);
        }
        let item = CompletionItem {
            label: template.trigger.clone(),
            detail: Some(template.description.clone()),
            kind: Some(CompletionItemKind::SNIPPET),
            filter_text: Some(template.trigger.clone()),
            sort_text: Some(format!("0_{}", template.trigger)),
            preselect: Some(true),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            insert_text_mode: as_is.then_some(InsertTextMode::AS_IS),
            text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                range: Range {
                    start: Position {
                        line: position.line,
                        character: line[..start].encode_utf16().count() as u32,
                    },
                    end: position,
                },
                new_text: snippet,
            })),
            ..Default::default()
        };
        Ok(Some(CompletionResponse::Array(vec![item])))
    }
}

fn absolute(path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.into()
    } else {
        std::env::current_dir().unwrap().join(path)
    }
}

fn default_config() -> PathBuf {
    let directory = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(std::env::var_os("HOME").unwrap()).join(".config"));
    directory.join("zed-live-templates/templates.toml")
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let mut config = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--config" => {
                config =
                    Some(absolute(Path::new(&args.next().ok_or_else(|| {
                        anyhow::anyhow!("--config requires a path")
                    })?)))
            }
            "--help" => {
                println!(
                    "server [--config templates.toml]\nRuns the Live Templates language server over stdin/stdout."
                );
                return Ok(());
            }
            _ => anyhow::bail!("unknown argument: {arg}"),
        }
    }
    let cli_config = config.is_some();
    let state = State {
        config_path: config.unwrap_or_else(default_config),
        source: None,
        config: None,
        documents: HashMap::new(),
        root: None,
        as_is: false,
    };
    let (service, socket) = LspService::new(|_: Client| Backend {
        state: Mutex::new(state),
        execution: Semaphore::new(1),
        cli_config,
    });
    Server::new(tokio::io::stdin(), tokio::io::stdout(), socket)
        .serve(service)
        .await;
    Ok(())
}
