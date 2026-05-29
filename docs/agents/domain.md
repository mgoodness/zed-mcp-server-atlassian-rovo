# Domain docs

This repository uses a single-context documentation layout.

## Layout

- Root context: `CONTEXT.md`
- ADR directory: `docs/adr/`

## Consumer rules for agent skills

Skills that need domain context should:

1. Read `CONTEXT.md` at the repo root first, if present.
2. Then read relevant ADRs under `docs/adr/`.
3. Prefer existing domain language and decisions from those docs over inventing new terminology.
4. If those files do not exist yet, proceed carefully and note that domain context has not been established.

## Notes

This repo is not configured as a multi-context workspace. There is no root `CONTEXT-MAP.md`.
