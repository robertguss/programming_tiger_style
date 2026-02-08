# Quickstart

This guide gets you to first successful use of Contract System v2. After completing it, you will
know where the contracts live, how validation works, and what a strict TDD + evidence loop looks
like in practice.

## Prerequisites

- Git
- Bash (macOS/Linux default shell is fine)
- A local clone path you can write to

## 1) Clone and Enter the Repository (macOS/Linux)

```bash
git clone https://github.com/robertguss/programming_tiger_style.git
cd programming_tiger_style
```

If you already have a clone, run:

```bash
cd /path/to/programming_tiger_style
git pull
```

Windows (PowerShell) equivalent:

```powershell
git clone https://github.com/robertguss/programming_tiger_style.git
Set-Location programming_tiger_style
```

## 2) Inspect the Contract Layers

Run:

```bash
ls contracts/core contracts/languages templates checklists scripts .github/workflows
```

You should see these major groups:

- `contracts/core/`: cross-language rules (TDD, risk, security, review, architecture, evidence)
- `contracts/languages/`: Rust/Python/TypeScript rules
- `templates/`: task packet, test plan, evidence packet, ADR, handoff
- `checklists/`: PR checklist and adversarial review checklist
- `scripts/`: validation scripts
- `.github/workflows/contract-gates.yml`: CI enforcement entrypoint

Windows (PowerShell) note:

```powershell
Get-ChildItem contracts/core, contracts/languages, templates, checklists, scripts, .github/workflows
```

## 3) Run Validation Scripts Locally

Run the TDD validator against an empty range (safe smoke check):

```bash
bash scripts/validate_tdd_cycle.sh --base HEAD
```

Expected output:

```text
No commits in range HEAD..HEAD. Nothing to validate.
```

Run evidence validation against the PR template body:

```bash
bash scripts/validate_evidence_packet.sh --pr-body .github/pull_request_template.md
```

Expected output:

```text
PR body evidence validation passed.
```

Windows note: use Git Bash for these shell scripts, or execute equivalent logic in a PowerShell
wrapper.

## 4) First Red -> Green -> Refactor + Evidence Mini Flow

This repository is primarily contracts and templates, so the quickest way to practice strict TDD is
on a target code repository that uses this system.

For a feature slice in that target repository:

1. Red: add a failing automated test first and run only that test.
2. Green: add minimal production code to make that same test pass.
3. Refactor: improve structure without behavior changes and run full suite.
4. Evidence: record all three stages in an evidence packet.

Example command pattern (replace with your project toolchain):

```bash
# red
pytest tests/test_feature.py::test_rejects_invalid_limit

# green
pytest tests/test_feature.py::test_rejects_invalid_limit

# refactor gate
pytest
```

Then document the cycle using `templates/EVIDENCE_PACKET_TEMPLATE.md` sections:

- `## Red`
- `## Green`
- `## Refactor`
- `## Validation Commands`

## Next Steps

- Use [Using with Coding Agents](./using-with-coding-agents.md) to run this workflow through Codex
  or another agent.
- Use [Contract Reference Map](./contract-reference-map.md) when you need exact file purpose.
