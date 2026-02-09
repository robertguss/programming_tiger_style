# FAQ

## Do I need all files on day one?

No. Start with phased adoption from [Adopting in a New Project](../getting-started/adopting-in-a-new-project.md).
Minimum useful baseline is core contracts, one language contract, templates, and validation scripts.
Then add tiered review controls and CI gates.

## How strict is commit prefix enforcement?

For non-doc changes validated by `scripts/validate_tdd_cycle.sh`, commit subjects must use allowed
prefixes and preserve Red -> Green -> Refactor ordering. Docs/contracts-only ranges can skip strict
prefix sequence unless `--strict-doc-only` is set.

See [CI and Validation](../guides/ci-and-validation.md) for details and failure signatures.

## What if my repository uses non-GitHub CI?

Keep the same script checks and run them in your CI provider. You need equivalent merge-blocking
steps for:

1. `scripts/validate_tdd_cycle.sh`
2. `scripts/validate_evidence_packet.sh`

The provider can change; the contract semantics should not.

## How do we roll this out in a large legacy codebase?

Use [Legacy Adoption Mode](../getting-started/legacy-adoption-mode.md) and ratchet in stages:

1. Stage A: enforce on new/touched files.
2. Stage B: expand module by module with explicit milestones.
3. Stage C: full-repository strictness.

This staged model does not waive TDD, evidence, or risk-tier controls.

## How do I handle docs-only changes?

By default, `scripts/validate_tdd_cycle.sh` recognizes docs/contracts-only diffs and skips strict
prefix sequence checks. Keep evidence expectations proportionate to risk tier and repository policy.

## Can I use this without Codex?

Yes. Codex is the primary collaboration model here, but the templates and validators are agent
agnostic. Keep the same artifacts and lifecycle with any agent platform.

See [Using with Coding Agents](../guides/using-with-coding-agents.md).

## What if a change touches both low-risk docs and high-risk code?

Assign tier by highest impact. If any scope element triggers Tier 2/Tier 3 rules, the full higher-
tier controls apply.

See [Risk Tiers and Controls](./risk-tiers-and-controls.md).

## Where is the canonical file index?

Use [Contract Reference Map](./contract-reference-map.md). It maps each file to purpose, usage
moment, and workflow step.
