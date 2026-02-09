# Evidence Packet

## Objective

- Problem solved: Manual adoption/install flow is cumbersome and error-prone.
- Intended outcome: Provide `tiger-style` CLI for repeatable install/configure/doctor/bootstrap workflow.

## Risk Tier

- Tier: `2`
- Rationale: New executable plus CI/release automation with cross-platform behavior.

## Scope

- Files/components changed: CLI crate, root Rust workspace, docs, workflows, and this evidence packet.
- Explicitly excluded scope: replacing existing validator scripts with Rust implementation.

## Red

- Failing test(s): Three feature-slice RED tests are listed below.
  - Slice 1 (`install` baseline): `install_writes_required_tree`
  - Slice 2 (`configure` autodetect): `configure_autodetect_sets_manifest_statuses`
  - Slice 3 (`doctor` + `bootstrap`): `bootstrap_installs_configures_and_passes_doctor`
- Command(s): Targeted RED commands are listed below.
  - `cargo test --manifest-path tooling/tiger-style-cli/Cargo.toml --test cli -- install_writes_required_tree`
  - `cargo test --manifest-path tooling/tiger-style-cli/Cargo.toml --test cli -- configure_autodetect_sets_manifest_statuses`
  - `cargo test --manifest-path tooling/tiger-style-cli/Cargo.toml --test cli -- bootstrap_installs_configures_and_passes_doctor`
- Failure summary: Each RED stage failed for expected unimplemented behavior.
  - Test failed with `install command is not implemented yet` and exit code `2`.
  - Test failed with `configure command is not implemented yet` and exit code `2`.
  - Test failed with `doctor checks failed` because doctor returned unimplemented status.
- Expected failure rationale: The implementation intentionally followed scaffold-first strict TDD.
  - The command surface was scaffolded first; install behavior had not been implemented yet.
  - Configure behavior and language autodetect had not been implemented yet.
  - Doctor validation behavior was intentionally deferred until this feature slice.

## Green

- Minimal implementation summary:
  - Slice 1 (`install`): embedded install assets, implemented file copy/create/overwrite paths,
    dry-run behavior, unix script permission setting, and conflict detection with diff previews.
  - Slice 2 (`configure`): implemented manifest mode handling (`autodetect`, `all-active`,
    `all-inactive`), language detection with directory exclusions, canonical manifest rendering, and
    AGENTS template creation/conflict handling.
  - Slice 3 (`doctor` + `bootstrap`): implemented structural checks, manifest parsing, active
    language tool checks, validator checks with strict/non-strict bash handling, machine-readable
    JSON output, and bootstrap orchestration with dry-run doctor skip.
- Command(s): Targeted GREEN commands are listed below.
  - `cargo test --manifest-path tooling/tiger-style-cli/Cargo.toml --test cli -- install_`
  - `cargo test --manifest-path tooling/tiger-style-cli/Cargo.toml --test cli -- configure_`
  - `cargo test --manifest-path tooling/tiger-style-cli/Cargo.toml --test cli -- doctor_`
  - `cargo test --manifest-path tooling/tiger-style-cli/Cargo.toml --test cli -- bootstrap_`
- Passing summary: All targeted GREEN test groups passed after minimal implementation.
  - Install integration tests passed (`4 passed; 0 failed`).
  - Configure integration tests passed (`3 passed; 0 failed`).
  - Doctor and bootstrap integration tests passed (`4 passed; 0 failed` across targeted runs).

## Refactor

- Structural improvements made:
  - Slice 1 (`install`): extracted shared action logging helper to remove duplicated output logic.
  - Slice 2 (`configure`): extracted shared status-label helper to remove duplicated mode rendering
    logic.
  - Slice 3 (`doctor`): replaced repeated per-tool checks with table-driven iteration.
- Why behavior is unchanged: Refactors only reduced duplication and preserved behavior contracts.
  - Refactor touched only log-path structure and kept identical create/overwrite semantics.
  - Refactor touched only formatting logic in dry-run output and did not alter manifest decisions.
  - Refactor touched only control-flow structure for equivalent tool-check behavior.
- Confirmation commands: Full-suite command used after each refactor stage is listed below.
  - `cargo test --manifest-path tooling/tiger-style-cli/Cargo.toml`

## Invariants

- Invariant assertions added/updated:
  - Existing files are never overwritten by default when contents differ (conflict exit code `3`).
  - Manifest output always includes exact rust/python/typescript status lines.
  - `doctor` enforces tool presence only for active languages.
- Boundary conditions covered:
  - Empty repository autodetect results in all languages inactive.
  - Existing custom `AGENTS.md` conflicts without `--force`.
  - `bootstrap --dry-run` writes nothing and skips `doctor`.
  - `doctor --strict` fails when bash is unavailable.

## Security Impact

- Threats considered: command execution safety, file overwrite safety, path traversal safety.
- Mitigations: explicit command invocation, conflict-default fail, bounded traversal exclusions.
- Residual risk: user may still apply `--force`; documented behavior required.

## Performance Impact

- Baseline measurement:
  - Manual install required multiple copy/chmod commands plus manual manifest edits.
- Post-change measurement:
  - One command (`tiger-style bootstrap`) now performs install+configure+doctor in a single run.
- Delta explanation:
  - CLI consolidates repeated manual operations, reducing operator steps and setup variance.

## Assumptions

1. GitHub releases are the canonical distribution channel for CLI binaries.
2. Existing manual documentation remains fallback during initial CLI adoption.

## Open Questions

1. None.

## Rollback Plan

- Trigger conditions: incorrect install layout, unsafe overwrite behavior, invalid doctor pass/fail outcomes.
- Rollback steps: revert CLI and workflow commits; retain existing docs/script-based installation flow.

## Validation Commands

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings -D clippy::unwrap_used -D clippy::expect_used -D clippy::undocumented_unsafe_blocks
cargo test --workspace --all-features
scripts/validate_tdd_cycle.sh --base HEAD~10
scripts/validate_evidence_packet.sh --file .evidence/EVIDENCE_PACKET.md
```
