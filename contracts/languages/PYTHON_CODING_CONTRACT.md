# Python Coding Contract

## Purpose

This contract defines how humans and AI agents write Python in this repository. Priority order is
fixed:

1. Safety
2. Performance
3. Developer Experience

If a tradeoff is required, choose the higher priority item.

## Contract Integration

- This language contract supplements the core contracts in `contracts/core/`.
- Core contracts are mandatory for all work, including Python.
- If rules conflict, apply the stricter rule and document rationale in evidence.

## Scope

This contract applies to:

- All production Python code.
- All tests, scripts, and tooling code unless explicitly exempted.
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
- Prefer explicit branches over dense compound conditions.
- Split complex branching so positive and negative cases are both handled explicitly.

### 2) Put explicit limits on work and resources

- Bound retries, queue depth, batch size, and per-request work.
- Bound payload sizes and memory growth of in-process collections.
- Require explicit timeouts for external I/O (HTTP, DB, queues, file/network operations).
- Use bounded pools/executors for concurrency.

### 3) Treat assertions and invariants as design checks

- Assert preconditions, postconditions, and critical invariants.
- Pair assertions across boundaries (for example, before write and after read).
- Prefer multiple simple assertions over one dense assertion.
- Use `assert` for programmer invariants; use explicit exceptions for runtime validation.

### 4) Handle all errors explicitly

- No bare `except:` blocks.
- Catch specific exception types and preserve context.
- Do not silently swallow exceptions.
- Library code must raise typed/domain exceptions instead of exiting the process.

### 5) Keep state tight and single-sourced

- Keep variable scope as small as possible.
- Avoid duplicated state that can diverge.
- Compute values close to use sites to reduce stale checks.
- Avoid mutable global state unless explicitly justified.

### 6) Keep functions small and cohesive

- Hard limit: 70 lines per function (excluding decorators and docstring).
- Each function should have one clear responsibility.
- Keep branch orchestration in parent functions and move pure computation to helpers.

### 7) Use explicit types and units

- Type hints are required for public functions, methods, and module-level constants.
- Validate untrusted input at boundaries before use.
- Encode units in names (`timeout_s`, `size_bytes`, `latency_ms`).
- Avoid implicit type coercion in critical paths.

### 8) Avoid dangerous dynamic behavior

- `eval` and `exec` are forbidden in production code.
- Dynamic imports that depend on untrusted input are forbidden.
- Mutable default arguments are forbidden.

### 9) Be explicit at call sites

- Do not rely on implicit defaults for correctness- or safety-critical behavior.
- Prefer keyword arguments over ambiguous positional argument lists.
- Important return values must be consumed; do not ignore failures.

### 10) Zero-warning policy

- The codebase must lint, typecheck, and test with zero warnings.
- New warnings block merge.

### 11) Mandatory TDD cycle (Red -> Green -> Refactor)

- All code changes must follow Red -> Green -> Refactor in strict order.
- Each cycle should be small and focused on one behavior at a time.
- Refactors that change behavior require a new Red step first.

## Test Code Allowances

- Test-only code may use limited convenience patterns (broad assertion helpers or intentional
  exception fixtures) when failures are local to setup/assertion behavior.
- Test allowances must not be used to hide production-path error handling defects.
- Keep test helpers separate from production modules so review scope is explicit.

## Python-Specific Style and API Rules

- Formatting is mandatory with a single formatter configuration.
- Line length limit is 100 columns.
- Prefer `snake_case` for files, variables, and functions.
- Use context managers for resource lifetimes (`with` for files, locks, connections).
- Async code must never block the event loop with synchronous I/O or sleep calls.
- For CPU-intensive tasks, use process pools or offload outside the async loop.

## AI Agent Workflow

Before writing code, the agent must produce a short design sketch in the PR description or issue
comment:

1. Invariants and failure modes.
2. Bounds (timeouts, retries, queue depth, payload limits, batch size).
3. Error model (exceptions and propagation strategy).
4. TDD plan naming the first failing test and expected failure mode.
5. Test plan (valid, invalid, boundary, and regression cases).

During implementation, the agent must:

1. Red: add/modify a failing test and run it to confirm expected failure.
2. Green: implement the minimal code change required to make the failing test pass.
3. Refactor: improve structure while keeping tests green and behavior unchanged.
4. Add or update invariants and boundary checks.
5. Keep commits logically atomic and explain why in commit message or PR notes.

Before merge, the agent must:

1. Run all required checks locally.
2. Confirm zero warnings and passing tests after refactor.
3. Include evidence of Red -> Green -> Refactor in PR notes or commit sequence.
4. Confirm contract compliance checklist is complete.

## Required CI Gates

Minimum required commands:

```bash
ruff format --check .
ruff check . --output-format=full
mypy .
pytest -q
```

Required for services with concurrency/performance-critical paths:

```bash
pytest -q -m "not slow" --maxfail=1
```

Optional but recommended where available:

```bash
pyright
pytest -q --hypothesis-show-statistics
```

TDD evidence gate:

- PR must include a Red -> Green -> Refactor trace (commit order or PR log) with test command
  evidence.

## Pull Request Contract Checklist

Each PR must answer yes/no to the following:

1. Was a failing test written first and observed failing for the expected reason (Red)?
2. Was the minimal implementation added to make that test pass (Green)?
3. Was refactor performed only after Green, with tests still passing (Refactor)?
4. Are all loops/retries bounded or intentionally non-terminating with justification?
5. Are limits explicit (timeouts, retries, queue sizes, payload and memory growth)?
6. Are preconditions/postconditions/invariants checked where critical?
7. Are exceptions explicit and specific, with no swallowed failures?
8. Are function sizes and scopes kept within contract limits?
9. Are boundary types and units explicit, with input validation?
10. Are dangerous dynamic behaviors (`eval`, `exec`) absent?
11. Did lint/typecheck/tests pass with zero warnings?
12. Does the PR explain why the design is safe and performant, not only what changed?

## Exception Process

If a rule must be broken, add a short exception note in the PR:

1. Rule being waived.
2. Why waiver is required.
3. Risk introduced.
4. Compensating controls (tests, checks, monitors, rollback plan).
5. Expiration condition for removing the waiver.

No exception is accepted without explicit reviewer approval.
