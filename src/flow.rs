use std::path::Path;

use zed::LanguageServerId;
use zed_extension_api::{self as zed, Result, serde_json};

struct FlowExtension {}

impl FlowExtension {
    const WORKTREE_SERVER_PATH: &'static str = "node_modules/.bin/flow";
    const PACKAGE_NAME: &'static str = "flow-bin";

    fn worktree_flow_exists(&self, worktree: &zed::Worktree) -> bool {
        let package_json = worktree
            .read_text_file("package.json")
            .unwrap_or(String::from(r#"{}"#));

        let package_json: Option<serde_json::Value> =
            serde_json::from_str(package_json.as_str()).ok();

        package_json.is_some_and(|f| {
            !f["dependencies"][Self::PACKAGE_NAME].is_null()
                || !f["devDependencies"][Self::PACKAGE_NAME].is_null()
        })
    }

    fn worktree_flowconfig_exists(&self, worktree: &zed::Worktree) -> bool {
        worktree.read_text_file(".flowconfig").is_ok()
    }
}

impl zed::Extension for FlowExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        if !self.worktree_flowconfig_exists(worktree) {
            // TODO: skip silently
            return Err(".flowconfig file not found".to_string());
        }

        // primary: run flow binary from node_modules in worktree
        if self.worktree_flow_exists(worktree) {
            let server_path = Path::new(worktree.root_path().as_str())
                .join(Self::WORKTREE_SERVER_PATH)
                .to_string_lossy()
                .to_string();

            return Ok(zed::Command {
                command: server_path,
                args: vec!["lsp".to_string()],
                env: Default::default(),
            });
        }

        // fallback: use flow from $PATH
        if let Some(path) = worktree.which("flow") {
            return Ok(zed::Command {
                command: path,
                args: vec!["lsp".to_string()],
                env: Default::default(),
            });
        }

        Err("Flow binary not found. Please install the npm package 'flow-bin'".to_string())
    }
}

zed::register_extension!(FlowExtension);
