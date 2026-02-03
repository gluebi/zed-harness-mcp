use std::fs;
use std::env;
use zed_extension_api::{
    self as zed, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

const VERSION: &str = "1.0.0";
const BINARY_NAME: &str = "harness-mcp-server";
const RELEASE_BASE_URL: &str = "https://github.com/myposter-de/mcp/releases/download/v1.0.0";

struct HarnessMcpServer;

fn get_asset_name() -> String {
    let os = match std::env::consts::OS {
        "macos" => "darwin",
        "linux" => "linux",
        "windows" => "windows",
        os => os,
    };
    let arch = match std::env::consts::ARCH {
        "x86_64" => "amd64",
        "aarch64" => "arm64",
        arch => arch,
    };
    let ext = if os == "windows" { "zip" } else { "tar.gz" };
    format!("{}_{}_{}_{}.{}", BINARY_NAME, VERSION, os, arch, ext)
}

fn read_env_file() -> Vec<(String, String)> {
    let mut env_vars = Vec::new();
    
    if let Ok(content) = fs::read_to_string(".env") {
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim().to_string();
                let value = value.trim().trim_matches('"').trim_matches(''').to_string();
                env_vars.push((key, value));
            }
        }
    }
    
    env_vars
}

impl zed::Extension for HarnessMcpServer {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Command> {
        let asset_name = get_asset_name();
        let download_url = format!("{}/{}", RELEASE_BASE_URL, asset_name);
        
        let binary_path = format!("{}", BINARY_NAME);
        
        if !std::path::Path::new(&binary_path).exists() {
            zed::download_file(
                &download_url,
                &asset_name,
                zed::DownloadedFileType::GzipTar,
            )?;
        }
        
        let env_vars = read_env_file();
        
        let current_dir = env::current_dir().unwrap();
        let full_binary_path = current_dir.join(BINARY_NAME).to_string_lossy().to_string();
        
        Ok(Command {
            command: full_binary_path,
            args: vec!["stdio".to_string(), "--toolsets=fme".to_string()],
            env: env_vars,
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        Ok(Some(ContextServerConfiguration {
            installation_instructions: r#"# Harness MCP Server

This extension connects Zed to Harness MCP Server for Feature Management and Experimentation (Split.io) integration.

## Requirements

- A Harness/Split.io account
- Split.io Admin API Key

## Setup

1. Create a .env file in your project root with your API key:
   HARNESS_API_KEY=your-split-admin-api-key

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
1. Check that your .env file contains a valid HARNESS_API_KEY
2. Restart Zed
3. Re-enable the context server
"#.to_string(),
            default_settings: "{}".to_string(),
            settings_schema: "{}".to_string(),
        }))
    }
}

zed::register_extension!(HarnessMcpServer);
