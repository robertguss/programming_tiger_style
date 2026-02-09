# Using Contract System v2 with Coding Agents

This guide describes how to run this system with coding agents. It is Codex-first and includes
adaptation notes for other agents.

## Agent Briefing Inputs (Required)

Before asking an agent to implement, provide the information required by
`contracts/core/INTERACTION_CONTRACT_FOR_CODEX.md` and map it to `templates/TASK_PACKET_TEMPLATE.md`.

Minimum briefing fields:

- Objective
- Non-goals
- Scope (in-scope and out-of-scope files/systems)
- Constraints and forbidden approaches
- Acceptance criteria
- Risk tier and rationale

If any of these are missing for Tier 1-3 work, stop and complete the packet before
implementation. Tier 0 may use lightweight planning notes unless repository policy requires full
artifacts.

## Prompt Structure You Can Reuse

```text
Objective:
Non-goals:
Scope:
Constraints:
Acceptance criteria:
Risk tier (0-3) and rationale:

Create/update:
- Task packet (required for Tier 1-3; Tier 0 optional unless repo policy requires)
- Test plan (required for Tier 1-3; Tier 0 optional unless repo policy requires)
- Evidence packet

Enforce strict Red -> Green -> Refactor per feature slice.
Do not write production code before a failing test exists.
```

## Agent Execution Loop

Use this sequence for every meaningful change:

1. For Tier 1-3, create a task packet from `templates/TASK_PACKET_TEMPLATE.md` (Tier 0 optional
   unless repo policy requires it).
2. For Tier 1-3, create a test plan from `templates/TEST_PLAN_TEMPLATE.md` (Tier 0 optional unless
   repo policy requires it).
3. Execute strict per-slice Red -> Green -> Refactor.
4. Capture evidence in `templates/EVIDENCE_PACKET_TEMPLATE.md`.
5. Complete `checklists/PR_CONTRACT_CHECKLIST.md`.
6. If Tier 2/Tier 3, complete `checklists/ADVERSARIAL_REVIEW_CHECKLIST.md`.
7. Run validators:
   - `bash scripts/validate_tdd_cycle.sh --base <base-ref>`
   - `bash scripts/validate_evidence_packet.sh --pr-body <pr-body-file>` or
     `--file .evidence/EVIDENCE_PACKET.md`

## Codex-First Operating Notes

- Ask Codex to restate assumptions and unknowns before coding.
- Require explicit red/green/refactor evidence in agent output.
- Require the agent to declare risk tier and controls from
  `contracts/core/RISK_TIER_POLICY.md`.
- Require explicit pushback if requested work is unsafe or contradictory.

## Generic Adaptation Notes for Other Agents

Most agent platforms can follow this system if you preserve the same lifecycle and artifacts:

- Keep strict Red -> Green -> Refactor ordering.
- Keep the same template files for planning and evidence.
- Keep the same risk-tier decision policy.
- Keep the same CI enforcement expectations.

Do not replace evidence with natural-language claims. Evidence must include commands and observed
results.

## Common Failure Modes When Using Agents

- Jumping straight to implementation without a failing test.
- Bundling multiple feature slices into one large red/green loop.
- Skipping risk tier assignment for "small" changes touching auth/persistence.
- Submitting PRs without required evidence headings.

Use [CI and Validation](./ci-and-validation.md) for exact failure signatures and remediations.
