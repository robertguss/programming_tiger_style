# PR Contract Checklist

## TDD Compliance

- [ ] A failing test was written first and observed failing for expected reason (Red).
- [ ] Minimal implementation made Red test pass (Green).
- [ ] Refactor happened only after Green and tests remained green.
- [ ] PR contains explicit Red -> Green -> Refactor evidence.

## Evidence Completeness

- [ ] Red evidence includes failing test name, command, failure summary, and expected failure rationale.
- [ ] Green evidence includes command and passing summary.
- [ ] Refactor evidence includes unchanged-behavior rationale and green confirmation command.

## Risk And Scope

- [ ] Risk tier declared with rationale tied to blast radius/reversibility.
- [ ] Task packet/test plan are present for Tier 1-3, or Tier 0 optionality is explicitly justified.
- [ ] Scope is bounded and matches objective.
- [ ] Rollback plan is documented.

## Correctness And Safety

- [ ] Preconditions/postconditions/invariants are asserted where critical.
- [ ] Boundary and invalid-input cases are covered by tests.
- [ ] Error handling is explicit; no swallowed critical failures.

## Security

- [ ] Security impact is assessed and documented.
- [ ] No secret leakage in code/logging.
- [ ] Auth/authz and sensitive paths include negative tests (Tier 2/Tier 3).
- [ ] Tier 2/Tier 3 changes include a linked threat-model artifact and residual risk owner.

## Performance

- [ ] Performance impact is documented.
- [ ] Relevant performance budgets are checked (Tier 2/Tier 3).

## Dependencies

- [ ] New dependencies (if any) include written justification and risk assessment.

## Exceptions

- [ ] If any contract rule is waived, exception details, approval reference, and expiration condition are documented.

## Review And CI

- [ ] Required CI gates pass with zero warnings.
- [ ] Required reviewer count is satisfied for this tier.
- [ ] Adversarial review checklist completed for Tier 2/Tier 3.
