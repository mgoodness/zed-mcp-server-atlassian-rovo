# ADR 0005: Release validation

- Status: Accepted
- Date: 2026-05-28

## Context

The earlier ADRs established that V1 is a thin Zed extension that:

- connects Zed to Atlassian's hosted Rovo MCP server
- uses `mcp-remote` as the default bridge strategy
- treats OAuth 2.1 as the primary supported authentication path
- focuses primary support on Jira and Confluence workflows

The repo now needs an explicit release-validation bar so it is clear what must be true before calling a build or release candidate acceptable.

Because this project is intentionally thin, release validation should focus on the user-visible integration path rather than deep product-specific logic. The most important risks are:

- the extension fails to load in Zed
- the context server fails to start
- the bridge command is wrong or missing prerequisites
- the auth flow fails in the common OAuth path
- Jira and Confluence smoke tests do not work even though the extension appears installed

## Decision

V1 release validation is a **smoke-test checklist**, not a broad automated product-certification suite.

A candidate is acceptable for V1 only if all of the following pass in a realistic local-development setup:

## Required validation checks

### 1. Build and packaging sanity

- `cargo check` passes
- the extension repository contains a valid `extension.toml`
- the extension can be installed in Zed as a dev extension

### 2. Extension installation sanity

- Zed recognizes the extension without manifest errors
- the extension exposes the `atlassian-rovo` context server
- the setup/configuration surface appears as expected when relevant

### 3. Bridge startup sanity

In an environment with Node.js available:

- Zed can launch the configured managed-Node + local `mcp-remote@0.1.37` bridge path
- startup failures are diagnosable through Zed logs or visible error output
- the recommended Atlassian endpoint is used by default unless explicitly overridden

### 4. Primary auth-path sanity

Using the normal supported auth path:

- the OAuth 2.1 browser flow can be initiated
- the user can complete sign-in successfully
- the resulting session is usable from Zed

### 5. V1 product smoke tests

After successful auth, at minimum the following should work for a tester with appropriate Atlassian permissions:

- one **Jira-oriented** request, such as `List my Jira issues`
- one **Confluence-oriented** request, such as `Search Confluence for onboarding docs`

These do not need to prove every upstream tool, only that the extension's main integration path is functioning for the primary supported workflows.

## Failure policy

A release candidate should be considered blocked if any of the following are true:

- the extension does not install in Zed
- the context server cannot be started with the documented default setup
- the OAuth 2.1 path is broken for the normal user flow
- both Jira and Confluence smoke tests cannot be completed successfully in a valid test environment

Best-effort support areas, such as Compass or advanced API-token-based auth, are **not** release blockers for V1 unless a regression breaks explicitly documented behavior.

## Rationale

This validation strategy fits the project shape.

- **Focuses on the real user path.** The key value is successful install, auth, and basic use in Zed.
- **Avoids fake precision.** The extension does not own upstream Atlassian tool behavior, so it should not pretend to certify every product surface.
- **Keeps the bar practical.** A lightweight extension needs a lightweight but meaningful release gate.
- **Aligns with V1 scope.** Jira and Confluence remain the core workflows that must work.

## Consequences

### Positive

- Clear release gate for early development
- Shared expectation for what “working” means
- Lower chance of shipping an extension that installs but fails at first use
- Validation effort stays proportional to the repo's thin-integration role

### Negative

- Much of the validation is manual or semi-manual
- Test quality depends on access to a real Atlassian environment
- Some upstream regressions may only be detected during smoke testing rather than automated CI

## Testing

Use the following operational documents when applying this ADR:

- `docs/release-checklist.md` — release/smoke-test checklist for candidates
- `docs/testing/local-dev.md` — step-by-step local development testing guide

## Operational guidance

For each release candidate, record:

- Zed version used for testing
- operating system used for testing
- Node.js availability/version
- whether OAuth 2.1 succeeded
- the Jira smoke-test prompt and outcome
- the Confluence smoke-test prompt and outcome
- any known best-effort areas not validated

## Alternatives considered

## 1. Require broad automated end-to-end coverage before any release

Rejected.

This would be disproportionate to the current project shape and would block progress before the core extension workflow is proven.

## 2. Rely only on `cargo check`

Rejected.

That would validate the Rust scaffold but not the user-visible install, auth, and bridge flow that actually defines success for this repo.

## 3. Treat every Rovo-exposed Atlassian product as a release blocker

Rejected.

That would expand the validation promise far beyond V1 scope.

## Follow-up questions

- Should the repo add a human-readable release checklist doc derived from this ADR?
- Which portions of this smoke-test flow can later be automated?
- At what point should Compass move from best-effort validation to release-blocking validation?
