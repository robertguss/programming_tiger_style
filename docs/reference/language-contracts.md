# Language Contracts

Contract System v2 ships three language contracts. They are first-class and equal: choose based on
actual code scope, not team preference.

## Available Contracts

- `contracts/ACTIVE_LANGUAGE_CONTRACTS.md`
- `contracts/languages/RUST_CODING_CONTRACT.md`
- `contracts/languages/PYTHON_CODING_CONTRACT.md`
- `contracts/languages/TYPESCRIPT_CODING_CONTRACT.md`

All three enforce strict TDD and include language-specific rules for error handling, type safety,
performance boundaries, and warning-free CI.

## Recursion and Test-Code Clarifications

- Recursion is allowed only with bounded depth/size behavior and documented failure modes.
- Depth and malformed-structure boundary tests are required when recursion is used.
- Test-only convenience patterns are allowed in a limited scope when failures are intentional and
  local to setup/assertion.
- Test allowances cannot be used to mask production-path typing/error-handling defects.

## Precedence and Composition

Rules compose in this order:

1. Core contracts always apply.
2. `contracts/ACTIVE_LANGUAGE_CONTRACTS.md` decides which language contracts are active.
3. Active language contracts apply for files in scope.
4. If rules conflict, apply the stricter rule and document rationale in evidence.

For polyglot repositories, apply each language contract to its language surface while keeping one
shared risk tier and shared evidence packet.

## Activation Source Of Truth

Use `contracts/ACTIVE_LANGUAGE_CONTRACTS.md` with exact status lines:

- `- rust: active|inactive`
- `- python: active|inactive`
- `- typescript: active|inactive`

If this file is absent, workflow fallback is file-extension autodetection. The manifest is preferred
because it provides deterministic intent for CI and reviewers.

## Selection Logic

### Single-language repository

Mark one language `active` in the manifest and enforce that contract for production and test files.

### Polyglot repository

Mark all relevant languages `active` and apply multiple language contracts concurrently. Example: a
Rust service plus TypeScript frontend must satisfy core contracts plus Rust and TypeScript language
rules.

### Scripting/tooling inside a main language repo

If scripts are in Python or TypeScript, they still fall under the respective language contract unless
explicitly exempted by documented exception. Keep manifest statuses aligned with real enforcement
scope.

## Same Feature, Different Language Constraints (Concrete Example)

Feature slice: "Reject invalid pagination limit and return typed error."

Rust path:

- Add failing test for invalid limit.
- Return typed error (`Result<_, DomainError>`), avoid `panic!/unwrap/expect` in production path.
- Use fixed-width numeric types at boundaries.

Python path:

- Add failing test for invalid limit.
- Raise explicit typed/domain exception, avoid broad `except:` handlers.
- Validate untrusted input before business logic.

TypeScript path:

- Add failing test for invalid limit.
- Validate external input as `unknown`, narrow to typed structure.
- Avoid `any`; return explicit typed error or discriminated union.

All three paths must preserve strict Red -> Green -> Refactor evidence and full-suite refactor gate.

## Cross-Language Consistency Checklist

- Shared feature semantics remain equivalent across languages.
- Error behavior is explicit and test-covered in each language.
- Performance and security constraints are asserted in each implementation.
- Evidence packet references all language-specific test commands used.
- Manifest status matches active code ownership and CI expectations.
- Recursion and test-code allowance decisions are documented with boundary test evidence.

## Related References

- [Risk Tiers and Controls](./risk-tiers-and-controls.md)
- [Using with Coding Agents](../guides/using-with-coding-agents.md)
- [Glossary](./glossary.md)
