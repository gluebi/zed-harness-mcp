use std::env;
use zed_extension_api::{
    self as zed, Command, ContextServerConfiguration, ContextServerId, Project, Result,
    settings::ContextServerSettings,
};

const VERSION: &str = "1.0.0";
const BINARY_NAME: &str = "harness-mcp-server";
const RELEASE_BASE_URL: &str = "https://github.com/myposter-de/mcp/releases/download/v1.0.0";

struct HarnessMcpServer;

fn get_asset_name() -> String {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    
    let os_name = match os {
        "macos" => "darwin",
        _ => os,
    };
    let arch_name = match arch {
        "x86_64" => "amd64",
        "aarch64" => "arm64",
        _ => arch,
    };
    let ext = if os == "windows" { "zip" } else { "tar.gz" };
    format!("{}_{}_{}_{}.{}", BINARY_NAME, VERSION, os_name, arch_name, ext)
}

impl zed::Extension for HarnessMcpServer {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let asset_name = get_asset_name();
        let download_url = format!("{}/{}", RELEASE_BASE_URL, asset_name);

        // Download and extract if binary doesn't exist
        zed::download_file(
            &download_url,
            &asset_name,
            zed::DownloadedFileType::GzipTar,
        ).ok();

        let binary_path = env::current_dir()
            .unwrap()
            .join(BINARY_NAME)
            .to_string_lossy()
            .to_string();

        // Get user settings
        let settings = ContextServerSettings::for_project(context_server_id.as_ref(), project)?;
        
        // Extract API key from settings
        let mut env_vars: Vec<(String, String)> = vec![];
        if let Some(settings_value) = settings.settings {
            if let Some(api_key) = settings_value.get("api_key").and_then(|v| v.as_str()) {
                env_vars.push(("HARNESS_API_KEY".to_string(), api_key.to_string()));
            }
        }

        Ok(Command {
            command: binary_path,
            args: vec!["stdio".to_string(), "--toolsets=fme".to_string()],
            env: env_vars,
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let settings_schema = serde_json::json!({
            "type": "object",
            "properties": {
                "api_key": {
                    "type": "string",
                    "description": "Your Split.io Admin API Key"
                }
            },
            "required": ["api_key"]
        });

        let default_settings = serde_json::json!({
            "api_key": ""
        });

        Ok(Some(ContextServerConfiguration {
            installation_instructions: r#"# Harness MCP Server

This extension connects Zed to Harness MCP Server for Feature Management and Experimentation (Split.io) integration.

## Requirements

- A Harness/Split.io account
- Split.io Admin API Key

## Setup

1. Add your API key to Zed settings (Settings > Open Settings):



2. Enable the context server in Zed Agent Panel settings

3. The extension will automatically download the Harness MCP binary on first use

## Getting Your API Key

1. Log in to Split.io (https://app.split.io)
2. Go to Admin Settings -> API Keys
3. Create or copy an Admin API Key

## Available Tools

- list_fme_workspaces: List all Split.io workspaces
- list_fme_environments: List environments for a workspace
- list_fme_feature_flags: List feature flags for a workspace
- get_fme_feature_flag_definition: Get flag definition in an environment
- create_fme_feature_flag: Create a new feature flag
- update_fme_feature_flag: Update rollout status or description

## Troubleshooting

If tools do not appear:
1. Check that your api_key is correctly set in Zed settings
2. Restart Zed
3. Re-enable the context server
"#.to_string(),
            default_settings: default_settings.to_string(),
            settings_schema: settings_schema.to_string(),
        }))
    }
}

zed::register_extension!(HarnessMcpServer);
