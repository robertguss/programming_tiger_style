# TDD Enforcement Contract

## Purpose

This contract enforces strict Test-Driven Development for all production code changes.

## Mandatory Cycle

All changes must follow this order:

1. Red: write or modify a test that fails for the expected reason.
2. Green: implement the minimum code required to pass that failing test.
3. Refactor: improve code structure while preserving behavior and keeping tests green.

Skipping Red, collapsing stages without proof, or coding first is non-compliant.

## Commit Taxonomy

For non-doc-only changes, commit subject lines must use these prefixes:

1. `RED:`
2. `GREEN:`
3. `REFACTOR:`

Additional prefixes allowed for supporting commits:

1. `DOCS:`
2. `CHORE:`
3. `BUILD:`
4. `TEST:`

At least one complete Red -> Green -> Refactor sequence is required in each change set.

## Stage Requirements

### Red Stage Requirements

1. Introduce a failing test tied to a behavior.
2. Confirm failure with command output.
3. Verify failure reason is the expected one.

### Green Stage Requirements

1. Implement only what is necessary to satisfy Red.
2. Run targeted tests until the Red test passes.
3. Avoid unrelated refactors.

### Refactor Stage Requirements

1. Improve design/readability/performance without changing behavior.
2. Re-run relevant tests.
3. Keep acceptance tests green.

## Bug Fix Rule

Every bug fix must include a regression test that:

1. Fails before the fix.
2. Passes after the fix.

## Forbidden Patterns

1. Writing implementation before any failing test.
2. Changing tests to match broken behavior without explicit requirement change.
3. Mixing broad refactors into Green.
4. Marking flaky tests as "pass" by retries instead of root-cause fixes.

## Evidence Requirements

Each PR must include:

1. Red evidence (test command and failing output summary).
2. Green evidence (test command and passing output summary).
3. Refactor evidence (what changed structurally, plus green test confirmation).

Use `templates/EVIDENCE_PACKET_TEMPLATE.md`.

## Enforcement

1. Local/CI enforcement script: `scripts/validate_tdd_cycle.sh`.
2. PR evidence enforcement script: `scripts/validate_evidence_packet.sh`.
3. Manual reviewer verification using `checklists/PR_CONTRACT_CHECKLIST.md`.

## Exceptions (Emergency Use Only)

TDD exceptions are strongly discouraged and allowed only for urgent incident mitigation where delay
would create greater risk.

An exception request must include:

1. Why strict Red -> Green -> Refactor cannot be executed first.
2. Immediate risk controls applied.
3. Timeboxed follow-up plan to restore full TDD coverage.
4. Expiration condition for the exception.

Explicit approval means both of the following:

1. One code owner approval.
2. One technical lead or maintainer approval.

The exception details and approvals must be recorded in the evidence packet and PR discussion.
