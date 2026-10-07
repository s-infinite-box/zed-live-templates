mod installer;

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
        let binary = settings.binary;
        let path = match binary.as_ref().and_then(|binary| binary.path.clone()) {
            Some(path) => path,
            None => {
                let result = installer::install(id);
                if let Err(error) = &result {
                    zed::set_language_server_installation_status(
                        id,
                        &zed::LanguageServerInstallationStatus::Failed(error.clone()),
                    );
                }
                result?
            }
        };
        let mut env = worktree.shell_env();
        let mut args = Vec::new();
        if let Some(binary) = binary {
            env.extend(binary.env.unwrap_or_default());
            args = binary.arguments.unwrap_or_default();
        }
        Ok(zed::Command {
            command: path,
            args,
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
