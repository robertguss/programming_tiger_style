## Objective

- Problem solved: enforce evidence semantic completeness.
- Intended outcome: validators reject heading-only evidence.

## Risk Tier

- Tier: `1`
- Rationale: validator behavior affects merge controls.

## Scope

- Files/components changed: scripts/validate_evidence_packet.sh and fixtures.
- Explicit exclusions: language contracts and workflow.

## Red

- Failing test(s): `missing_red_expected_failure` fixture.
- Command(s): `bash scripts/tests/test_validate_evidence_packet.sh`
- Failure summary: semantic checks should reject missing required red details.
- Expected failure rationale: fixture intentionally omits one required semantic field.

## Green

- Minimal implementation summary: semantic checks enforced in evidence validator.
- Command(s): `bash scripts/tests/test_validate_evidence_packet.sh`
- Passing summary: fixtures pass after semantic checks are implemented.

## Refactor

- Structural improvements: validation logic grouped by section.
- Why behavior is unchanged: required evidence semantics remain identical.
- Confirmation commands: `bash scripts/tests/test_validate_evidence_packet.sh`

## Invariants

- Invariants added/updated: required semantic labels stay stable.
- Boundary checks added/updated: blank-value fields fail validation.

## Security Impact

- Threats considered: false compliance from empty evidence sections.
- Mitigations: section-level semantic checks.
- Residual risk: reviewers must still verify correctness of claims.

## Performance Impact

- Baseline: heading checks only.
- Post-change: heading plus semantic checks.
- Delta explanation: negligible runtime impact on short markdown content.

## Assumptions

1. Evidence packets keep current heading names.

## Open Questions

1. Should future validation enforce command exit code snippets in evidence?

## Rollback Plan

- Trigger conditions: semantic checks block urgent rollout unexpectedly.
- Rollback steps: temporarily revert strict semantic checks behind a documented exception.

## Validation Commands

```bash
bash scripts/tests/test_validate_evidence_packet.sh
```
