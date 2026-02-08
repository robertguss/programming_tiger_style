# Evidence Requirements

## Purpose

This contract defines the minimum evidence required to merge code changes.

## Evidence Packet Format

Use `templates/EVIDENCE_PACKET_TEMPLATE.md`.

For pull requests, include evidence in one of the following:

1. PR description body.
2. Repository file (for example `.evidence/EVIDENCE_PACKET.md`) linked from PR body.

## Required Sections

Every evidence packet must include these exact section headings:

1. `## Objective`
2. `## Risk Tier`
3. `## Scope`
4. `## Red`
5. `## Green`
6. `## Refactor`
7. `## Invariants`
8. `## Security Impact`
9. `## Performance Impact`
10. `## Assumptions`
11. `## Open Questions`
12. `## Rollback Plan`
13. `## Validation Commands`

## Quality Bar

Evidence is valid only if it is specific and verifiable.

Examples of invalid evidence:

1. "Tests pass" without commands.
2. "No security impact" without rationale.
3. Missing failing-test proof for Red.

## Required Content Detail

### Red

Must include:

1. Test name(s).
2. Command used.
3. Failure summary.
4. Why the failure is expected.

### Green

Must include:

1. Minimal implementation summary.
2. Command used.
3. Passing summary.

### Refactor

Must include:

1. What was refactored.
2. Why behavior is unchanged.
3. Confirmation tests remain green.

## Enforcement

1. CI/local script: `scripts/validate_evidence_packet.sh`.
2. Reviewer checklist: `checklists/PR_CONTRACT_CHECKLIST.md`.

## Retention

Evidence must remain accessible through the merged PR history.
