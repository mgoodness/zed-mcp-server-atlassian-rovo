# ADR 0004: Authentication strategy

- Status: Accepted
- Date: 2026-05-28

## Context

The previous ADRs established that this repository is a thin Zed extension which:

- connects Zed to Atlassian's hosted Rovo MCP server
- uses `mcp-remote` as the default bridge strategy
- focuses V1 support on Jira and Confluence workflows

A remaining question was which authentication modes the project should support in practice.

Atlassian's Rovo MCP server supports OAuth 2.1 as the default path and may also support API-token-based authentication depending on Atlassian admin settings. This repository, however, is not the authentication authority; it is only responsible for launching the bridge in a Zed-friendly way and documenting the supported path.

The key constraints are:

- Atlassian owns the actual auth and permission model.
- The extension should not implement custom credential handling logic unless there is a clear product need.
- The first release should optimize for the most broadly supported, lowest-surprise setup path.
- Secrets must not be committed to the repository or baked into the extension.

## Decision

The V1 authentication strategy is:

- **Primary supported auth mode:** Atlassian-managed OAuth 2.1 through the hosted Rovo MCP flow
- **Secondary / advanced auth mode:** API-token-based auth only when supported by Atlassian and only through bridge/server configuration that remains outside repo-stored secrets
- **Non-goal for V1:** custom credential UX, token storage, or secret management implemented by this extension

In practical terms, V1 assumes the normal user flow is:

1. Zed launches the local `mcp-remote` bridge.
2. The bridge connects to Atlassian's hosted MCP endpoint.
3. Atlassian drives the OAuth 2.1 browser flow.
4. Atlassian permissions determine what the user can access and do.

If API-token auth is available in a user's Atlassian environment, it may be treated as an advanced compatibility path, but it is not the primary documented or validated path for V1.

## Rationale

This is the most appropriate strategy for a thin extension.

- **Matches vendor defaults.** Atlassian documents OAuth 2.1 as the standard path.
- **Keeps responsibility boundaries clean.** Atlassian remains responsible for auth and authorization behavior.
- **Avoids premature secret-handling code.** The extension does not need to invent storage, rotation, or secret UX in V1.
- **Reduces support complexity.** A single primary auth story is easier to document and troubleshoot.
- **Still leaves room for advanced setups.** Users whose environments allow API-token auth are not blocked from configuring it outside the extension's core promise.

## Consequences

### Positive

- Clear, simple first-run story for most users
- No repo-managed secret storage
- Lower implementation and support burden
- Better alignment with Atlassian's recommended flow

### Negative

- Some advanced or service-style setups may want API-token-first workflows that the extension does not treat as first-class in V1
- Troubleshooting remains partly dependent on Atlassian's browser/OAuth flow behavior
- Future secret-management improvements may still be desirable if Zed or user expectations evolve

### Operational guidance

For V1 docs and support:

- document OAuth 2.1 as the expected setup path
- avoid asking users to paste secrets into repository files
- treat API-token-based auth as advanced/best-effort guidance, not the default path
- keep auth troubleshooting focused on browser flow, Atlassian permissions, and bridge startup behavior

## Alternatives considered

## 1. Treat OAuth and API token auth as equally first-class in V1

Rejected.

This would broaden the support and documentation surface too early, while the extension itself does not add enough auth-specific value to justify that complexity.

## 2. Build custom secret management into the extension immediately

Rejected.

That would pull the project into credential UX and storage concerns before the core extension workflow is validated.

## 3. Support only API-token auth

Rejected.

This would diverge from Atlassian's default/recommended flow and create unnecessary friction for the main use case.

## Follow-up questions

- Should a future release provide more explicit advanced-auth documentation for API-token-based setups?
- If Zed adds stronger secret-management primitives for context servers, should this extension adopt them?
- What auth failures should be called out explicitly in a release-validation checklist?
