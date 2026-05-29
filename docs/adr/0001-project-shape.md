# ADR 0001: Project shape

- Status: Accepted
- Date: 2026-05-28

## Context

This repository exists to make Atlassian Rovo MCP capabilities available inside Zed.

Several project shapes were possible:

1. Build and maintain a custom MCP server that talks to Atlassian APIs directly.
2. Publish a lightweight documentation/configuration repo and ask users to wire everything together themselves.
3. Ship a Zed-focused integration that stays thin and relies on Atlassian's hosted Rovo MCP server for the actual product-facing behavior.

The current repo is empty, and the surrounding constraints favor a small integration surface:

- Zed already supports MCP-based integrations.
- Atlassian already operates the remote Rovo MCP server.
- Authentication, authorization, and product permissions should remain owned by Atlassian.
- This project should avoid duplicating vendor functionality unless there is a strong product reason.
- Fast time-to-value matters more than building a large bespoke integration layer.

## Decision

This project will be a **thin Zed extension** that connects Zed to Atlassian's hosted Rovo MCP server.

The extension will:

- present the integration in a Zed-native form
- own Zed-specific setup and user experience
- launch or configure a **local bridge process** for MCP connectivity
- treat Atlassian's hosted Rovo MCP server as the source of truth for available tools and product behavior

The project will **not**:

- implement a replacement MCP server for Jira, Confluence, or Compass
- recreate Atlassian auth, permission, or data-access logic
- grow into a generic multi-provider MCP framework

For the first implementation pass, the default assumption is a **thin wrapper around a local bridge such as `mcp-remote`**, because that keeps the repo focused on Zed integration instead of transport reimplementation.

## Rationale

This shape gives the project a clear job: make Atlassian Rovo usable in Zed with the smallest possible amount of custom code.

Why this is the right default:

- **Keeps scope tight.** The project focuses on Zed UX, packaging, and setup rather than rebuilding a server Atlassian already provides.
- **Preserves security boundaries.** Authentication and permissions continue to be enforced by Atlassian.
- **Improves maintainability.** A thin integration is easier to understand, test, and update as vendor behavior changes.
- **Speeds up delivery.** The shortest path to user value is making the existing Rovo MCP server easy to use in Zed.
- **Fits the repo name.** The repository is Zed-specific, not a general Atlassian MCP implementation.

## Consequences

### Positive

- Smaller codebase and lower maintenance burden
- Fewer chances to drift from Atlassian's supported auth and permission model
- Faster initial release path
- Clear ownership boundary between Zed integration concerns and Atlassian platform behavior

### Negative

- The project depends on Atlassian's remote MCP availability and behavior
- The project likely depends on a local bridge/runtime in the user's environment
- Some user experience details may be constrained by Zed and Atlassian's existing MCP surfaces

### Follow-on decisions

This ADR intentionally leaves some implementation details for later ADRs or issues:

- the exact extension layout and build system
- the exact bridge command and packaging strategy
- how health checks and troubleshooting are surfaced in Zed
- whether a direct remote-MCP connection path should be added later

## Alternatives considered

## 1. Build a custom Atlassian MCP server in this repo

Rejected.

This would greatly increase scope and operational burden while duplicating a hosted product that Atlassian already owns.

## 2. Make this repo documentation-only

Rejected.

That would reduce build complexity, but it would not create a clear, installable Zed experience and would leave too much integration work to end users.

## 3. Ship only a raw custom-server config snippet

Rejected as the primary shape.

This may still be useful for advanced users, but it is too barebones to serve as the project's main product shape.
