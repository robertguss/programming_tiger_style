# Glossary

## Feature Slice

The smallest user-visible behavior unit implemented and validated independently. Each feature slice
must follow strict Red -> Green -> Refactor before the next slice starts.

See [Using with Coding Agents](../guides/using-with-coding-agents.md).

## Red -> Green -> Refactor

A strict TDD cycle:

- Red: write and run a failing test first.
- Green: implement the minimum change to pass that test.
- Refactor: improve structure while preserving behavior and keeping full suite green.

See [Quickstart](../getting-started/quickstart.md) and [CI and Validation](../guides/ci-and-validation.md).

## Evidence Packet

A structured artifact documenting objective, risk tier, scope, red/green/refactor evidence,
security/performance impact, assumptions, open questions, rollback plan, and validation commands.

See `templates/EVIDENCE_PACKET_TEMPLATE.md` and [Contract Reference Map](./contract-reference-map.md).

## Adversarial Review

A deliberate review mode that tries to break the change through invalid inputs, ordering issues,
security abuse paths, and stress/failure conditions. Mandatory for Tier 2/Tier 3.

See `checklists/ADVERSARIAL_REVIEW_CHECKLIST.md` and
[Risk Tiers and Controls](./risk-tiers-and-controls.md).

## Blast Radius

The scale of impact if a change fails, including user impact, data integrity, security, and
operational stability. Blast radius is a primary input to risk tier assignment.

See `contracts/core/RISK_TIER_POLICY.md` and [Risk Tiers and Controls](./risk-tiers-and-controls.md).

## Task Packet

A structured request artifact defining objective, non-goals, scope, constraints, risk tier,
interfaces, acceptance criteria, and expected tests before coding begins.

See `templates/TASK_PACKET_TEMPLATE.md` and [Using with Coding Agents](../guides/using-with-coding-agents.md).

## Test Plan

A pre-implementation artifact defining red tests, green validation, boundary/negative cases,
security/performance checks, and commands.

See `templates/TEST_PLAN_TEMPLATE.md` and [Using with Coding Agents](../guides/using-with-coding-agents.md).

## Contract Gates

Automated CI checks that enforce TDD sequence and evidence structure before merge.

See `.github/workflows/contract-gates.yml` and [CI and Validation](../guides/ci-and-validation.md).
