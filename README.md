# Harness MCP Server for Zed

A Zed extension that connects to [Harness MCP Server](https://github.com/harness/harness-mcp) for Feature Management and Experimentation (Split.io) integration.

## Features

- **Feature Flags:** List, create, and manage feature flags
- **Workspaces:** Browse Split.io workspaces and environments
- **Rollouts:** Control feature flag rollout status and traffic splits

## Prerequisites

- A [Split.io](https://www.split.io/) account
- Split.io Admin API Key

## Installation

Install from Zed Extensions: `zed://extension/harness-mcp-server`

Or search for "Harness MCP Server" in Zed's extension panel.

## Setup

1. Create a `.env` file in your project root:
   ```
   HARNESS_API_KEY=your-split-admin-api-key
   ```

2. Enable the context server in Zed's Agent Panel settings

3. The extension will automatically download the Harness MCP binary on first use

## Getting Your API Key

1. Log in to [Split.io](https://app.split.io)
2. Go to **Admin Settings** → **API Keys**
3. Create or copy an **Admin API Key**

## Available Tools

| Tool | Description |
|------|-------------|
| `list_fme_workspaces` | List all Split.io workspaces |
| `list_fme_environments` | List environments for a workspace |
| `list_fme_feature_flags` | List feature flags for a workspace |
| `get_fme_feature_flag_definition` | Get flag definition in an environment |
| `get_fme_rollout_statuses` | Get available rollout statuses |
| `create_fme_feature_flag` | Create a new feature flag |
| `create_fme_feature_flag_definition` | Initialize a flag in an environment |
| `update_fme_feature_flag` | Update rollout status or description |

## Example Queries

- "List my FME workspaces"
- "Create a feature flag called new-checkout-flow"
- "Initialize new-checkout-flow in Staging with a 50/50 split"
- "Set the rollout status of new-checkout-flow to Experimenting"

## License

MIT
