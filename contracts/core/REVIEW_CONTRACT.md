# Review Contract

## Purpose

This contract defines review standards for correctness, safety, and maintainability.

## Review Principles

1. Findings-first review: identify risks and defects before style preferences.
2. Evidence-based review: decisions require concrete proof.
3. Severity-based triage: prioritize high-impact issues first.

## Review Workflow

1. Author self-review using `checklists/PR_CONTRACT_CHECKLIST.md`.
2. Primary reviewer pass for correctness and contract compliance.
3. Adversarial reviewer pass for Tier 2/Tier 3 using `checklists/ADVERSARIAL_REVIEW_CHECKLIST.md`.

## Required Reviewer Checks

1. Red -> Green -> Refactor evidence validity.
2. Risk tier assignment validity.
3. Invariants and boundary handling.
4. Error handling coverage.
5. Security/performance impacts.
6. Rollback readiness.

## Finding Severity Levels

1. P0: merge-blocking correctness/safety/security issue.
2. P1: high-risk bug or likely production regression.
3. P2: medium-risk maintainability or reliability concern.
4. P3: low-risk improvement suggestion.

## Merge Conditions

A PR may merge only when:

1. P0 and P1 findings are resolved or waived with approved exception.
2. Required checks pass.
3. Required reviewer approvals are present for assigned tier.

## Review Output

Review comments should include:

1. Concrete risk statement.
2. Impacted file/area.
3. Suggested remediation or decision needed.
