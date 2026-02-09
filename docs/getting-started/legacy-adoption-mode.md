# Legacy Adoption Mode

Legacy Adoption Mode provides a staged migration path for repositories with pre-existing style debt.
It reduces adoption risk without waiving core rigor controls.

## Non-Negotiable Controls

These controls remain mandatory in every stage:

1. Strict Red -> Green -> Refactor behavior.
2. Evidence packet completeness and validator compliance.
3. Risk-tier assignment and tier-specific controls.
4. Explicit exception approvals with compensating controls.

## Staged Rollout Model

### Stage A: New/Touched Files Enforcement

- Enforce language/style rules for new files and directly modified files.
- Keep core gates (`validate_tdd_cycle.sh`, `validate_evidence_packet.sh`) blocking.
- Allow temporary legacy waivers only for untouched modules.

### Stage B: Ratchet By Module

- Select priority modules per milestone and enforce full language rules there.
- Track module completion in release planning.
- Shrink waiver scope every milestone; no indefinite waivers.

### Stage C: Full Repository Strictness

- Enforce all active language contracts across the entire repository.
- Remove remaining temporary waivers.
- Keep branch protection fully blocking on contract gates.

## Waiver Tracking Requirements

Any temporary waiver must be tracked in PR evidence and include:

1. Exact rule waived and affected files/modules.
2. Risk introduced and compensating controls.
3. Expiration condition (date or milestone).
4. Owner accountable for removal.

## Suggested Rollout Timeline

Example 12-week pattern:

1. Weeks 1-2: Stage A baseline on all new/touched code.
2. Weeks 3-8: Stage B ratchet for highest-risk modules first.
3. Weeks 9-12: Stage C full strict enforcement and waiver burn-down.

Adjust timing by repository size and change velocity, but keep explicit stage exit criteria.

## Branch Protection Progression

1. Early Stage A: optional parallel monitor job plus blocking core gates.
2. Stable Stage A/B: blocking core gates and blocking language gates for ratcheted modules.
3. Stage C: blocking core + all active language gates for full repository.

## Exit Criteria

Legacy Adoption Mode is complete only when:

1. No active temporary waivers remain.
2. Full repository strict language enforcement is enabled.
3. Core and language gates run as blocking checks in default branch protection.
