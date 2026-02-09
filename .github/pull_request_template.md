## Objective

- Problem solved: Describe the concrete problem this PR addresses.
- Intended outcome: Describe the expected observable outcome after merge.

## Risk Tier

- Tier: `1`
- Rationale: Explain blast radius, reversibility, and data/security impact.

## Scope

- Files/components changed: List the exact files/components changed.
- Explicit exclusions: List what is intentionally out of scope.

## Red

- Failing test(s): Name the first failing test(s) that drove implementation.
- Command(s): `bash <red-command>`
- Failure summary: Describe the observed failure before implementation.
- Expected failure rationale: Explain why the failure is expected before implementation.

## Green

- Minimal implementation summary: Describe the smallest implementation that made Red pass.
- Command(s): `bash <green-command>`
- Passing summary: Record the passing result after implementation.

## Refactor

- Structural improvements: Describe refactors done after Green.
- Why behavior is unchanged: Explain why refactor preserved behavior.
- Confirmation commands: `bash <refactor-confirmation-command>`

## Invariants

- Invariants added/updated: Describe invariants asserted or preserved.
- Boundary checks added/updated: Describe boundary/invalid input checks.

## Security Impact

- Threat-model artifact (Tier 2/Tier 3): Link path/document and freshness note.
- Threats considered: List threats analyzed.
- Mitigations: List mitigations applied.
- Residual risk: Describe remaining risk and ownership.

## Performance Impact

- Baseline: Describe baseline behavior/measurement.
- Post-change: Describe post-change behavior/measurement.
- Delta explanation: Explain performance delta and significance.

## Assumptions

1. List current assumptions and why they are reasonable.

## Open Questions

1. List unresolved questions, if any.

## Rollback Plan

- Trigger conditions: Define when rollback is required.
- Rollback steps: Define explicit rollback actions.

## Validation Commands

```bash
bash scripts/validate_tdd_cycle.sh --base <base-ref>
bash scripts/validate_evidence_packet.sh --pr-body <path-to-pr-body>
```

## Exceptions

- Any contract rule waived: `no` (set to `yes` only with explicit approval)
- Exception approval reference: Link reviewer approval when `yes`.
- Expiration/removal condition: Define when the exception is removed.

## Checklist

- [ ] Tier is selected and rationale is specific.
- [ ] Task packet/test plan are attached for Tier 1-3, or Tier 0 optionality is justified by local policy.
- [ ] Red evidence includes failing command and expected failure rationale.
- [ ] Green evidence includes passing command and passing summary.
- [ ] Refactor evidence includes unchanged-behavior rationale and confirmation command.
- [ ] Exception section is fully completed when any rule is waived.
- [ ] I completed `checklists/PR_CONTRACT_CHECKLIST.md`.
- [ ] I completed `checklists/ADVERSARIAL_REVIEW_CHECKLIST.md` if Tier 2/Tier 3.
