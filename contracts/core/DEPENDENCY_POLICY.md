# Dependency Policy

## Purpose

This policy controls dependency growth and supply-chain risk.

## Default Position

Default is no new dependencies unless there is clear, documented justification.

## Evaluation Criteria

For each proposed dependency, document:

1. Problem being solved.
2. Why existing code/tools are insufficient.
3. Maintenance posture and release quality.
4. Security posture and known vulnerability history.
5. Runtime/performance cost.
6. Lock-in and migration risk.

## Approval Rules

1. Tier 0/Tier 1: one reviewer approval.
2. Tier 2/Tier 3: two reviewer approvals, one focused on security/operations.

## Constraints

1. Prefer mature, actively maintained, widely adopted dependencies.
2. Avoid overlapping dependencies solving the same problem.
3. Prefer narrow-scope packages over monoliths when feasible.
4. Pin or tightly constrain versions in lockfiles.

## Ongoing Maintenance

1. Run vulnerability checks where language ecosystem supports it.
2. Remove stale/unused dependencies.
3. Reassess dependency necessity periodically.

## Evidence Requirements

Dependency additions must be recorded in the evidence packet with:

1. Rationale.
2. Risk assessment.
3. Rollback/removal strategy.
