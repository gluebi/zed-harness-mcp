use std::env;
use zed_extension_api::{
    self as zed, Command, ContextServerConfiguration, ContextServerId, Project, Result,
    settings::ContextServerSettings,
};

const BINARY_NAME: &str = "harness-mcp-server";

struct HarnessMcpServer;

impl zed::Extension for HarnessMcpServer {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        // Use bundled binary
        let binary_path = env::current_dir()
            .unwrap()
            .join(BINARY_NAME)
            .to_string_lossy()
            .to_string();

        // Get user settings
        let settings = ContextServerSettings::for_project(context_server_id.as_ref(), project)?;

        // Build args with API key from settings
        let mut args = vec!["stdio".to_string(), "--toolsets=fme".to_string()];
        
        if let Some(settings_value) = settings.settings {
            if let Some(api_key) = settings_value.get("api_key").and_then(|v| v.as_str()) {
                if !api_key.is_empty() {
                    args.push(format!("--api-key={}", api_key));
                }
            }
        }

        Ok(Command {
            command: binary_path,
            args,
            env: vec![],
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
   {
     "context_servers": {
       "harness-mcp-server": {
         "settings": {
           "api_key": "your-split-io-admin-api-key"
         }
       }
     }
   }

2. Enable the context server in Zed Agent Panel settings

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
