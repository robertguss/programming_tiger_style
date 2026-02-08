# Resources

Curated references to support this contract system.

## Foundations

1. `resources/TIGER_STYLE.md`
2. `resources/P10.pdf` (NASA/JPL Power of Ten)

## TDD And Testing

1. _Test-Driven Development: By Example_ (Kent Beck)
2. _Growing Object-Oriented Software, Guided by Tests_ (Freeman & Pryce)
3. _xUnit Test Patterns_ (Meszaros)
4. Property-based testing references (language-specific ecosystem docs)
5. Mutation testing references (language-specific ecosystem docs)

## Reliability And Operations

1. Google SRE principles (error budgets, monitoring, incident response)
2. Postmortem best practices (blameless, action-oriented)

## Security

1. OWASP ASVS
2. OWASP Top 10
3. Language-specific secure coding guides

## Architecture And Design

1. Ports and adapters (hexagonal architecture)
2. Functional core / imperative shell
3. Domain-driven design tactical patterns

## Language-Specific Tooling Suggestions

### Rust

- `cargo clippy`, `cargo fmt`, `cargo test`
- `cargo-audit`, `cargo-deny`, `proptest`, `cargo-fuzz`, `loom`

### TypeScript

- `tsc --noEmit`, `eslint`, `prettier`, test runner of choice
- `fast-check` (property-based testing), `stryker` (mutation testing)

### Python

- `ruff`, `mypy`/`pyright`, `pytest`
- `hypothesis` (property-based testing), `mutmut` (mutation testing), `pip-audit`
