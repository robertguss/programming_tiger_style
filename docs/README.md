# Contract System v2 Documentation

This documentation is for maintainers, contributors, and teams adopting Contract System v2 in
AI-assisted software workflows. The goal is practical use: understand each file, run the system
with an agent, and adopt it in new repositories without guesswork.

This folder is the mdBook source (`/Users/robertguss/Projects/programming_tiger_style/book.toml`).
Run `just book-serve` from the repository root for a local docs site.

## Docs Structure

- `getting-started/`: onboarding and rollout guides.
- `guides/`: operator workflows and agent integration.
- `tooling/`: CLI usage and release/install guidance.
- `reference/`: canonical maps, policy references, glossary, and FAQ.
- `decisions/`: locked policy and hardening decisions.
- `templates/`: reusable downstream templates.

## Choose Your Path

### I want to use this repo as-is

Start with the [Quickstart](./getting-started/quickstart.md) to run validations and understand the repository
layout. Then use the [Contract Reference Map](./reference/contract-reference-map.md) as your index.

### I want to run this with an AI coding agent

Start with [Using with Coding Agents](./guides/using-with-coding-agents.md), then review
[CI and Validation](./guides/ci-and-validation.md) so agent output is merge-ready.

### I want to port this into my own repo

Start with [Tiger Style CLI](./tooling/cli.md) for one-command setup, then use
[Adopting in a New Project](./getting-started/adopting-in-a-new-project.md) for phased adoption and rollback
guidance.

## Docs Map

- [Quickstart](./getting-started/quickstart.md): First local run, first validation success, and first TDD/evidence
  loop.
- [Using with Coding Agents](./guides/using-with-coding-agents.md): Codex-first operating model with
  generic adaptation notes.
- [Adopting in a New Project](./getting-started/adopting-in-a-new-project.md): Phased rollout and bootstrap
  commands for downstream repositories.
- [Tiger Style CLI](./tooling/cli.md): Automated install/configure/doctor/bootstrap commands.
- [Legacy Adoption Mode](./getting-started/legacy-adoption-mode.md): Staged strictness and ratchet rollout for
  legacy repositories.
- [Contract Reference Map](./reference/contract-reference-map.md): Canonical file-to-purpose index for
  contracts, templates, checklists, scripts, and CI gates.
- [Risk Tiers and Controls](./reference/risk-tiers-and-controls.md): Practical tier assignment and required
  controls.
- [Language Contracts](./reference/language-contracts.md): Rust, Python, and TypeScript guidance with
  selection rules.
- [CI and Validation](./guides/ci-and-validation.md): Local validations, PR body requirements, and failure
  troubleshooting.
- [AGENTS.md Integration](./guides/agents-integration.md): How to align repository-level agent
  instructions with this contract system.
- [AGENTS.md Starter Template](./templates/AGENTS_TEMPLATE.md): Copyable template for downstream
  repositories.
- [Tiger Style v1.1 Hardening Decisions](./decisions/v1.1-hardening-decisions.md): Canonical policy,
  enforcement, and adoption decisions for v1.1.
- [FAQ](./reference/faq.md): Operational adoption and enforcement answers.
- [Glossary](./reference/glossary.md): Plain-language terms used across this project.

## 10-Minute First Success

In roughly ten minutes, you can clone the repo, inspect the contract layers, and run both
validation scripts with known-good inputs. Follow [Quickstart](./getting-started/quickstart.md), and you should
end with two successful outputs: a TDD validator run that exits cleanly for an empty commit range,
and an evidence validator run that passes against the PR template structure.

## What to Read Next

If you are a repository maintainer, move to [Adopting in a New Project](./getting-started/adopting-in-a-new-project.md).
If you are implementing work through an agent right now, move to
[Using with Coding Agents](./guides/using-with-coding-agents.md).
