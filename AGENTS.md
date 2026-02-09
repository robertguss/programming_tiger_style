# Repository Guidelines

## Project Structure & Module Organization
- Rust workspace root: `Cargo.toml`, `Cargo.lock`, and shared `justfile`.
- CLI implementation lives in `tooling/tiger-style-cli/src/` (`main.rs` entrypoint, command modules such as `install.rs`, `doctor.rs`, `configure.rs`).
- Rust integration tests live in `tooling/tiger-style-cli/tests/`.
- Contract system sources are in `contracts/core/` and `contracts/languages/`; templates are in `templates/`.
- Validation scripts and fixtures are in `scripts/`, `scripts/tests/`, and `scripts/test-fixtures/`.
- Project docs are under `docs/` (mdBook sources) and rendered output in `docs/book/`.

## Build, Test, and Development Commands
- `just build`: build CLI in debug mode.
- `just run -- --help`: run CLI locally with arbitrary args.
- `just test`: run all Rust tests (`cargo test --workspace --all-features`).
- `just check`: CI-equivalent Rust gate (`fmt-check + clippy + test`).
- `just fmt` / `just fmt-check`: format/check Rust code.
- `bash scripts/validate_tdd_cycle.sh --base origin/main`: enforce commit-prefix and Red->Green->Refactor ordering.
- `bash scripts/validate_evidence_packet.sh --pr-body /tmp/pr_body.md`: validate required PR evidence headings/fields.
- `just book-build` / `just book-serve`: build/serve docs with mdBook.

## Coding Style & Naming Conventions
- Rust formatting is mandatory via `rustfmt`; target line length is 100 columns.
- Use `snake_case` for Rust files, modules, functions, and variables.
- Keep production Rust warning-free; `clippy` denies `unwrap`, `expect`, and undocumented `unsafe`.
- Markdown/prose formatting follows `.prettierrc.json` (`printWidth: 100`, `proseWrap: always`).
- Shell scripts should follow existing style (`#!/usr/bin/env bash` + `set -euo pipefail`).

## Testing Guidelines
- Put Rust behavior tests close to the CLI crate (`tooling/tiger-style-cli/tests/cli.rs` and module tests).
- Name shell validator tests as `scripts/tests/test_*.sh`; add fixtures under `scripts/test-fixtures/<area>/`.
- No percentage coverage threshold is defined; merges require passing tests plus both validator scripts.

## Commit & Pull Request Guidelines
- Use commit subjects with enforced prefixes: `RED`, `GREEN`, `REFACTOR`, `DOCS`, `CHORE`, `BUILD`, `TEST`.
- For code changes, keep commit flow in strict TDD order: Red -> Green -> Refactor.
- PRs should use `.github/pull_request_template.md` headings and include risk tier, validation commands, rollback plan, and exception status.
- Complete `checklists/PR_CONTRACT_CHECKLIST.md` for meaningful changes; include `checklists/ADVERSARIAL_REVIEW_CHECKLIST.md` for Tier 2/3 work.
