# Issue tracker

This repository uses a local-markdown issue tracker.

## Source of truth

Issues live as markdown files under `.scratch/<feature>/` in this repository.

Each feature or workstream should get its own directory under `.scratch/`, and issues for that area should live there.

## Consumer rules for agent skills

Skills that create or update issues should:

1. Treat `.scratch/<feature>/` markdown files as the issue tracker.
2. Prefer creating one markdown file per issue.
3. Use clear, stable filenames derived from the issue title.
4. Preserve existing user-written content when updating issue files.
5. Link related docs, PRDs, and follow-up work using repo-relative paths where possible.

## Suggested issue shape

A typical local issue file can include:

- Title
- Status
- Summary
- Context
- Acceptance criteria
- Notes / open questions

## Notes

This repo does not currently use GitHub Issues or GitLab Issues as the canonical tracker for agent workflows.
