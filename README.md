# Atlassian Rovo MCP Server for Zed

Thin Zed extension that exposes Atlassian's hosted Rovo MCP server inside Zed.

## Shape

This repository follows the project shape recorded in `docs/adr/0001-project-shape.md`:

- **Zed extension** as the product surface
- **Atlassian-hosted Rovo MCP server** as the server of record
- **`mcp-remote`** as the initial bridge strategy

The extension does not implement its own Atlassian MCP server. It starts a local bridge process that connects Zed to Atlassian's remote MCP endpoint.

## Current implementation

This repository currently provides:

- `extension.toml` manifest for a Zed MCP extension
- `src/lib.rs` Rust entrypoint using `zed_extension_api`
- a single context server: `atlassian-rovo`
- setup instructions and a JSON schema for extension settings
- default launch behavior using Zed's managed Node runtime plus an extension-local `mcp-remote@0.1.37` bridge targeting `https://mcp.atlassian.com/v1/mcp/authv2 --debug`

## Prerequisites

- [Zed](https://zed.dev)
- Rust installed via `rustup`
- `wasm32-wasip2` target: `rustup target add wasm32-wasip2`
- Node.js 18+ available to Zed
- An Atlassian Cloud site with Jira, Confluence, Compass, or related supported products
- A browser available for OAuth 2.1 sign-in

## Local development

1. Open Zed.
2. Run `zed: extensions`.
3. Click `Install Dev Extension`.
4. Select this repository.
5. Open the Agent Panel and enable the `Atlassian Rovo` context server.
6. Complete the browser auth flow when prompted.

For extra logs, launch Zed from a terminal with `zed --foreground`.

The extension also sets a platform-appropriate `BROWSER` environment variable (`open` on macOS, `xdg-open` on Linux) to make browser launch more reliable when Zed is started outside a shell.

## Configuration

The extension exposes a small settings surface:

```json
{
  "context_servers": {
    "atlassian-rovo": {
      "settings": {
        "endpoint": "https://mcp.atlassian.com/v1/mcp/authv2",
        "proxy_package": "mcp-remote@0.1.37"
      }
    }
  }
}
```

### Notes

- `endpoint` defaults to Atlassian's recommended MCP endpoint.
- `proxy_package` defaults to `mcp-remote@0.1.37`.
- Advanced users can override the spawned command through Zed's standard `context_servers.<id>.command` settings, but doing so bypasses the default auto-installed bridge path.

## Validation and testing

For release gating and local test runs, use:

- `docs/release-checklist.md` — operator-friendly release/smoke-test checklist
- `docs/testing/local-dev.md` — step-by-step local dev-extension testing guide

Quick smoke-test prompts:

- `List my Jira issues`
- `Search Confluence for onboarding docs`
- `Summarize the latest work in Compass`

If authentication fails, restart the flow and check that Node.js is available to Zed and that your Atlassian site access is valid.

For browser-launch problems, inspect the `mcp-remote` debug logs in `~/.mcp-auth/*_debug.log`. They include the authorization URL and any browser-open failure details.

If the bridge package is not yet installed, the first run may spend a few seconds installing `mcp-remote` into the extension's local `node_modules` before the context server starts.

## License

Apache-2.0. See `LICENSE`.
