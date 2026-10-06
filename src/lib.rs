use zed_extension_api::{self as zed, settings::LspSettings};

struct Extension;

impl zed::Extension for Extension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let settings = LspSettings::for_worktree(id.as_ref(), worktree)?;
        let path = settings
            .binary
            .as_ref()
            .and_then(|binary| binary.path.clone())
            .ok_or_else(|| {
                "请在 Zed 的 lsp.templates.binary.path 中配置 server 可执行文件的绝对路径"
                    .to_owned()
            })?;
        let binary = settings.binary.unwrap();
        let mut env = worktree.shell_env();
        env.extend(binary.env.unwrap_or_default());
        Ok(zed::Command {
            command: path,
            args: binary.arguments.unwrap_or_default(),
            env,
        })
    }

    fn language_server_initialization_options(
        &mut self,
        id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<Option<zed::serde_json::Value>> {
        Ok(LspSettings::for_worktree(id.as_ref(), worktree)?.initialization_options)
    }
}

zed::register_extension!(Extension);
