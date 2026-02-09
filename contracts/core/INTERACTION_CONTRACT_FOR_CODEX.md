# Interaction Contract For Codex

## Purpose

This contract defines how to collaborate with Codex for highest-quality outcomes.

## Core Principle

Output quality is proportional to input quality. Provide clear constraints, acceptance criteria, and
context boundaries.

## Required Request Structure

Use `templates/TASK_PACKET_TEMPLATE.md` for Tier 1-3 implementation requests.

Minimum required fields:

1. Objective.
2. Non-goals.
3. Files/systems in scope.
4. Constraints and forbidden approaches.
5. Acceptance tests.
6. Risk tier.

Tier 0 requests may use lightweight planning notes unless repository policy explicitly requires the
full task packet/test plan.

## Collaboration Protocol

1. Invite pushback explicitly for unsafe or incoherent requirements.
2. Ask for assumptions and unknowns before coding.
3. Confirm lock-in decisions explicitly (for example, "lock this decision").
4. Keep one primary objective per request when possible.

## Depth Modes

1. Quick mode:

- Use for low-risk, low-ambiguity tasks.
- Favor speed with strict guardrails.

2. Deep mode:

- Use for architecture, Tier 2/Tier 3 changes, or ambiguous requirements.
- Require full task packet, test plan, and evidence packet.

## Agent Output Expectations

Codex should provide:

1. Clear plan or execution sequence.
2. Explicit assumptions and unresolved questions.
3. Red -> Green -> Refactor evidence.
4. Risk-tier-aware testing and validation summary.

## Disagreement And Escalation

If Codex identifies unsafe direction:

1. State the risk clearly.
2. Propose safer alternatives.
3. Request explicit override if user chooses riskier path.

## Session Continuity

For long-running work, capture handoff context with `templates/SESSION_HANDOFF_TEMPLATE.md`.
