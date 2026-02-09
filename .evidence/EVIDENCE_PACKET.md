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

- Failing test(s):
  - Slice 1 (`install` baseline): `install_writes_required_tree`
- Command(s) used:
  - `cargo test --manifest-path tooling/tiger-style-cli/Cargo.toml --test cli -- install_writes_required_tree`
- Failure summary:
  - Test failed with `install command is not implemented yet` and exit code `2`.
- Why this failure is expected:
  - The command surface was scaffolded first; install behavior had not been implemented yet.

## Green

- Minimal implementation summary:
  - Slice 1 (`install`): embedded install assets, implemented file copy/create/overwrite paths,
    dry-run behavior, unix script permission setting, and conflict detection with diff previews.
- Command(s) used:
  - `cargo test --manifest-path tooling/tiger-style-cli/Cargo.toml --test cli -- install_`
- Passing summary:
  - Install integration tests passed (`4 passed; 0 failed`).

## Refactor

- Structural improvements made: To be filled.
- Why behavior is unchanged: To be filled.
- Command(s) used to confirm tests remain green: To be filled.

## Invariants

- Invariant assertions added/updated: To be filled.
- Boundary conditions covered: To be filled.

## Security Impact

- Threats considered: command execution safety, file overwrite safety, path traversal safety.
- Mitigations: explicit command invocation, conflict-default fail, bounded traversal exclusions.
- Residual risk: user may still apply `--force`; documented behavior required.

## Performance Impact

- Baseline measurement: N/A before implementation.
- Post-change measurement: To be filled from test/command runs.
- Delta explanation: To be filled.

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
# Commands run for validation
```
