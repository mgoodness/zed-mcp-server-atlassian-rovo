use serde::Deserialize;
use std::env;
use zed::settings::ContextServerSettings;
use zed_extension_api as zed;

const SERVER_ID: &str = "atlassian-rovo";
const DEFAULT_ENDPOINT: &str = "https://mcp.atlassian.com/v1/mcp/authv2";
const DEFAULT_PROXY_PACKAGE: &str = "mcp-remote@0.1.37";
const DEFAULT_PROXY_ENTRYPOINT: &str = "dist/proxy.js";

struct AtlassianRovoExtension {
    installed_proxy_package: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct AtlassianRovoSettings {
    endpoint: Option<String>,
    proxy_package: Option<String>,
}

impl AtlassianRovoSettings {
    fn from_value(value: Option<zed::serde_json::Value>) -> zed::Result<Self> {
        match value {
            Some(value) => zed::serde_json::from_value(value).map_err(|error| error.to_string()),
            None => Ok(Self::default()),
        }
    }

    fn endpoint(&self) -> String {
        self.endpoint
            .clone()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| DEFAULT_ENDPOINT.to_string())
    }

    fn proxy_package(&self) -> String {
        self.proxy_package
            .clone()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| DEFAULT_PROXY_PACKAGE.to_string())
    }
}

fn default_env() -> Vec<(String, String)> {
    match zed::current_platform().0 {
        zed::Os::Mac => vec![("BROWSER".to_string(), "open".to_string())],
        zed::Os::Linux => vec![("BROWSER".to_string(), "xdg-open".to_string())],
        zed::Os::Windows => Vec::new(),
    }
}

fn parse_package_spec(spec: &str) -> (&str, &str) {
    match spec.rfind('@') {
        Some(index) if index > 0 => (&spec[..index], &spec[index + 1..]),
        _ => (spec, "latest"),
    }
}

fn package_entrypoint(package_name: &str) -> zed::Result<String> {
    let package_path = env::current_dir()
        .map_err(|error| error.to_string())?
        .join("node_modules")
        .join(package_name)
        .join(DEFAULT_PROXY_ENTRYPOINT);

    Ok(package_path.to_string_lossy().to_string())
}

impl AtlassianRovoExtension {
    fn install_proxy_package_if_needed(&mut self, package_spec: &str) -> zed::Result<String> {
        let (package_name, requested_version) = parse_package_spec(package_spec);
        let desired_version = if requested_version == "latest" {
            zed::npm_package_latest_version(package_name)?
        } else {
            requested_version.to_string()
        };

        let installed_version = zed::npm_package_installed_version(package_name)?;
        let should_install = installed_version.as_deref() != Some(desired_version.as_str())
            || self.installed_proxy_package.as_deref() != Some(package_spec);

        if should_install {
            zed::npm_install_package(package_name, &desired_version)?;
        }

        self.installed_proxy_package = Some(package_spec.to_string());
        package_entrypoint(package_name)
    }
}

impl zed::Extension for AtlassianRovoExtension {
    fn new() -> Self {
        Self {
            installed_proxy_package: None,
        }
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &zed::ContextServerId,
        project: &zed::Project,
    ) -> zed::Result<zed::Command> {
        let config = ContextServerSettings::for_project(SERVER_ID, project)?;
        let settings = AtlassianRovoSettings::from_value(config.settings)?;

        if let Some(command_override) = config.command {
            let mut env = default_env();
            env.extend(command_override.env.unwrap_or_default());

            return Ok(zed::Command {
                command: command_override.path.unwrap_or_else(|| "npx".to_string()),
                args: command_override.arguments.unwrap_or_default(),
                env,
            });
        }

        let proxy_entrypoint = self.install_proxy_package_if_needed(&settings.proxy_package())?;

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![proxy_entrypoint, settings.endpoint(), "--debug".to_string()],
            env: default_env(),
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &zed::ContextServerId,
        _project: &zed::Project,
    ) -> zed::Result<Option<zed::ContextServerConfiguration>> {
        let installation_instructions = r#"
# Atlassian Rovo setup

This extension starts a local `mcp-remote` bridge and connects it to Atlassian's hosted Rovo MCP server.

## Requirements

- Node.js 18+ available to Zed
- an Atlassian Cloud site with Jira, Confluence, Compass, or another supported Atlassian product
- a browser for the OAuth 2.1 sign-in flow

## What happens on first run

Zed will install the configured `mcp-remote` package into the extension's local `node_modules` (if needed), then launch the bridge with Zed's managed Node runtime against:

```text
https://mcp.atlassian.com/v1/mcp/authv2 --debug
```

That bridge opens the Atlassian auth flow in your browser and forwards MCP traffic between Zed and Atlassian.
The extension also sets a platform-appropriate `BROWSER` environment variable to make browser launch more reliable.

## Validation

After setup, try prompts like:

- `List my Jira issues`
- `Search Confluence for onboarding docs`
- `Summarize the latest Compass updates`

## Notes

- This extension does not store secrets in the repository.
- Atlassian authorization and permissions remain the source of truth.
- `mcp-remote` debug logs are written under `~/.mcp-auth/`.
- Advanced users can override the command Zed spawns through `context_servers.atlassian-rovo.command` in Zed settings, but doing so bypasses the default auto-installed bridge path.
"#
        .trim()
        .to_string();

        let settings_schema = zed::serde_json::json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "additionalProperties": false,
            "properties": {
                "endpoint": {
                    "type": "string",
                    "title": "Atlassian MCP endpoint",
                    "description": "Remote Atlassian Rovo MCP endpoint for the bridge to connect to.",
                    "default": DEFAULT_ENDPOINT
                },
                "proxy_package": {
                    "type": "string",
                    "title": "Bridge package",
                    "description": "npm package spec to auto-install and launch through Zed's managed Node runtime for the local mcp-remote bridge.",
                    "default": DEFAULT_PROXY_PACKAGE
                }
            }
        })
        .to_string();

        let default_settings = zed::serde_json::json!({
            "endpoint": DEFAULT_ENDPOINT,
            "proxy_package": DEFAULT_PROXY_PACKAGE
        })
        .to_string();

        Ok(Some(zed::ContextServerConfiguration {
            installation_instructions,
            settings_schema,
            default_settings,
        }))
    }
}

zed::register_extension!(AtlassianRovoExtension);
