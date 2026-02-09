# Rust Coding Contract

## Purpose

This contract defines how humans and AI agents write Rust in this repository. Priority order is
fixed:

1. Safety
2. Performance
3. Developer Experience

If a tradeoff is required, choose the higher priority item.

## Contract Integration

- This language contract supplements the core contracts in `contracts/core/`.
- Core contracts are mandatory for all work, including Rust.
- If rules conflict, apply the stricter rule and document rationale in evidence.

## Scope

This contract applies to:

- All production Rust code.
- All tests, benchmarks, examples, and tooling code unless explicitly exempted.
- All AI-generated or AI-edited code.

## TDD Mandate (Red -> Green -> Refactor)

- TDD is mandatory for any code change.
- Production code must not be added or changed before a failing test exists.
- The cycle is strict and ordered:

1. Red: add or modify a test that fails for the expected reason.
2. Green: implement the minimum code required to make the test pass.
3. Refactor: improve code structure while keeping behavior unchanged and tests green.

- For bug fixes, a regression test must fail before the fix and pass after the fix.
- Skipping Red or combining Red/Green into a single unverified step is non-compliant.

## Non-Negotiable Rules

### 1) Keep control flow simple and analyzable

- Recursion is allowed only when input is finite/acyclic by construction or explicit depth/size
  limits are enforced.
- Document recursion rationale and failure mode in design notes/evidence.
- Add boundary tests for recursion depth and malformed structures.
- When depth risk is non-trivial, document why an iterative alternative is not preferred.
- Every loop must have a clear upper bound, or a clear reason it is intentionally non-terminating.
- Prefer structured branching over dense compound boolean expressions.
- Split complex conditions into explicit branches so positive and negative cases are both clear.

### 2) Put explicit limits on work and resources

- Bound queue sizes, retries, batch sizes, and per-request work.
- Bound memory growth. Pre-size collections when limits are known.
- For hard real-time or latency-critical paths, avoid post-init allocation and pre-allocate
  capacity.
- Fail fast when limits are exceeded, and return a typed error.
- Timeouts are required for external I/O and cross-process boundaries.

### 3) Treat assertions as design checks

- Assert preconditions, postconditions, and critical invariants.
- Pair assertions across boundaries (for example, before write and after read).
- Prefer multiple simple assertions over one compound assertion.
- Use `debug_assert!` for hot-path checks that are safe to remove in release.

### 4) Handle all errors explicitly

- Never ignore `Result` or `Option` values from fallible operations.
- `unwrap()`, `expect()`, and `panic!()` are forbidden in production paths.
- Allowed exceptions are limited to unreachable states proven by invariant and process-entry fatal
  configuration validation with an actionable message.
- For library code, return typed errors; do not terminate the process.

### 5) Keep data scope tight and state single-sourced

- Declare variables at the smallest possible scope.
- Avoid duplicated state and avoid aliases that can diverge.
- Compute values close to use sites to reduce stale checks.
- Prefer immutable bindings; mutate only where required and local.

### 6) Keep functions small and cohesive

- Hard limit: 70 lines per function (excluding signature and attributes).
- Each function should have one clear responsibility.
- Keep branch orchestration in parent functions; move leaf computation to helpers.

### 7) Use explicit types and units

- Use fixed-width integers (`u32`, `u64`, etc.) for stored or serialized data.
- Avoid leaking `usize` across API, persistence, or network boundaries.
- Encode units in names (`timeout_ms`, `size_bytes`, `offset_bytes`).
- Use checked/saturating/overflow-aware arithmetic deliberately.

### 8) Unsafe is opt-in and audited

- Safe Rust is the default.
- Every `unsafe` block requires a `// SAFETY:` comment explaining the invariant.
- Isolate unsafe code in narrow modules with focused tests.
- If unsafe is used, add tests that stress invalid and boundary inputs.

### 9) Be explicit at call sites

- Do not rely on implicit defaults for behavior that affects safety or correctness.
- Prefer named config structs/builders over ambiguous positional arguments.
- For public APIs, mark important return values with `#[must_use]`.

### 10) Zero-warning policy

- The codebase must build with zero compiler and lint warnings.
- New warnings block merge.

### 11) Mandatory TDD cycle (Red -> Green -> Refactor)

- All code changes must follow Red -> Green -> Refactor in strict order.
- Each cycle should be small and focused on one behavior at a time.
- Refactors that change behavior require a new Red step first.

## Test Code Allowances

- Test-only code may use limited convenience patterns (`unwrap`, `expect`, `panic!`) when failure is
  intentional and local to setup/assertion logic.
- Test allowances must not be used to hide production-path error handling defects.
- Keep test helpers separated from production modules so exception scope is explicit.

## Rust-Specific Style and API Rules

- Formatting is mandatory with `rustfmt`.
- Line length limit is 100 columns.
- Prefer `snake_case` for functions, variables, modules, and files.
- Prefer domain nouns/verbs over abbreviations.
- For booleans that affect behavior, prefer enums over boolean flags.
- For async code, never block executors (`std::thread::sleep`, blocking I/O) and use bounded
  channels/concurrency.

## AI Agent Workflow

Before writing code, the agent must produce a short design sketch in the PR description or issue
comment:

1. Invariants and failure modes.
2. Bounds (time, memory, retries, queue depth, batch size).
3. Error model (typed errors and propagation strategy).
4. TDD plan naming the first failing test and expected failure mode.
5. Test plan (valid, invalid, boundary, and regression cases).

During implementation, the agent must:

1. Red: add/modify a failing test and run it to confirm expected failure.
2. Green: implement the minimal code change required to make the failing test pass.
3. Refactor: improve structure while keeping tests green and behavior unchanged.
4. Add or update assertions for invariants.
5. Keep each commit logically atomic and explain why in commit message or PR notes.

Before merge, the agent must:

1. Run all required checks locally.
2. Confirm zero warnings and passing tests after refactor.
3. Include evidence of Red -> Green -> Refactor in PR notes or commit sequence.
4. Confirm contract compliance checklist is complete.

## Required CI Gates

Minimum required commands:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- \
  -D warnings \
  -D clippy::unwrap_used \
  -D clippy::expect_used \
  -D clippy::undocumented_unsafe_blocks
cargo test --workspace --all-features
```

Required for crates/modules containing `unsafe` or critical memory/concurrency logic:

```bash
cargo test --workspace --all-features --release
```

Optional but recommended where available:

```bash
cargo +nightly miri test
```

TDD evidence gate:

- PR must include a Red -> Green -> Refactor trace (commit order or PR log) with test command
  evidence.

## Pull Request Contract Checklist

Each PR must answer yes/no to the following:

1. Was a failing test written first and observed failing for the expected reason (Red)?
2. Was the minimal implementation added to make that test pass (Green)?
3. Was refactor performed only after Green, with tests still passing (Refactor)?
4. Are all loops bounded or intentionally non-terminating with justification?
5. Are limits explicit (timeouts, retries, queue sizes, batch sizes, memory growth)?
6. Are preconditions/postconditions/invariants asserted where critical?
7. Are all fallible operations handled without `unwrap()`/`expect()` in production paths?
8. Are function sizes and scopes kept within contract limits?
9. Are integer types and units explicit at boundaries?
10. Is every `unsafe` block documented with `// SAFETY:` and covered by targeted tests?
11. Did CI pass with zero warnings?
12. Does the PR explain why the design is safe and performant, not only what changed?

## Exception Process

If a rule must be broken, add a short exception note in the PR:

1. Rule being waived.
2. Why waiver is required.
3. Risk introduced.
4. Compensating controls (tests, assertions, monitors, rollback plan).
5. Expiration condition for removing the waiver.

No exception is accepted without explicit reviewer approval.
