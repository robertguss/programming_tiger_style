## Objective

- Problem solved: test missing refactor semantics.
- Intended outcome: validator rejects incomplete refactor details.

## Risk Tier

- Tier: `1`
- Rationale: validator logic update.

## Scope

- Files/components changed: scripts/validate_evidence_packet.sh.
- Explicit exclusions: workflow changes.

## Red

- Failing test(s): `missing_refactor_unchanged_behavior` fixture.
- Command(s): `bash scripts/tests/test_validate_evidence_packet.sh`
- Failure summary: red section is complete.
- Expected failure rationale: fixture targets refactor semantics.

## Green

- Minimal implementation summary: semantic checks added.
- Command(s): `bash scripts/tests/test_validate_evidence_packet.sh`
- Passing summary: green section is complete.

## Refactor

- Structural improvements: helper extraction.
- Why behavior is unchanged:
- Confirmation commands: `bash scripts/tests/test_validate_evidence_packet.sh`

## Invariants

- Invariants added/updated: required labels remain present.
- Boundary checks added/updated: blank fields fail.

## Security Impact

- Threats considered: heading-only compliance.
- Mitigations: semantic completeness checks.
- Residual risk: reviewers must verify command accuracy.

## Performance Impact

- Baseline: heading checks only.
- Post-change: semantic checks included.
- Delta explanation: negligible.

## Assumptions

1. Heading names remain stable.

## Open Questions

1. Should evidence include run duration?

## Rollback Plan

- Trigger conditions: blocking false positives.
- Rollback steps: revert semantic parser.

## Validation Commands

```bash
bash scripts/tests/test_validate_evidence_packet.sh
```
