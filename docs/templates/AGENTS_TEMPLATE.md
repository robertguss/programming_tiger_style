# AGENTS.md Template

Use this as a starting point in repositories adopting Contract System v2.

---

# Repository Guidelines

## Project Structure and Module Organization

- Entry points:
- Core modules and responsibilities:
- Integration/system test locations:
- Documentation/planning artifact locations:

## Build, Test, and Development Commands

- Build command:
- Unit test command:
- Integration test command:
- Full validation command:
- Optional local run command:

## Coding Style and Naming Conventions

- Formatting tool and command:
- Naming conventions by language:
- Additional repo-specific style rules:

## Strict TDD and Evidence Rules

- Production code must not be written before a failing test exists for that feature slice.
- Enforce Red -> Green -> Refactor ordering.
- Record evidence for each slice in PR body or evidence packet.
- Required commit prefixes (if enforced): `RED`, `GREEN`, `REFACTOR`, `DOCS`, `CHORE`, `BUILD`,
  `TEST`.

## Risk Tier Rules

- Declare risk tier (`0 | 1 | 2 | 3`) before implementation.
- If uncertain between tiers, choose the higher tier.
- Require controls from `contracts/core/RISK_TIER_POLICY.md`.

## PR and Review Expectations

- Complete `checklists/PR_CONTRACT_CHECKLIST.md` for meaningful changes.
- Complete `checklists/ADVERSARIAL_REVIEW_CHECKLIST.md` for Tier 2/Tier 3.
- Include required evidence headings in PR body.
- Merge only after contract gates pass.

## Validation Commands (Required Before PR)

```bash
bash scripts/validate_tdd_cycle.sh --base origin/main
bash scripts/validate_evidence_packet.sh --pr-body /tmp/pr_body.md
```

## Optional Dogfooding Commands

Use this section if your repository includes a self-querying CLI or analyzer.

- Index/build dogfooding command:
- Query/find dogfooding command:
- Post-change dogfooding command set:

## Security and Configuration Notes

- Secrets handling notes:
- Local-only vs networked behavior notes:
- State/data directories to ignore in git:

## Contract Integration Statement

This repository follows Contract System v2. Core contracts in `contracts/core/` are mandatory.
Language contract activation is declared in `contracts/ACTIVE_LANGUAGE_CONTRACTS.md`, and active
contracts in `contracts/languages/` apply based on changed files. In conflicts, the stricter rule
applies and must be documented in evidence.

---

## How to Customize Safely

1. Replace placeholders with concrete repository paths and commands.
2. Keep strict TDD and evidence requirements intact.
3. Keep risk-tier controls intact.
4. Add repository-specific guidance only when it narrows ambiguity.
5. Avoid adding rules that weaken core contracts.
