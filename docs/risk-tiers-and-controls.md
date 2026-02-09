# Risk Tiers and Required Controls

Risk tiers scale process rigor with blast radius. Tier assignment is mandatory for meaningful
changes and should be decided before implementation.

## Practical Tier Definitions

- Tier 0: low-risk edits such as documentation, comments, or cosmetic non-executable changes.
- Tier 1: standard feature work without critical data integrity, security, or concurrency risk.
- Tier 2: high-risk paths such as auth/authz, persistence invariants, money/data integrity, or
  concurrency-sensitive logic.
- Tier 3: critical operations with high blast radius, irreversible effects, or safety-sensitive
  behavior.

## Stepwise Tier Assignment Flow

1. Start from Tier 1 by default for executable feature work.
2. If the change is purely non-executable/cosmetic docs, move to Tier 0.
3. Escalate to at least Tier 2 if any escalation trigger is touched (auth, secrets, schema,
   persistence invariants, payments, shared concurrency primitives, critical infra dependencies).
4. Escalate to Tier 3 if failure could be safety-critical, irreversible, or platform-wide.
5. If uncertain between two tiers, pick the higher tier.
6. Record rationale in the evidence packet, and in the task packet when a task packet is required
   (Tier 1-3 by default, or Tier 0 when repo policy requires it).

## Required Controls by Tier (Operational View)

### Tier 0

Use strict TDD/evidence discipline where code exists, but planning artifacts can be lighter. One
reviewer is sufficient. Adversarial review is optional unless risk indicators appear.

Repository owners may choose stricter Tier-0 policy and require task packet/test plan.

### Tier 1

Use task packet, test plan, and evidence packet. Keep strict Red -> Green -> Refactor per feature
slice. One reviewer minimum. Rollback conditions are required.

### Tier 2

Everything in Tier 1 plus mandatory adversarial review checklist, explicit security review, explicit
performance budget checks, and at least two reviewers.

### Tier 3

All Tier 2 controls with highest scrutiny: strong reviewer coverage, explicit exception approval for
any deviation, and full evidence quality with residual risk clearly accepted.

## Escalation Triggers with Examples

- Authentication or authorization touched.
  Example: changing token validation middleware.
- Database schema or persistence invariants touched.
  Example: editing migration semantics for unique constraints.
- Payment/ledger/financial logic touched.
  Example: changing reconciliation rules.
- Shared concurrency primitive touched.
  Example: modifying lock-free queue behavior.
- Critical infrastructure dependency introduced/changed.
  Example: replacing a primary data storage client.

Any of these should move work to at least Tier 2.

## De-escalation Rules

De-escalate only with written rationale and reviewer approval. "Small diff" is not enough to justify
lowering tier if impact remains high.

## Where to Record Tier Decisions

- `templates/TASK_PACKET_TEMPLATE.md` under `## Risk Tier`
- `templates/TEST_PLAN_TEMPLATE.md` under `## Risk Tier`
- `templates/EVIDENCE_PACKET_TEMPLATE.md` under `## Risk Tier`

## Related References

- [Contract Reference Map](./contract-reference-map.md)
- [Using with Coding Agents](./using-with-coding-agents.md)
- [CI and Validation](./ci-and-validation.md)
