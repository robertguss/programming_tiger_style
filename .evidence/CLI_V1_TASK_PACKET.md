# CLI v1 Task Packet

## Objective

- Build a cross-platform `tiger-style` CLI that installs, configures, and validates Contract System v2 in downstream repositories.

## Non-Goals

- Replacing current shell validators (`scripts/validate_tdd_cycle.sh`, `scripts/validate_evidence_packet.sh`).
- Supporting package-manager installation inside target repos.
- Implementing advanced migration/diff/upgrade subcommands beyond v1 scope.

## Context

- v1.1 adoption currently depends on manual copy commands from `docs/adopting-in-a-new-project.md`.
- This change must dogfood Tiger Style (Tier 2) with strict Red -> Green -> Refactor commit sequencing.

## Scope

- In scope:
  - `tooling/tiger-style-cli/**`
  - root `Cargo.toml` workspace setup
  - `.github/workflows/cli-ci.yml`
  - `.github/workflows/cli-release.yml`
  - `docs/cli.md`
  - `README.md`
  - `docs/adopting-in-a-new-project.md`
  - `.evidence/*` artifacts for this implementation
- Out of scope:
  - redesigning existing core contracts/templates beyond wiring into CLI installation set.

## Constraints

- Language: Rust + Clap.
- Binary name: `tiger-style`.
- Commands: `install`, `configure`, `doctor`, `bootstrap`.
- Conflict default: fail-safe with diff preview; `--force` required to overwrite.
- `bootstrap` runs non-strict doctor by default.
- Release tags: `tiger-style-cli-v*`.

## Risk Tier

- Tier: `2`
- Rationale: introduces new executable installer path, release automation, and cross-platform behavior affecting repository setup correctness.

## Interfaces And Dependencies

- CLI interface:
  - `tiger-style install --target <path> [--force] [--dry-run]`
  - `tiger-style configure --target <path> [--manifest-mode autodetect|all-inactive|all-active] [--force] [--dry-run]`
  - `tiger-style doctor --target <path> [--strict] [--format text|json]`
  - `tiger-style bootstrap --target <path> [--force] [--dry-run]`
- New Rust dependencies: `clap`, `anyhow`, `thiserror`, `include_dir`, `walkdir`, `similar`, `serde`, `serde_json`.

## Acceptance Criteria

1. CLI installs full v1.1 downstream layout and preserves idempotence.
2. CLI configure writes canonical manifest status lines and can autodetect active languages.
3. CLI doctor validates structure/tools/validator scripts and exits with defined status codes.
4. Conflicts are reported with diff preview and fail exit code `3` without `--force`.
5. Test suite covers install conflict modes, configure autodetect matrix, doctor missing-tool behavior, bootstrap orchestration, and dry-run behavior.

## Test Expectations

- Required unit tests: manifest detection, conflict rendering, doctor manifest parsing.
- Required integration tests: install flow, configure modes, bootstrap flow, exit-code behavior.
- Required regression tests: conflicting file handling and AGENTS.md conflict handling.
- Required performance/security checks: bounded filesystem traversal and explicit command execution without shell interpolation.

## Security Requirements

- No shell interpolation with untrusted input.
- No secret material embedded in emitted files/logs.
- Clear error surfaces for missing tooling and malformed manifest.

## Performance Requirements

- CLI should complete typical install/configure/doctor runs quickly on small-to-medium repos.
- Repository traversal for autodetect must skip heavy known directories.

## Observability Requirements

- Human-readable action logs for text output mode.
- Machine-readable JSON summary for doctor mode.

## Rollback Conditions

- Roll back if CLI writes incorrect file layout, corrupts existing files without `--force`, or reports false-success health checks.
- Rollback mechanism: revert CLI/workflow/doc commits and keep manual adoption docs as primary path.

## Definition Of Done

- CLI commands behave per contract and tests pass.
- Tier 2 artifacts and evidence are complete.
- Root and CLI validations pass, including `scripts/validate_tdd_cycle.sh` and `scripts/validate_evidence_packet.sh`.
