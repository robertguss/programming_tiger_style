# Adopting Contract System v2 in a New Project

This guide helps you install Contract System v2 into a different repository with minimal disruption.
Default rollout is phased so teams can gain enforcement without stalling delivery.

## Phase 1 (Default Start): Core Contracts + Templates + Validation Scripts

Install core contracts, language contracts, templates, and validators.

### Preferred Path: Tiger Style CLI

If `tiger-style` is installed locally, run:

```bash
tiger-style bootstrap --target /absolute/path/to/your-repo
```

This command runs:

1. `install` (copies contract assets),
2. `configure` (writes `contracts/ACTIVE_LANGUAGE_CONTRACTS.md`, creates `AGENTS.md`),
3. `doctor` (validates structure/tooling; non-strict by default).

The install phase copies `contracts/`, `templates/`, `checklists/`, `scripts/`, and baseline GitHub
workflow/PR template assets.

For dry-run preview:

```bash
tiger-style bootstrap --target /absolute/path/to/your-repo --dry-run
```

After bootstrap, review language activation:

```text
# contracts/ACTIVE_LANGUAGE_CONTRACTS.md
- rust: active|inactive
- python: active|inactive
- typescript: active|inactive
```

Then run an explicit health check:

```bash
tiger-style doctor --target /absolute/path/to/your-repo --format text
```

### Fallback Path: Manual Copy Commands

macOS/Linux:

```bash
CONTRACT_SRC=/absolute/path/to/programming_tiger_style
TARGET_REPO=/absolute/path/to/your-repo

cd "$TARGET_REPO"
mkdir -p contracts/core contracts/languages templates checklists scripts .github/workflows .evidence
cp -R "$CONTRACT_SRC"/contracts/core/. contracts/core/
cp -R "$CONTRACT_SRC"/contracts/languages/. contracts/languages/
cp "$CONTRACT_SRC"/contracts/ACTIVE_LANGUAGE_CONTRACTS.md contracts/ACTIVE_LANGUAGE_CONTRACTS.md
cp -R "$CONTRACT_SRC"/templates/. templates/
cp -R "$CONTRACT_SRC"/scripts/. scripts/
cp "$CONTRACT_SRC"/.github/pull_request_template.md .github/pull_request_template.md
cp "$CONTRACT_SRC"/.github/workflows/contract-gates.yml .github/workflows/contract-gates.yml
chmod +x scripts/*.sh
```

Windows (PowerShell) equivalent:

```powershell
$ContractSrc = "C:\absolute\path\to\programming_tiger_style"
$TargetRepo = "C:\absolute\path\to\your-repo"

Set-Location $TargetRepo
New-Item -ItemType Directory -Force contracts\core, contracts\languages, templates, checklists, scripts, .github\workflows, .evidence | Out-Null
Copy-Item "$ContractSrc\contracts\core\*" contracts\core -Recurse -Force
Copy-Item "$ContractSrc\contracts\languages\*" contracts\languages -Recurse -Force
Copy-Item "$ContractSrc\contracts\ACTIVE_LANGUAGE_CONTRACTS.md" contracts\ACTIVE_LANGUAGE_CONTRACTS.md -Force
Copy-Item "$ContractSrc\templates\*" templates -Recurse -Force
Copy-Item "$ContractSrc\scripts\*" scripts -Recurse -Force
Copy-Item "$ContractSrc\.github\pull_request_template.md" .github\pull_request_template.md -Force
Copy-Item "$ContractSrc\.github\workflows\contract-gates.yml" .github\workflows\contract-gates.yml -Force
```

Run smoke checks:

```bash
bash scripts/validate_tdd_cycle.sh --help
bash scripts/validate_evidence_packet.sh --help
```

Set language activation status before enabling CI:

```text
# contracts/ACTIVE_LANGUAGE_CONTRACTS.md
- rust: active|inactive
- python: active|inactive
- typescript: active|inactive
```

## Phase 2: Risk-Tier Controls + Review Checklists

Add risk-tier enforcement and review discipline:

- Include `contracts/core/RISK_TIER_POLICY.md` in contributor workflow docs.
- Require task packets and test plans for Tier 1+ work.
- Require PR checklist completion for all meaningful PRs.
- Require adversarial review checklist for Tier 2/Tier 3.

Install checklists if not already copied:

```bash
cp -R "$CONTRACT_SRC"/checklists/. checklists/
```

## Phase 3: CI Gate Enforcement + Tiered Adversarial Review

Enable CI checks in your main branch pipeline. If GitHub Actions is used, the copied
`.github/workflows/contract-gates.yml` is ready as a baseline.

For non-GitHub CI systems, port equivalent steps:

1. Validate shell script syntax.
2. Validate TDD commit sequence across the change range.
3. Validate PR body (or attached evidence packet) required headings.
4. Enforce language gates only for languages marked active in
   `contracts/ACTIVE_LANGUAGE_CONTRACTS.md` (or explicit autodetect fallback if manifest is absent).

## Legacy Adoption Mode (Existing Repositories)

For large legacy repositories, use staged rollout from [Legacy Adoption Mode](./legacy-adoption-mode.md):

1. Stage A: enforce on new/touched files.
2. Stage B: ratchet strictness by module and milestone.
3. Stage C: enforce strictness across the full repository.

Do not waive TDD/evidence/risk-tier controls during staged rollout.

## Recommended Downstream Directory Layout

```text
your-repo/
  contracts/
    core/
    ACTIVE_LANGUAGE_CONTRACTS.md
    languages/
  templates/
  checklists/
  scripts/
  .github/
    pull_request_template.md
    workflows/
      contract-gates.yml
  .evidence/
```

## Minimal Viable Adoption vs Full Enforcement

Minimal viable adoption:

- Core contracts present
- Active language manifest declared
- One language contract selected
- Task packet, test plan, evidence packet templates used
- Local script validations run before merge

Full enforcement:

- Tier policy mandatory on all PRs
- PR and adversarial checklists enforced per tier
- CI gates block merges on contract violations
- Reviewer counts enforced by risk tier

## Rollback Path if Adoption Blocks Delivery

Use a controlled rollback rather than deleting the system:

1. Keep templates and evidence requirements active.
2. Keep core validators blocking and use staged language ratchet from Legacy Adoption Mode.
3. Record exceptions with rationale in evidence packets.
4. Re-enable blocking gates after top failure causes are fixed.

This preserves process continuity while reducing immediate friction.

## Next Steps

- Use [Tiger Style CLI](../tooling/cli.md) for command details and conflict/validation behavior.
- Use [AGENTS.md Integration](../guides/agents-integration.md) to align agent-specific instructions.
- Use [Risk Tiers and Controls](../reference/risk-tiers-and-controls.md) to operationalize reviewer and test
  depth by change risk.
- Use [Legacy Adoption Mode](./legacy-adoption-mode.md) when staged rollout is required for existing
  repositories.
