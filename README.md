# programming_tiger_style

Standards-first repository for high-rigor coding with humans and AI agents.

This project translates TigerBeetle style and NASA/JPL Power of Ten ideas into an executable
Contract System v2 for AI-assisted software development.

## Documentation

Start here:

- [Quickstart](./docs/getting-started/quickstart.md)
- [Tiger Style CLI](./docs/tooling/cli.md)
- [Using with Coding Agents](./docs/guides/using-with-coding-agents.md)
- [Adopting in a New Project](./docs/getting-started/adopting-in-a-new-project.md)
- [Contract Reference Map](./docs/reference/contract-reference-map.md)

Full docs hub:

- [Documentation Index](./docs/README.md)
- [Changelog](./CHANGELOG.md)

Docs are organized under:

- `docs/getting-started/`
- `docs/guides/`
- `docs/tooling/`
- `docs/reference/`
- `docs/decisions/`

Render docs as a local site with mdBook:

```bash
just book-build
just book-serve
```

## Source Foundations

1. `resources/TIGER_STYLE.md`
2. `resources/P10.pdf`

## Contract System v2

Contract System v2 has five layers:

1. Core contracts (cross-language): `contracts/core/`
2. Language activation manifest: `contracts/ACTIVE_LANGUAGE_CONTRACTS.md`
3. Language contracts:
- `contracts/languages/RUST_CODING_CONTRACT.md`
- `contracts/languages/TYPESCRIPT_CODING_CONTRACT.md`
- `contracts/languages/PYTHON_CODING_CONTRACT.md`
4. Delivery templates: `templates/`
5. Validation and governance:
- `checklists/`
- `scripts/`
- `.github/workflows/contract-gates.yml`

In conflicts, apply the stricter rule.

## Mandatory Development Model

All code changes must follow strict TDD:

1. Red: write/modify a failing test.
2. Green: implement the minimum code to pass.
3. Refactor: improve structure without changing behavior.

No Red -> Green -> Refactor evidence means no merge.

## Key Files

### Core Contracts

- `contracts/core/AI_AGENT_CORE_CONTRACT.md`
- `contracts/core/TDD_ENFORCEMENT_CONTRACT.md`
- `contracts/core/RISK_TIER_POLICY.md`
- `contracts/core/EVIDENCE_REQUIREMENTS.md`
- `contracts/core/ARCHITECTURE_CONTRACT.md`
- `contracts/core/SECURITY_CONTRACT.md`
- `contracts/core/PERFORMANCE_CONTRACT.md`
- `contracts/core/DEPENDENCY_POLICY.md`
- `contracts/core/REVIEW_CONTRACT.md`
- `contracts/core/INTERACTION_CONTRACT_FOR_CODEX.md`
- `contracts/ACTIVE_LANGUAGE_CONTRACTS.md`

### Templates

- `templates/TASK_PACKET_TEMPLATE.md`
- `templates/TEST_PLAN_TEMPLATE.md`
- `templates/EVIDENCE_PACKET_TEMPLATE.md`
- `templates/ADR_TEMPLATE.md`
- `templates/SESSION_HANDOFF_TEMPLATE.md`

### Checklists

- `checklists/PR_CONTRACT_CHECKLIST.md`
- `checklists/ADVERSARIAL_REVIEW_CHECKLIST.md`

### Automation

- `scripts/validate_tdd_cycle.sh`
- `scripts/validate_evidence_packet.sh`
- `.github/pull_request_template.md`
- `.github/workflows/contract-gates.yml`

### References

- `resources/RESOURCES.md`

## Operating Workflow

1. For Tier 1-3, create a task packet (Tier 0 optional unless repo policy is stricter).
2. Assign risk tier.
3. For Tier 1-3, create a test plan (Tier 0 optional unless repo policy is stricter).
4. Execute Red -> Green -> Refactor in small cycles.
5. Produce evidence packet.
6. Complete PR and adversarial checklists.
7. Pass CI contract gates.

## For Codex Users

Before implementation, provide:

1. Objective and non-goals.
2. Constraints and forbidden approaches.
3. Interfaces/files in scope.
4. Acceptance criteria.
5. Risk tier.

For Tier 1-3 work, provide full task packet and test plan. For Tier 0 work, lightweight planning
notes are acceptable unless repository policy requires full artifacts.

Then require explicit evidence in PR output.

## Repository Status

This repository stores contract definitions, templates, and enforcement automation.

It also ships a Rust CLI (`tooling/tiger-style-cli`) that automates downstream installation and
configuration:

```bash
tiger-style bootstrap --target /absolute/path/to/your-repo
```

For local CLI development:

```bash
just build
just run -- --help
just install-cargo
```
