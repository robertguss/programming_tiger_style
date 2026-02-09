# TypeScript Coding Contract

## Purpose

This contract defines how humans and AI agents write TypeScript in this repository. Priority order
is fixed:

1. Safety
2. Performance
3. Developer Experience

If a tradeoff is required, choose the higher priority item.

## Contract Integration

- This language contract supplements the core contracts in `contracts/core/`.
- Core contracts are mandatory for all work, including TypeScript/JavaScript.
- If rules conflict, apply the stricter rule and document rationale in evidence.

## Scope

This contract applies to:

- All production TypeScript and JavaScript code.
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
- Split complex branching into positive and negative cases that are both handled explicitly.

### 2) Put explicit limits on work and resources

- Bound retries, queue depth, batch size, and per-request work.
- Bound payload sizes and in-memory collection growth.
- Require explicit timeouts for I/O (HTTP, database, queues, files).
- Use cancellation (`AbortSignal`) for long-running async operations.

### 3) Treat assertions and invariants as design checks

- Assert preconditions, postconditions, and critical invariants.
- Pair assertions across boundaries (for example, before persistence and after retrieval).
- Prefer multiple simple assertions over one dense assertion.
- Keep assertions side-effect free.

### 4) Handle all errors explicitly

- Every rejected promise must be handled or deliberately propagated.
- No floating promises in production code.
- `catch` blocks must preserve context and return typed failures where possible.
- `throw` raw strings is forbidden.

### 5) Keep state tight and single-sourced

- Keep variable scope as small as possible.
- Avoid duplicated state that can drift out of sync.
- Compute values close to use sites to reduce stale checks.
- Prefer immutable data (`const`, `readonly`) by default.

### 6) Keep functions small and cohesive

- Hard limit: 70 lines per function (excluding signature and decorators).
- Each function should have one clear responsibility.
- Keep control flow orchestration in parent functions and move pure computation to helpers.

### 7) Use explicit types at boundaries

- `strict` mode is required in `tsconfig`.
- `any` is forbidden in production code. Use `unknown` at external boundaries and narrow it.
- Validate untrusted input at runtime before use (HTTP, env, file, queue, DB).
- Prefer discriminated unions over boolean flag combinations for state transitions.

### 8) Avoid dangerous dynamic behavior

- `eval`, `new Function`, and implicit code execution from strings are forbidden.
- Prototype mutation in application code is forbidden.
- Global mutable singleton state requires explicit justification.

### 9) Be explicit at call sites

- Do not rely on implicit defaults for correctness- or safety-critical behavior.
- Prefer options objects with named fields over ambiguous positional argument lists.
- Important return values must be consumed and should be modeled to prevent accidental ignoring.

### 10) Zero-warning policy

- The codebase must build, typecheck, and lint with zero warnings.
- New warnings block merge.

### 11) Mandatory TDD cycle (Red -> Green -> Refactor)

- All code changes must follow Red -> Green -> Refactor in strict order.
- Each cycle should be small and focused on one behavior at a time.
- Refactors that change behavior require a new Red step first.

## Test Code Allowances

- Test-only code may use limited convenience patterns (targeted `any`, loose doubles, intentional
  rejection helpers) when failures are local to setup/assertion behavior.
- Test allowances must not be used to hide production-path type/error handling defects.
- Keep test helpers separate from production modules so review scope is explicit.

## TypeScript-Specific Style and API Rules

- Formatting is mandatory with a single formatter configuration.
- Line length limit is 100 columns.
- Prefer `snake_case` for file names and `camelCase` for variables/functions.
- Public interfaces and exported functions require explicit return types.
- Async code must not block the event loop with CPU-heavy sync work on hot paths.
- For CPU-intensive tasks, offload to worker threads or separate services.

## AI Agent Workflow

Before writing code, the agent must produce a short design sketch in the PR description or issue
comment:

1. Invariants and failure modes.
2. Bounds (timeouts, retries, queue depth, payload limits, batch size).
3. Error model (typed errors and propagation strategy).
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
npx tsc --noEmit
npx eslint . --max-warnings 0
npx prettier --check .
npm test
```

Required for services with performance-critical paths:

```bash
npm run test:perf
```

Optional but recommended where available:

```bash
npm run test:fuzz
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
7. Are all promises and errors handled without floating rejections?
8. Are function sizes and scopes kept within contract limits?
9. Are boundary types explicit, with runtime validation for untrusted input?
10. Are dangerous dynamic behaviors (`eval`, runtime code strings) absent?
11. Did typecheck/lint/tests pass with zero warnings?
12. Does the PR explain why the design is safe and performant, not only what changed?

## Exception Process

If a rule must be broken, add a short exception note in the PR:

1. Rule being waived.
2. Why waiver is required.
3. Risk introduced.
4. Compensating controls (tests, checks, monitors, rollback plan).
5. Expiration condition for removing the waiver.

No exception is accepted without explicit reviewer approval.
