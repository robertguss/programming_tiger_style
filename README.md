# programming_tiger_style

Standards-first repository for writing code with humans and AI agents.

This repo captures a safety-focused engineering philosophy and turns it into enforceable coding
contracts for Rust, TypeScript, and Python, with mandatory TDD.

## Why this exists

These documents are built on two source references:

- `TIGER_STYLE.md` (TigerBeetle style philosophy)
- `P10.pdf` (NASA/JPL Power of Ten)

The contracts in this repo adapt those ideas into practical rules for modern language stacks and
agent-assisted development.

## Core principles

- Priority order is fixed: Safety -> Performance -> Developer Experience.
- Keep control flow simple and bounded.
- Handle errors explicitly.
- Use assertions/invariants deliberately.
- Prefer explicit limits (timeouts, retries, queue depth, batch size, memory growth).
- Enforce strict TDD with Red -> Green -> Refactor for every code change.

## Contracts

- Rust: `RUST_CODING_CONTRACT.md`
- TypeScript: `TYPESCRIPT_CODING_CONTRACT.md`
- Python: `PYTHON_CODING_CONTRACT.md`

Each contract includes:

- Non-negotiable rules
- Language-specific style/API constraints
- AI agent workflow requirements
- Required CI gates
- PR compliance checklist
- Exception process

## Mandatory TDD cycle

All contracts require strict TDD in this order:

1. Red: write/modify a test that fails for the expected reason.
2. Green: implement the minimum change to make the test pass.
3. Refactor: improve structure with behavior unchanged and tests still green.

Bug fixes must start with a failing regression test.

## How to use this repo

1. Pick the language contract that matches the code you are writing.
2. Start with the contract's AI/human workflow and define invariants, bounds, and error model.
3. Execute work in strict Red -> Green -> Refactor cycles.
4. Run the contract's CI gate commands.
5. Complete the PR checklist and include TDD evidence.

## Intended use with AI agents

- Load the relevant contract before generating or editing code.
- Treat contract rules as hard constraints, not suggestions.
- Reject outputs that skip Red/Green or bypass required checks.

## Repository status

This repository currently stores standards and contracts only.
