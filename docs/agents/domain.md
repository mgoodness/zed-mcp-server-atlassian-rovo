# Domain docs

This repository uses a single-context documentation layout.

## Layout

- Root context: `CONTEXT.md`
- ADR directory: `docs/adr/`
- User documentation: `README.md`
- Operational docs: `docs/release-checklist.md`, `docs/testing/local-dev.md`

## Consumer rules for agent skills

Skills that need domain context should:

1. Read `CONTEXT.md` at the repo root first, if present.
2. Then read relevant ADRs under `docs/adr/`.
3. For operational tasks (testing, releasing, smoke-testing), also consult `docs/release-checklist.md` and `docs/testing/local-dev.md`.
4. Prefer existing domain language and decisions from those docs over inventing new terminology.

## Notes

This repo is not configured as a multi-context workspace. There is no root `CONTEXT-MAP.md`.
