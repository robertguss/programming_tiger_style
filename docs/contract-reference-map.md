# Contract Reference Map

This is the canonical file-to-purpose index for Contract System v2. Use it when you need to know
what each file does, when to use it, and which workflow step it supports.

## Workflow Steps

The map below references these workflow steps:

1. Plan: define objective, scope, constraints, and risk tier.
2. Test design: define failing and passing coverage before implementation.
3. Implement: execute Red -> Green -> Refactor in small slices.
4. Evidence: record objective, red/green/refactor traces, and impact.
5. Review: complete checklists and risk-tier controls.
6. CI gate: validate commit sequence and evidence structure.

## Core Contracts (`contracts/core/*`)

| File | What it is | When to use | Required for tier | Workflow step |
| --- | --- | --- | --- | --- |
| `contracts/core/AI_AGENT_CORE_CONTRACT.md` | Baseline agent behavior and required inputs/outputs | Any AI-assisted change (Tier 1-3 planning artifacts required; Tier 0 optional unless stricter local policy) | 0-3 | 1, 3, 4 |
| `contracts/core/TDD_ENFORCEMENT_CONTRACT.md` | Strict TDD cycle rules and commit taxonomy | Any executable code change | 0-3 | 2, 3, 6 |
| `contracts/core/RISK_TIER_POLICY.md` | Tier definitions and required controls by risk | During task planning | 0-3 | 1, 5 |
| `contracts/core/EVIDENCE_REQUIREMENTS.md` | Required evidence packet sections and quality bar | Before PR creation | 0-3 | 4, 6 |
| `contracts/core/ARCHITECTURE_CONTRACT.md` | Structural design and modularity rules | Design-heavy changes and refactors | 1-3 (recommended for 0) | 1, 3 |
| `contracts/core/SECURITY_CONTRACT.md` | Security baseline, testing, and incident readiness | Any change touching trust boundaries/data | 1-3 (required for 2/3) | 1, 3, 5 |
| `contracts/core/PERFORMANCE_CONTRACT.md` | Performance budgets and regression controls | Performance-sensitive changes | 1-3 (required for 2/3) | 2, 3, 4 |
| `contracts/core/DEPENDENCY_POLICY.md` | Dependency admission and maintenance policy | Adding or changing dependencies | 1-3 | 1, 5 |
| `contracts/core/REVIEW_CONTRACT.md` | Reviewer responsibilities and merge conditions | Code review and approval | 0-3 | 5 |
| `contracts/core/INTERACTION_CONTRACT_FOR_CODEX.md` | Codex collaboration protocol and depth modes | When using Codex agent workflows | 0-3 | 1, 3, 4 |

## Language Contracts (`contracts/languages/*`)

| File | What it is | When to use | Required for tier | Workflow step |
| --- | --- | --- | --- | --- |
| `contracts/languages/RUST_CODING_CONTRACT.md` | Rust-specific coding and TDD rules | Rust code or tests are in scope | 0-3 for Rust scope | 2, 3, 5 |
| `contracts/languages/PYTHON_CODING_CONTRACT.md` | Python-specific coding and TDD rules | Python code or tests are in scope | 0-3 for Python scope | 2, 3, 5 |
| `contracts/languages/TYPESCRIPT_CODING_CONTRACT.md` | TypeScript/JavaScript coding and TDD rules | TS/JS code or tests are in scope | 0-3 for TS/JS scope | 2, 3, 5 |

## Templates (`templates/*`)

| File | What it is | When to use | Required for tier | Workflow step |
| --- | --- | --- | --- | --- |
| `templates/TASK_PACKET_TEMPLATE.md` | Structured implementation request | Before coding | 1-3 (optional for 0) | 1 |
| `templates/TEST_PLAN_TEMPLATE.md` | Planned red/green/refactor and boundary coverage | Before coding | 1-3 (optional for 0) | 2 |
| `templates/EVIDENCE_PACKET_TEMPLATE.md` | Standard evidence format for completed work | Before PR merge | 0-3 | 4 |
| `templates/ADR_TEMPLATE.md` | Architecture decision record format | Design decisions with tradeoffs | 1-3 (optional for 0) | 1 |
| `templates/SESSION_HANDOFF_TEMPLATE.md` | Structured handoff between sessions/agents | Long-running or interrupted work | 0-3 (recommended) | 4, 5 |

## Checklists (`checklists/*`)

| File | What it is | When to use | Required for tier | Workflow step |
| --- | --- | --- | --- | --- |
| `checklists/PR_CONTRACT_CHECKLIST.md` | PR-level TDD, risk, security, performance, and review checklist | Before merge | 0-3 | 5 |
| `checklists/ADVERSARIAL_REVIEW_CHECKLIST.md` | Failure-oriented review checklist | Tier 2/Tier 3 or high uncertainty | Required for 2/3, optional otherwise | 5 |

## Validation Scripts (`scripts/*`)

| File | What it is | When to use | Required for tier | Workflow step |
| --- | --- | --- | --- | --- |
| `scripts/validate_tdd_cycle.sh` | Validates commit subject prefixes and Red -> Green -> Refactor ordering | Before PR, in CI | 0-3 | 6 |
| `scripts/validate_evidence_packet.sh` | Validates required evidence headings and unresolved placeholders | Before PR, in CI | 0-3 | 6 |

## GitHub Gate Files (`.github/*`)

| File | What it is | When to use | Required for tier | Workflow step |
| --- | --- | --- | --- | --- |
| `.github/workflows/contract-gates.yml` | Pull request/push gate that runs validators | If using GitHub Actions | 0-3 (recommended baseline) | 6 |
| `.github/pull_request_template.md` | PR body structure aligned with evidence requirements | PR authoring in GitHub | 0-3 | 4, 6 |

## Precedence Rule

When multiple files apply to the same change, use the stricter rule. In practice this usually means:

1. Core contracts apply to all work.
2. Matching language contracts apply in addition.
3. Risk-tier controls can raise testing/review requirements.
4. CI validators enforce required evidence and TDD sequence.
5. Repository policy may require stricter Tier-0 planning artifacts than baseline policy.
