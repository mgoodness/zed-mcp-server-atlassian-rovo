# Release checklist

Use this checklist for any release candidate or meaningful milestone build of `zed-mcp-server-atlassian-rovo`.

This checklist operationalizes `docs/adr/0005-release-validation.md`.

## Candidate details

- Date:
- Version / tag:
- Tester:
- Zed version:
- Operating system:
- Node.js version:
- Atlassian environment used for testing:

## 1. Build and packaging sanity

- [ ] `cargo check` passes
- [ ] `extension.toml` exists and is valid for the intended extension shape
- [ ] Required project files are present:
  - [ ] `extension.toml`
  - [ ] `Cargo.toml`
  - [ ] `src/lib.rs`
  - [ ] `README.md`

### Notes

-

## 2. Dev extension install in Zed

- [ ] Open Zed
- [ ] Run `zed: extensions`
- [ ] Use `Install Dev Extension`
- [ ] Select this repository
- [ ] Confirm Zed installs the extension without manifest/load errors
- [ ] Confirm the extension appears as `Atlassian Rovo MCP Server`

### Notes

-

## 3. Context server presence and setup UX

- [ ] Open the Agent Panel
- [ ] Confirm the `atlassian-rovo` context server is exposed
- [ ] Confirm setup/configuration UI appears as expected when relevant
- [ ] Confirm default settings are visible or inferable:
  - [ ] endpoint defaults to `https://mcp.atlassian.com/v1/mcp/authv2`
  - [ ] bridge package defaults to `mcp-remote@0.1.37`

### Notes

-

## 4. Bridge startup sanity

- [ ] Node.js is available to Zed in the test environment
- [ ] Zed can launch the default bridge command
- [ ] No unexpected startup failure occurs before auth begins
- [ ] If startup fails, the failure is diagnosable via Zed logs or visible error output

### Default bridge startup shape

```text
Zed-managed Node runtime + extension-local `mcp-remote@0.1.37` install targeting `https://mcp.atlassian.com/v1/mcp/authv2 --debug`
```

### Notes

-

## 5. Primary auth-path sanity

- [ ] OAuth 2.1 browser flow can be initiated
- [ ] Sign-in completes successfully
- [ ] The resulting authenticated session is usable from Zed
- [ ] No repo-stored secrets or ad hoc credential files were required for the primary flow

### Notes

-

## 6. V1 product smoke tests

### Jira

Prompt used:

```text
List my Jira issues
```

- [ ] Jira smoke test succeeds
- [ ] Result is consistent with the tester's Atlassian permissions

### Jira notes

-

### Confluence

Prompt used:

```text
Search Confluence for onboarding docs
```

- [ ] Confluence smoke test succeeds
- [ ] Result is consistent with the tester's Atlassian permissions

### Confluence notes

-

## 7. Known non-blocking / best-effort areas

Record anything checked outside the V1 blocking path.

### Compass

- [ ] Not tested
- [ ] Tested successfully
- [ ] Tested with issues

Notes:

-

### Advanced API-token auth

- [ ] Not tested
- [ ] Tested successfully
- [ ] Tested with issues

Notes:

-

### Other Rovo-exposed products

- [ ] Not tested
- [ ] Tested successfully
- [ ] Tested with issues

Notes:

-

## 8. Release decision

### Blocking issues

-

### Non-blocking issues

-

### Decision

- [ ] Accept candidate
- [ ] Reject candidate
- [ ] Accept with noted follow-ups

## 9. Follow-up actions

-
