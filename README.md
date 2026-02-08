# programming_tiger_style

Standards-first repository for high-rigor coding with humans and AI agents.

This project translates TigerBeetle style and NASA/JPL Power of Ten ideas into an executable
Contract System v2 for AI-assisted software development.

## Source Foundations

1. `TIGER_STYLE.md`
2. `P10.pdf`

## Contract System v2

Contract System v2 has four layers:

1. Core contracts (cross-language): `contracts/core/`
2. Language contracts:
- `RUST_CODING_CONTRACT.md`
- `TYPESCRIPT_CODING_CONTRACT.md`
- `PYTHON_CODING_CONTRACT.md`
3. Delivery templates: `templates/`
4. Validation and governance:
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

- `RESOURCES.md`

## Operating Workflow

1. Create a task packet.
2. Assign risk tier.
3. Create a test plan.
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

Then require explicit evidence in PR output.

## Repository Status

This repository stores contract definitions, templates, and enforcement automation.
