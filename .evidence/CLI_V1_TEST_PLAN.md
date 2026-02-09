# CLI v1 Test Plan

## Objective

- Verify `tiger-style` safely installs, configures, and validates Contract System v2 in downstream repositories.

## Risk Tier

- Tier: `2`
- Rationale: new installer executable and release automation with cross-platform behavior.

## Invariants

1. Existing files are not overwritten unless `--force` is provided.
2. Manifest output always contains exact canonical status lines for rust/python/typescript.
3. `doctor` exit code semantics remain stable (`0` success, `2` validation failures).

## Red Tests (Must Fail First)

| Test ID | Type | Scenario | Expected Failure |
| ------- | ---- | -------- | ---------------- |
| RED-1 | integration | Fresh install command expected to copy baseline files | Command fails before implementation due to missing command logic |
| RED-2 | integration | Configure autodetect should set active language status lines | Missing configure logic or incorrect manifest output |
| RED-3 | integration | Doctor should fail when active language tool is unavailable | Doctor not implemented or does not enforce required tool checks |

## Green Validation Tests

| Test ID | Type | Scenario | Expected Pass Condition |
| ------- | ---- | -------- | ----------------------- |
| GREEN-1 | integration | `install` on empty temp repo | Required files/dirs exist after command |
| GREEN-2 | integration | `install` conflict without `--force` | Exit code `3` and diff preview emitted |
| GREEN-3 | integration | `configure --manifest-mode autodetect` matrix | Correct active/inactive statuses for each fixture repo |
| GREEN-4 | integration | `doctor --format json` | JSON includes pass/fail plus missing tools/checks |
| GREEN-5 | integration | `bootstrap` orchestration | Install+configure+doctor run sequentially and succeed |

## Refactor Safety Net

- CLI integration tests under `tooling/tiger-style-cli/tests`.
- Full crate suite (`cargo test --manifest-path tooling/tiger-style-cli/Cargo.toml`).
- Workspace rust checks after root workspace is added.

## Boundary And Negative Tests

| Test ID | Boundary/Abuse Case | Expected Behavior |
| ------- | ------------------- | ----------------- |
| NEG-1 | Existing edited file during install | Conflict failure, no overwrite without `--force` |
| NEG-2 | Existing custom AGENTS.md during configure | Conflict failure unless `--force` |
| NEG-3 | Empty repo autodetect | All languages set inactive |
| NEG-4 | Missing bash during doctor non-strict | Warning only (not failure) |
| NEG-5 | Missing bash during doctor strict | Validation failure exit code `2` |

## Performance Tests

- Validate directory traversal excludes heavy directories and completes quickly in test fixtures.
- Budgets asserted: command-level completion in local fixture runs without pathological slowdown.

## Security Tests

- Auth/authz cases: N/A for local CLI.
- Malformed input cases: invalid target path and malformed manifest parsing.
- Data exposure cases: ensure no secret-bearing env/file dumps in errors.

## Commands

```bash
# Red
cargo test --manifest-path tooling/tiger-style-cli/Cargo.toml --test cli -- install_writes_required_tree

# Green
cargo test --manifest-path tooling/tiger-style-cli/Cargo.toml --test cli -- install_writes_required_tree

# Full suite
cargo test --manifest-path tooling/tiger-style-cli/Cargo.toml
```

## Exit Criteria

- All required tests pass.
- No flaky/unresolved failures.
- Contract validators pass with evidence packet updated.
