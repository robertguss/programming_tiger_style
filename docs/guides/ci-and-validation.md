# CI and Validation

This project enforces contract compliance through local scripts and a GitHub workflow gate.

## Local Validators

### `scripts/validate_tdd_cycle.sh`

What it checks:

- Commit subject prefixes are allowed (`RED`, `GREEN`, `REFACTOR`, `DOCS`, `CHORE`, `BUILD`, `TEST`).
- Commit order follows strict Red -> Green -> Refactor state transitions for non-doc changes.
- Incomplete trailing cycles fail.
- Invalid sequencing fails the run.

Typical usage:

```bash
bash scripts/validate_tdd_cycle.sh --base origin/main
```

Safe smoke check:

```bash
bash scripts/validate_tdd_cycle.sh --base HEAD
```

### `scripts/validate_evidence_packet.sh`

What it checks:

- Required headings exist in evidence packet or PR body.
- Placeholder markers are not present.
- `Risk Tier`, `Red`, `Green`, and `Refactor` sections include required semantic fields.

Typical usage with PR body markdown file:

```bash
bash scripts/validate_evidence_packet.sh --pr-body /tmp/pr_body.md
```

Typical usage with repository evidence file:

```bash
bash scripts/validate_evidence_packet.sh --file .evidence/EVIDENCE_PACKET.md
```

## GitHub Actions Gate

Workflow: `.github/workflows/contract-gates.yml`

On pull requests and pushes to `main`, it performs:

1. `detect-active-languages` job: read `contracts/ACTIVE_LANGUAGE_CONTRACTS.md` (or fallback
   autodetect) and expose active language flags.
2. `contract-core-gates` job: shell syntax checks, TDD sequence validation, and evidence validation.
3. Conditional `rust-gates` / `python-gates` / `typescript-gates` jobs for active languages only.
4. Explicit tooling-availability checks for each active language before running language gates.

## PR Body Evidence Requirements

PR content must include these headings:

- `## Objective`
- `## Risk Tier`
- `## Scope`
- `## Red`
- `## Green`
- `## Refactor`
- `## Invariants`
- `## Security Impact`
- `## Performance Impact`
- `## Assumptions`
- `## Open Questions`
- `## Rollback Plan`
- `## Validation Commands`

Semantic minimums enforced by validator:

- `## Risk Tier`: non-empty `Tier` (`0-3`) and `Rationale`.
- `## Red`: failing test, command, failure summary, expected failure rationale.
- `## Green`: command and passing summary.
- `## Refactor`: unchanged-behavior rationale and confirmation command.

The provided `.github/pull_request_template.md` includes required headings, semantic fields, and
exception signaling/checklist attestations.

## Common Failure Signatures and Fixes

### Failure: invalid commit prefix

Example:

```text
Invalid commit prefix in <sha>: '<subject>'
Allowed prefixes: RED, GREEN, REFACTOR, DOCS, CHORE, BUILD, TEST
```

Fix:

- Rewrite commit subjects in the validated range with allowed prefixes.
- Keep ordering consistent with Red -> Green -> Refactor for non-doc changes.

### Failure: green before red

Example:

```text
Invalid sequence: GREEN requires an open RED stage.
```

Fix:

- Split work into proper sequence.
- Ensure failing test commit exists before implementation commit.

### Failure: missing semantic evidence field

Example:

```text
Green section must include Passing summary.
Evidence packet validation failed.
```

Fix:

- Fill the required semantic field with concrete, reviewable content.
- Re-run `scripts/validate_evidence_packet.sh` against PR body markdown.

### Failure: missing evidence headings

Example:

```text
Missing required heading: ## Security Impact
Evidence packet validation failed.
```

Fix:

- Add all required headings exactly as expected.
- Keep headings in Markdown level-2 format.

### Failure: unresolved placeholders

Example:

```text
Evidence packet contains unresolved placeholders.
```

Fix:

- Replace all placeholders with concrete values before merge.

## Running CI-Equivalent Checks Locally

```bash
bash -n scripts/validate_tdd_cycle.sh
bash -n scripts/validate_evidence_packet.sh
bash scripts/validate_tdd_cycle.sh --base origin/main
bash scripts/validate_evidence_packet.sh --pr-body .github/pull_request_template.md
```

## Non-GitHub CI Note

If your project does not use GitHub Actions, keep the same two validator scripts and run them in your
CI provider with equivalent merge-blocking semantics.

## Legacy Rollout Guidance

When adopting in a legacy repository:

1. Keep core gates blocking from day one.
2. Use language-gate ratchet stages from [Legacy Adoption Mode](../getting-started/legacy-adoption-mode.md).
3. Track temporary waivers with expiration criteria in PR evidence.

## Related References

- [Contract Reference Map](../reference/contract-reference-map.md)
- [FAQ](../reference/faq.md)
