# ADR 0002: Bridge strategy

- Status: Accepted
- Date: 2026-05-28

## Context

`docs/adr/0001-project-shape.md` established that this repository will be a thin Zed extension that connects to Atlassian's hosted Rovo MCP server through a local bridge rather than implementing its own Atlassian MCP server.

The remaining bridge decision was how the extension should connect Zed's local MCP process model to Atlassian's remote MCP endpoint.

The relevant constraints are:

- Zed MCP extensions can start a local command for a context server.
- Atlassian documents a local-client strategy based on `mcp-remote` for IDEs and other desktop tools.
- Atlassian's current recommended endpoint for local clients is `https://mcp.atlassian.com/v1/mcp/authv2`.
- Authentication and authorization should remain owned by Atlassian, not reimplemented here.
- This project should optimize for fast setup, small maintenance surface, and a Zed-native installation experience.

## Decision

The extension will standardize on a **local `mcp-remote` bridge** as the default transport strategy.

The default bridge shape is:

```text
Zed-managed Node runtime + extension-local `mcp-remote@0.1.37` targeting https://mcp.atlassian.com/v1/mcp/authv2
```

In product terms, this means:

- Zed launches a local stdio command.
- That command runs the locally installed `mcp-remote` bridge through Zed's managed Node runtime.
- `mcp-remote` connects to Atlassian's hosted MCP endpoint.
- Atlassian handles OAuth 2.1 and permission enforcement.

The extension will expose this as the default behavior, while still allowing advanced users to override the command or endpoint through Zed's context-server settings.

## Rationale

This is the best fit for the current project shape.

- **Matches Zed's extension model.** Zed is good at launching local commands for context servers.
- **Matches Atlassian's guidance.** Atlassian already documents `mcp-remote` as the bridge for local IDE-style clients.
- **Avoids transport reimplementation.** The repo does not need to build or maintain its own remote-MCP proxy.
- **Preserves vendor boundaries.** Auth, authorization, and server behavior stay owned by Atlassian.
- **Minimizes initial code.** The extension can focus on setup UX, validation guidance, and packaging.
- **Keeps future options open.** If Zed later supports a direct remote MCP path that makes the bridge unnecessary, the extension can evolve without undoing a large custom transport layer.

## Consequences

### Positive

- Very small bridge-specific code surface in this repo
- Fastest path to a working Zed integration
- Aligns with documented Atlassian local-client setup
- Keeps authentication behavior close to the vendor-supported path

### Negative

- Requires Node.js to be available to Zed
- Adds a runtime dependency on the `mcp-remote` package
- Pinning a tested bridge version improves reproducibility but requires explicit upgrade decisions
- Startup and failures now depend on both the local bridge and the remote Atlassian service

### Operational rules

For the first implementation pass:

- The default endpoint is `https://mcp.atlassian.com/v1/mcp/authv2`.
- The default bridge package is `mcp-remote@0.1.37`.
- The extension should not hardcode credentials or attempt to store secrets in-repo.
- Users may override the command, arguments, or endpoint through Zed settings when needed.

## Alternatives considered

## 1. Direct remote MCP connection from Zed, with no local bridge

Deferred.

This may become viable or preferable later, but the current project shape and current scaffold are optimized around Zed launching a local context-server command. A bridge-based default is the more conservative implementation path right now.

## 2. Build and maintain a custom bridge in this repository

Rejected.

That would increase maintenance burden and create transport/auth behavior that could drift from the vendor-supported path.

## 3. Vendor or bundle a dedicated bridge binary in the extension

Rejected for the first pass.

That would improve control and reproducibility, but it would materially increase packaging and release complexity before the integration shape is proven.

## 4. Pin an exact `mcp-remote` version immediately

Accepted after validation.

The extension now pins `mcp-remote@0.1.37` and installs it into the extension's local `node_modules` before launch. This removes the `npx` bootstrap path that was contributing to context-server startup timeouts during first-run OAuth.

## Follow-up questions

- When the project is ready for wider distribution, how should the pinned `mcp-remote` version be updated and validated over time?
- Should the extension eventually support a direct remote-MCP mode in addition to the bridge-based default?
- How should bridge startup failures and re-authentication prompts be surfaced in the Zed UI?
