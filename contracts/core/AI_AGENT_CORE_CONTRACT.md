# AI Agent Core Contract

## Purpose

This contract defines baseline rules for AI coding agents (including Codex) across all languages in
this repository.

Priority order is fixed:

1. Safety
2. Performance
3. Developer Experience

## Scope

This contract applies to every code change, regardless of language.

## Contract Hierarchy

When multiple contracts apply, use this precedence:

1. `contracts/core/*.md` (this directory)
2. language contracts (`contracts/languages/RUST_CODING_CONTRACT.md`,
   `contracts/languages/TYPESCRIPT_CODING_CONTRACT.md`,
   `contracts/languages/PYTHON_CODING_CONTRACT.md`)
3. task-specific instructions in the active issue/PR

If two rules conflict, apply the stricter rule and document the decision in the evidence packet.

## Canonical v1.1 Decisions

Tiger Style v1.1 locks these decisions:

1. Policy truth: contract text is authoritative.
2. Enforcement truth: validators and CI gates must enforce active contract requirements.
3. Adoption truth: staged rollout is allowed only for legacy style debt; TDD, evidence, and
   risk-tier controls remain mandatory.

## Precedence Across Policy, Enforcement, and Docs

If contract text, validators/workflows, and documentation diverge, resolve using:

1. Core contract text.
2. Enforcement implementation (`scripts/*`, `.github/workflows/*`).
3. Documentation and examples.

Any divergence is a compliance defect and must be fixed before merge, or covered by an explicit,
timeboxed exception with compensating controls in the evidence packet.

## Operating Modes

AI agents must operate in two explicit modes:

1. Discovery mode:

- Clarify problem, constraints, interfaces, and unknowns.
- For Tier 1-3 work, produce task packet and test plan before code changes.
- For Tier 0 work, lightweight planning notes are acceptable unless repository policy requires full
  artifacts.

2. Execution mode:

- Implement strict Red -> Green -> Refactor cycles.
- Generate evidence for each cycle.

Switch from Discovery to Execution only when confidence is at least 95% that requirements,
boundaries, and acceptance criteria are clear.

## Non-Negotiable Agent Behaviors

1. Dissent duty:

- The agent must push back on unsafe, incoherent, or internally contradictory requirements.

2. Uncertainty disclosure:

- Unknowns, assumptions, and confidence risks must be made explicit.

3. No silent assumptions:

- Every assumption must be logged in the evidence packet.

4. Boundary-first validation:

- Validate untrusted input before business logic.

5. Minimal-diff rule:

- Keep changes tightly scoped to the objective; avoid opportunistic refactors during Green steps.

6. Deterministic test rule:

- Tests must be deterministic unless explicitly marked and justified.

7. Source-of-truth rule:

- For uncertain API/tool behavior, check official docs before implementation.

8. No placeholder-completion rule:

- Do not leave TODO/FIXME placeholders for core logic on merged paths.

## Required Inputs Before Implementation

Tier 1-3 tasks must provide a task packet using `templates/TASK_PACKET_TEMPLATE.md` and include:

1. Objective and non-goals.
2. Constraints and forbidden approaches.
3. Interfaces and files in scope.
4. Acceptance tests.
5. Security/performance requirements.
6. Definition of done and rollback conditions.

Tier 0 tasks may use lightweight planning notes or the full task packet. Repository policy may
choose stricter behavior and require full Tier-0 planning artifacts.

## Required Outputs For Every Change

1. Test plan from `templates/TEST_PLAN_TEMPLATE.md` for Tier 1-3 (Tier 0 optional unless repo
   policy requires it).
2. Evidence packet from `templates/EVIDENCE_PACKET_TEMPLATE.md`.
3. Completed PR checklist (`checklists/PR_CONTRACT_CHECKLIST.md`).
4. Adversarial review pass (`checklists/ADVERSARIAL_REVIEW_CHECKLIST.md`) for Tier 2/Tier 3.

## Definition Of Done

A change is done only when:

1. Red -> Green -> Refactor evidence is present and valid.
2. Required CI gates pass.
3. Risk-tier obligations are satisfied (`contracts/core/RISK_TIER_POLICY.md`).
4. Contract exceptions, if any, are explicitly approved and documented.

## Exceptions

Exceptions require all of the following:

1. Rule being waived.
2. Justification.
3. Risk introduced.
4. Compensating controls.
5. Expiration/removal condition.

No exception is valid without reviewer approval.
