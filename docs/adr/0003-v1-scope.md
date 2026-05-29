# ADR 0003: V1 scope

- Status: Accepted
- Date: 2026-05-28

## Context

The earlier ADRs established that this repository is a thin Zed extension which:

- integrates Atlassian Rovo into Zed
- relies on Atlassian's hosted MCP server for tool behavior
- uses `mcp-remote` as the default bridge strategy

The remaining product question was what the first release should promise to users.

Because this extension is intentionally thin, it does not implement Jira-, Confluence-, or Compass-specific business logic itself. Even so, the repo still needs a clear V1 scope for:

- documentation and examples
- validation and smoke-test expectations
- support and troubleshooting posture
- deciding what counts as in-scope vs nice-to-have

Atlassian's hosted Rovo MCP server can expose capabilities across multiple Atlassian products, but not all products are equally central to the project's first release.

## Decision

V1 scope is:

- **Primary supported workflows:** Jira and Confluence
- **Secondary / best-effort workflows:** other Atlassian products exposed by Rovo, including Compass
- **Non-goal for V1:** product-specific Zed UX beyond generic MCP connectivity and setup

In practical terms, V1 promises that this extension should make it straightforward for a Zed user to:

- authenticate successfully with Atlassian Rovo
- use Jira-backed MCP workflows from Zed
- use Confluence-backed MCP workflows from Zed
- troubleshoot the bridge/setup path when those primary workflows fail

V1 does **not** promise:

- custom Zed UI tailored to a specific Atlassian product
- product-specific wrappers around Jira, Confluence, or Compass actions
- exhaustive validation coverage for every Atlassian product Rovo may expose
- compatibility guarantees for unsupported or newly added Rovo product surfaces beyond best-effort behavior

## Rationale

This scope keeps the project honest about what it can support early.

- **Matches the most common value path.** Jira and Confluence are the most obvious initial use cases for IDE-centered Atlassian workflows.
- **Fits the thin-extension shape.** The extension's main job is connectivity, setup, and packaging, not per-product feature engineering.
- **Keeps support manageable.** The project can define realistic smoke tests and troubleshooting guidance without claiming universal product coverage.
- **Leaves room for upside.** If Compass or other Rovo-exposed products work through the same bridge, users benefit immediately, but the project is not forced to treat them as fully validated first-class surfaces on day one.

## Consequences

### Positive

- Clear documentation target for V1 examples and validation steps
- Smaller support surface for the first release
- Reduced pressure to add product-specific code prematurely
- Easy path to expand scope later if usage justifies it

### Negative

- Users may expect full Atlassian-product parity because the underlying Rovo server is broader than the extension's stated support posture
- Compass and other products may work, but with weaker guarantees in V1
- Future scope expansion will require explicit docs and support updates

### Operational guidance

For V1 docs and validation:

- prioritize Jira and Confluence examples
- treat successful auth and basic Jira/Confluence task execution as the minimum smoke-test bar
- describe Compass and other product flows as best-effort unless and until a later ADR broadens support

## Alternatives considered

## 1. Jira-only V1

Rejected.

This would keep scope very tight, but it is narrower than necessary for a thin bridge-based extension and would understate the natural value of connecting Atlassian Rovo to Zed.

## 2. Jira + Confluence + Compass all as first-class V1 targets

Rejected for now.

This is attractive, but it expands the documentation, validation, and support promise before the extension has real usage feedback.

## 3. All Rovo-exposed Atlassian products are equally in-scope for V1

Rejected.

That would create an overly broad support commitment without product-specific validation or troubleshooting guidance.

## Follow-up questions

- Should Compass become a first-class supported workflow in V1.1 or V2?
- What exact Jira and Confluence smoke tests should the release checklist require?
- Should future docs include product-specific setup examples, or remain product-agnostic?
