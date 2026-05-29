# Context

## Project

- Name: `zed-mcp-server-atlassian-rovo`
- Status: scaffolding stage; no implementation has been committed yet

## Purpose

`zed-mcp-server-atlassian-rovo` is intended to help Zed users connect the editor's agent features to Atlassian's Rovo MCP server so they can work with Jira, Confluence, and related Atlassian products from inside Zed.

The likely goal of this repository is to package the Atlassian Rovo integration in a Zed-friendly form, reducing setup friction and making Atlassian tools available through the Model Context Protocol instead of requiring users to switch tools constantly.

Success for this project means a Zed user can configure the integration, authenticate successfully, and use Atlassian-backed MCP tools from within Zed with behavior that matches their existing Atlassian permissions.

For V1, the primary supported workflows are Jira and Confluence. Other Rovo-exposed Atlassian products are best-effort unless explicitly broadened by a later ADR.

## Core concepts

- **Zed** — The editor and agent host that will surface the MCP integration to the end user.
  - **Why it matters:** This repo exists to make Atlassian capabilities usable from inside Zed.
  - **Related concepts:** MCP client, extension/configuration, user settings

- **MCP (Model Context Protocol)** — The protocol Zed uses to communicate with context servers and tools.
  - **Why it matters:** It is the integration boundary between Zed and any external tool/data source.
  - **Related concepts:** MCP server, transport, tools, prompts

- **Atlassian Rovo MCP server** — Atlassian's hosted MCP gateway for Jira, Confluence, Compass, and related Atlassian Cloud data.
  - **Why it matters:** This is the remote system the integration is expected to connect to.
  - **Related concepts:** Atlassian Cloud, authentication, remote MCP endpoint

- **Bridge/proxy process** — A local process or configuration layer that lets a local MCP-capable client talk to a remote Atlassian MCP endpoint.
  - **Why it matters:** Zed typically connects to MCP servers either as local commands or remote endpoints; a proxy/bridge is often the glue between Zed and Atlassian's hosted service.
  - **Related concepts:** `mcp-remote`, local command, transport

- **Authentication** — The mechanism used to authorize access to Atlassian data. In V1, the primary supported path is Atlassian-managed OAuth 2.1, with API-token-based auth treated as an advanced compatibility path where Atlassian permits it.
  - **Why it matters:** The integration is only useful if users can authenticate reliably and safely.
  - **Related concepts:** browser flow, tokens, scopes, user permissions

- **Atlassian Cloud data** — Jira issues, Confluence pages, Compass entities, and other supported Atlassian resources.
  - **Why it matters:** These are the user-visible resources the integration exists to expose. In V1, Jira and Confluence are the primary supported workflows.
  - **Related concepts:** search, summarization, create/update actions, permissions

- **User permissions** — The existing Atlassian permissions of the authenticated user.
  - **Why it matters:** The integration should respect Atlassian authorization rules rather than inventing its own.
  - **Related concepts:** least privilege, security, auditability

## System boundaries

This section describes the intended responsibilities of this repo based on the project name and external Atlassian/Zed documentation. The current repository does not yet contain implementation code.

### In scope

- Shipping a Zed-specific integration for Atlassian Rovo MCP
- Capturing setup instructions, configuration conventions, and local development workflow
- Launching or describing the local bridge/proxy process needed by Zed
- Guiding users through authentication and initial connection checks
- Providing a stable Zed-facing integration surface for Atlassian MCP tools

### Out of scope

- Re-implementing Atlassian's remote MCP server itself
- Owning Jira, Confluence, or Compass business logic
- Bypassing Atlassian authentication, permission, or audit controls
- Acting as a generic MCP framework for every provider
- Storing long-lived secrets in the repository

## Architecture notes

The codebase is currently empty, but the project shape has been chosen in `docs/adr/0001-project-shape.md`.

### Chosen structure

The intended architecture has three layers:

1. **Zed extension layer**
   - Defines how the integration appears to Zed
   - Owns the Zed-specific installation and configuration experience

2. **Connection/bridge layer**
   - Starts or references the process that connects a local MCP client to Atlassian's hosted MCP service
   - Uses a thin `mcp-remote`-based bridge approach rather than re-implementing Atlassian server behavior

3. **Operational guidance layer**
   - Documents prerequisites, authentication steps, troubleshooting, and supported workflows

### Expected data flow

1. User enables or installs the integration in Zed.
2. Zed launches a local command or connects using configured MCP settings.
3. The integration bridges requests to the Atlassian Rovo MCP server.
4. Atlassian authenticates the user and authorizes actions using the user's existing permissions.
5. Tool results flow back through MCP into Zed's agent experience.

### Important runtime boundaries

- **Zed runtime boundary** — local editor environment
- **Bridge/proxy boundary** — local process translating or relaying MCP traffic
- **Atlassian boundary** — remote hosted MCP service and Atlassian Cloud products
- **Authentication boundary** — browser/token flow and any locally cached credentials

## Source of truth

Record where important decisions or facts live.

- Product/domain language: this file
- Architecture decisions: `docs/adr/`
- Issue tracking: `.scratch/<feature>/`
- Repo workflow conventions: `AGENTS.md`
- Vendor/platform behavior: Zed MCP docs and Atlassian Rovo MCP documentation

## Constraints

- The integration must fit Zed's MCP model and configuration capabilities.
- The integration depends on Atlassian's supported authentication flows and endpoint behavior.
- User actions must respect existing Atlassian permissions and security controls.
- Secrets and tokens should not be committed to the repo.
- The repo should prefer thin integration code/config over duplicating vendor functionality.
- Because the project is at scaffolding stage, architectural decisions should remain reversible until implementation pressure makes them concrete.

## Open questions

- What is the exact installation experience the project wants to optimize first: local development, internal team rollout, or public extension distribution?
- When the project is ready for wider distribution, should the bridge dependency be pinned to a tested `mcp-remote` version instead of using `@latest`?
- Should a direct remote-MCP connection path be supported later, in addition to the bridge-based default?


- How should connection health, authentication failures, and permission-related errors be surfaced to the user?
- Does this project intend to ship code, documentation, or both?

## Change log

- 2026-05-28: Replaced the placeholder context template with an initial project context inferred from the repository name, Zed MCP documentation, and Atlassian Rovo MCP documentation. This should be refined once implementation code or a concrete product plan exists.
- 2026-05-28: Recorded the initial project-shape decision in `docs/adr/0001-project-shape.md`: this repo will be a thin Zed extension that connects to Atlassian's hosted Rovo MCP server via a local bridge rather than implementing a custom Atlassian MCP server.
- 2026-05-28: Scaffolded the first-pass Zed extension layout with `extension.toml`, a Rust extension crate, and an `atlassian-rovo` context-server definition that launches `mcp-remote` against Atlassian's recommended `/v1/mcp/authv2` endpoint.
- 2026-05-28: Updated the bridge startup path after live validation: the extension now installs a pinned `mcp-remote@0.1.37` package into local `node_modules` and launches it through Zed's managed Node runtime instead of bootstrapping through `npx`.
- 2026-05-28: Recorded the V1 support scope in `docs/adr/0003-v1-scope.md`: Jira and Confluence are the primary supported workflows for the first release, while other Rovo-exposed Atlassian products remain best-effort.
- 2026-05-28: Recorded the authentication strategy in `docs/adr/0004-auth-strategy.md`: V1 treats Atlassian-managed OAuth 2.1 as the primary supported auth path, while API-token-based auth remains an advanced compatibility option rather than a first-class extension feature.
- 2026-05-28: Recorded the V1 release-validation bar in `docs/adr/0005-release-validation.md`: releases are gated by smoke tests covering extension install, bridge startup, OAuth setup, and one Jira plus one Confluence workflow.
