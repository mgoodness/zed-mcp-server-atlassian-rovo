# Local development testing guide

Use this guide when testing `zed-mcp-server-atlassian-rovo` locally in Zed.

This document is the step-by-step companion to `docs/release-checklist.md`.

## Goal

Verify that the extension can:

1. build successfully
2. install as a dev extension in Zed
3. start the `atlassian-rovo` context server
4. complete the normal OAuth 2.1 flow
5. execute one Jira workflow and one Confluence workflow

## Prerequisites

Before testing, make sure you have:

- Zed installed
- Rust installed via `rustup`
- the `wasm32-wasip2` target installed
- Node.js 18+ available to Zed
- access to an Atlassian Cloud environment with Jira and Confluence
- a browser available for OAuth sign-in

## 1. Build sanity check

From the repository root, run:

```sh
cargo check
```

Expected result:

- the command completes successfully
- no manifest or crate errors block the scaffold

If this fails, stop here and fix the build issue before testing in Zed.

## 2. Install the extension in Zed

1. Open Zed.
2. Run `zed: extensions` from the command palette.
3. Click `Install Dev Extension`.
4. Select the repository root.

Expected result:

- Zed installs the extension without manifest errors
- the extension appears as `Atlassian Rovo MCP Server`

If installation fails, check Zed logs and confirm that `extension.toml` and the Rust scaffold are present.

## 3. Confirm the context server appears

1. Open the Agent Panel.
2. Look for the `Atlassian Rovo` / `atlassian-rovo` context server.
3. Open any available configuration/setup surface.

Expected result:

- the context server is visible
- the setup text is sensible
- the default values match the current scaffold

Current defaults:

- endpoint: `https://mcp.atlassian.com/v1/mcp/authv2`
- bridge package: `mcp-remote@0.1.37`

## 4. Start the bridge

When enabled, the extension should launch Zed's Node runtime with the extension-local `mcp-remote` bridge, targeting:

```text
https://mcp.atlassian.com/v1/mcp/authv2 --debug
```

Expected result:

- Zed can spawn the process
- the process starts without immediately failing due to missing Node.js access
- if it fails, the failure is visible in logs or error output

### Common failure checks

- Node.js is unavailable to Zed
- the extension could not install the pinned `mcp-remote` package into its local `node_modules`
- local network/browser restrictions interfere with auth startup
- browser launch fails inside Zed; inspect `~/.mcp-auth/*_debug.log` for the auth URL and browser-open errors
- the endpoint was manually overridden incorrectly

## 5. Complete the OAuth flow

Use the normal Atlassian sign-in flow when prompted.

Expected result:

- browser login starts
- sign-in completes successfully
- the session returns to a usable authenticated state in Zed

If auth fails:

- retry the flow once
- confirm your Atlassian account has access to the relevant site/products
- check for popup, localhost redirect, or browser-blocking issues

## 6. Run the V1 smoke tests

After authentication succeeds, run the following prompts in Zed.

### Jira smoke test

```text
List my Jira issues
```

Expected result:

- the request succeeds
- returned data is consistent with the tester's Atlassian permissions

### Confluence smoke test

```text
Search Confluence for onboarding docs
```

Expected result:

- the request succeeds
- returned data is consistent with the tester's Atlassian permissions

## 7. Optional best-effort checks

These are useful, but not V1 release blockers.

### Compass

Example prompt:

```text
Summarize the latest work in Compass
```

### Advanced auth path

If your environment supports API-token auth and you deliberately test it, record:

- how it was configured
- whether it worked
- whether it required undocumented steps

## 8. Record the outcome

Capture results in `docs/release-checklist.md` or your release notes for the candidate.

At minimum, record:

- Zed version
- operating system
- Node.js version
- whether OAuth succeeded
- Jira smoke-test outcome
- Confluence smoke-test outcome
- any non-blocking issues discovered

## Troubleshooting tips

- Launch Zed from a terminal with `zed --foreground` for extra logs.
- Use Zed logs to distinguish installation failures from bridge-start failures.
- If auth starts but the prompts fail afterward, verify Atlassian product access and permissions before assuming the extension is broken.
- If the bridge command was overridden in settings, revert to defaults before diagnosing deeper issues.
