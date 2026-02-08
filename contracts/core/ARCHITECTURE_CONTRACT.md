# Architecture Contract

## Purpose

This contract defines architecture standards that keep systems testable, safe, and maintainable.

## Architectural Principles

1. Functional core, imperative shell:

- Keep domain logic pure where practical.
- Isolate I/O, side effects, and framework glue at boundaries.

2. Explicit boundaries:

- Clearly separate domain, infrastructure, and interface layers.

3. Dependency direction:

- Dependencies must point inward toward stable domain logic.

4. Small composable units:

- Favor cohesive modules over broad utility dumping grounds.

## Required Patterns

1. Hexagonal/ports-and-adapters style for external systems.
2. Explicit domain models for important business invariants.
3. State-machine modeling where behavior has discrete transitions.
4. Idempotent operation design for retried workflows.

## Anti-Patterns

1. Business logic in controllers/handlers/routes.
2. Hidden global mutable state as implicit dependency.
3. Cross-layer imports that bypass defined boundaries.
4. Temporal coupling without explicit ordering contracts.

## Design Artifacts

For Tier 2/Tier 3 work, include:

1. Task packet (`templates/TASK_PACKET_TEMPLATE.md`).
2. Test plan (`templates/TEST_PLAN_TEMPLATE.md`).
3. ADR for meaningful architectural decisions (`templates/ADR_TEMPLATE.md`).

## Modularity Rules

1. Keep module APIs small and explicit.
2. Avoid circular dependencies.
3. Keep naming domain-centric and consistent.

## Verification

1. Architecture assumptions and boundaries must be asserted by tests.
2. Integration tests must validate adapter behavior at external boundaries.
3. Refactors must preserve public contracts unless intentionally versioned.
