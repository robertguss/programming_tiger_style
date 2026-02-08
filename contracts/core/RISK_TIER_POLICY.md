# Risk Tier Policy

## Purpose

This policy defines risk tiers and required controls for each change.

## Tier Definitions

1. Tier 0 (Low): docs, comments, non-executable metadata, cosmetic refactors.
2. Tier 1 (Moderate): normal feature work without critical data/security/concurrency impact.
3. Tier 2 (High): money/data integrity paths, auth/authz, persistence, distributed state,
   concurrency-sensitive code.
4. Tier 3 (Critical): safety-critical logic, cryptographic controls, irreversible operations,
   high-blast-radius platform primitives.

## Required Controls By Tier

| Control                           | Tier 0           | Tier 1           | Tier 2           | Tier 3           |
| --------------------------------- | ---------------- | ---------------- | ---------------- | ---------------- |
| Red -> Green -> Refactor evidence | Required         | Required         | Required         | Required         |
| Task packet                       | Optional         | Required         | Required         | Required         |
| Test plan                         | Optional         | Required         | Required         | Required         |
| Evidence packet                   | Required         | Required         | Required         | Required         |
| Adversarial review checklist      | Optional         | Recommended      | Required         | Required         |
| Security review                   | Optional         | Recommended      | Required         | Required         |
| Performance budget check          | Optional         | Recommended      | Required         | Required         |
| Rollback plan                     | Optional         | Required         | Required         | Required         |
| Reviewer count                    | 1                | 1                | 2                | 2+               |
| Explicit exception approval       | Required if used | Required if used | Required if used | Required if used |

## Assignment Rules

Assign the highest applicable tier using blast radius, reversibility, and data impact.

If uncertain between two tiers, select the higher tier.

## Escalation Triggers

Automatically escalate to at least Tier 2 when a change touches:

1. Authentication, authorization, or secrets handling.
2. Database schema/migrations and persistence invariants.
3. Payment, billing, financial, or ledger behavior.
4. Shared concurrency primitives.
5. Critical infrastructure dependencies.

## De-escalation

Tier may be reduced only with documented rationale and reviewer approval.

## Auditing

Every PR must declare tier and rationale in the evidence packet.
