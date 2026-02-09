# Tiger Style v1.1 Upstream Patch List (Draft)

Target upstream repository:

- `/Users/robertguss/Projects/programming_tiger_style`

Date:

- 2026-02-09

## Goal

Define a practical, file-level patch set for Tiger Style `v1.1` that resolves first-adoption friction discovered while auditing `repo-scout`, without weakening rigor.

## Recommended Positioning

Tiger Style should stay strict, but `v1.1` should distinguish:

1. **Policy truth** (contracts must be internally consistent).
2. **Enforcement truth** (CI/scripts must actually enforce policy).
3. **Adoption truth** (legacy repos need a staged path to full compliance).

## Proposed Canonical Decisions (v1.1)

### D-01: Tier 0 planning artifacts are optional by default; Tier 1+ required

Rationale:

- This matches practical docs-only/cosmetic workflows and existing `RISK_TIER_POLICY.md`.
- Keep evidence required for all tiers.

### D-02: Contract gate must include language quality gates, not only script checks

Rationale:

- Language contracts currently define required CI commands; workflow baseline must reflect this.

### D-03: Validators must enforce evidence semantics, not only structure

Rationale:

- Heading/prefix checks alone can produce false compliance.

## Patch Set Overview

| ID | Priority | Theme | Primary Files |
| --- | --- | --- | --- |
| TSV11-01 | P1 | Resolve Tier-0 artifact contradiction | `contracts/core/AI_AGENT_CORE_CONTRACT.md`, `contracts/core/RISK_TIER_POLICY.md`, docs |
| TSV11-02 | P1 | CI baseline parity with language contracts | `.github/workflows/contract-gates.yml`, docs |
| TSV11-03 | P1 | Validator semantic hardening | `scripts/validate_tdd_cycle.sh`, `scripts/validate_evidence_packet.sh`, docs |
| TSV11-04 | P2 | Active language manifest | `contracts/ACTIVE_LANGUAGE_CONTRACTS.md` (new), docs, AGENTS template |
| TSV11-05 | P2 | Legacy adoption mode and ratchet plan | `docs/adopting-in-a-new-project.md`, new docs page, FAQ |
| TSV11-06 | P2 | Recursion policy reframing | all language contracts |
| TSV11-07 | P3 | Test-code allowance clarity | all language contracts |
| TSV11-08 | P2 | PR template/checklist attestation quality | `.github/pull_request_template.md`, `checklists/PR_CONTRACT_CHECKLIST.md` |

## Detailed Patch Specs

### TSV11-01 (P1): Resolve Tier-0 Artifact Contradiction

Problem:

- `AI_AGENT_CORE_CONTRACT.md` currently states all tasks must provide task packet/test plan.
- `RISK_TIER_POLICY.md` allows Tier 0 optional task packet/test plan.

Files to change:

- `contracts/core/AI_AGENT_CORE_CONTRACT.md`
- `contracts/core/RISK_TIER_POLICY.md`
- `docs/using-with-coding-agents.md`
- `docs/agents-integration.md`
- `docs/risk-tiers-and-controls.md`
- `docs/contract-reference-map.md`
- `README.md`

Patch intent:

1. In `AI_AGENT_CORE_CONTRACT.md`, change universal language to tier-aware wording:
   - Tier 1/2/3: task packet + test plan required before implementation.
   - Tier 0: lightweight packet/test plan optional unless repo policy makes them required.
2. Keep `Evidence packet` required for all tiers.
3. Align all docs/examples with the same wording.
4. Add one explicit sentence defining override:
   - Repository policy may choose stricter behavior (for example, requiring task packet/test plan for Tier 0).

Acceptance criteria:

- No document says both “always required” and “optional for Tier 0.”
- One canonical rule appears in contracts and docs.

---

### TSV11-02 (P1): CI Baseline Parity With Language Contracts

Problem:

- `.github/workflows/contract-gates.yml` runs script validators only.
- Language contracts define required CI gates (Rust/Python/TypeScript) not represented in baseline workflow.

Files to change:

- `.github/workflows/contract-gates.yml`
- `docs/ci-and-validation.md`
- `docs/adopting-in-a-new-project.md`
- `docs/contract-reference-map.md`

Patch intent:

1. Split gate logic in workflow into:
   - `contract-core-gates` (scripts)
   - language-specific gates (`rust-gates`, `python-gates`, `typescript-gates`)
2. Language jobs should run only when language is active (see TSV11-04 manifest).
3. Add explicit failure when active language tooling is missing (for example, Rust active but `cargo` unavailable in CI image).
4. Keep core validators mandatory regardless of language.

Minimum command mapping to enforce:

- Rust:
  - `cargo fmt --all -- --check`
  - `cargo clippy --workspace --all-targets --all-features -- -D warnings -D clippy::unwrap_used -D clippy::expect_used -D clippy::undocumented_unsafe_blocks`
  - `cargo test --workspace --all-features`
- Python:
  - `ruff format --check .`
  - `ruff check . --output-format=full`
  - `mypy .`
  - `pytest -q`
- TypeScript:
  - `npx tsc --noEmit`
  - `npx eslint . --max-warnings 0`
  - `npx prettier --check .`
  - `npm test`

Acceptance criteria:

- Baseline workflow enforces both core and language contract gates.
- CI cannot report green while skipping required language quality checks.

---

### TSV11-03 (P1): Validator Semantic Hardening

Problem:

- `validate_tdd_cycle.sh` enforces prefixes/order coarsely; it does not verify robust cycle semantics.
- `validate_evidence_packet.sh` checks headings/placeholders only, not content quality required by contract.

Files to change:

- `scripts/validate_tdd_cycle.sh`
- `scripts/validate_evidence_packet.sh`
- `docs/ci-and-validation.md`
- `docs/faq.md`

Patch intent (`validate_tdd_cycle.sh`):

1. Keep allowed prefixes check.
2. Enforce cycle sequencing with a stricter state model:
   - `GREEN` must have prior unmatched `RED`.
   - `REFACTOR` must have prior unmatched `GREEN`.
3. Detect incomplete trailing cycles (for non-doc ranges).
4. Add `--allow-empty-range` option for smoke checks (default false for CI mode).
5. Keep docs-only skip behavior, but narrow docs-only matcher to explicitly non-executable paths.

Patch intent (`validate_evidence_packet.sh`):

1. Keep required headings check.
2. Add minimal semantic checks:
   - `## Red` must include non-empty failing test name, command, failure summary, expected-failure rationale.
   - `## Green` must include command and passing summary.
   - `## Refactor` must include unchanged-behavior rationale and green confirmation.
   - `## Risk Tier` must include tier value and rationale.
3. Reject generic placeholder statements beyond token checks (`TBD`, `<fill>`, etc.) by requiring non-empty value lines.
4. Add clear error messages indicating missing semantic elements by section.

Acceptance criteria:

- A PR body with headings but empty content fails validation.
- A commit range with invalid staged ordering fails with actionable diagnostics.

---

### TSV11-04 (P2): Active Language Manifest

Problem:

- Partial installs and polyglot scoping are not explicitly declared.
- CI/docs cannot reliably know which language contracts are active.

Files to add/change:

- `contracts/ACTIVE_LANGUAGE_CONTRACTS.md` (new)
- `docs/language-contracts.md`
- `docs/adopting-in-a-new-project.md`
- `docs/templates/AGENTS_TEMPLATE.md`
- `README.md`
- `docs/contract-reference-map.md`

Patch intent:

1. Add a canonical manifest file format, for example:
   - `- rust: active|inactive`
   - `- python: active|inactive`
   - `- typescript: active|inactive`
2. State activation rule:
   - Core contracts always active.
   - Language contracts active only if marked active in manifest (or default autodetect fallback if manifest absent).
3. Update adoption docs to require creating this file on install.
4. Update AGENTS template contract statement to reference this manifest.

Acceptance criteria:

- New adopters can declare language scope unambiguously.
- CI and reviewers can determine expected language gates without guesswork.

---

### TSV11-05 (P2): Legacy Adoption Mode and Ratchet Plan

Problem:

- Hard style limits (for example, function length/line length) are strong, but there is no migration playbook for legacy codebases.

Files to add/change:

- `docs/adopting-in-a-new-project.md`
- `docs/faq.md`
- `docs/ci-and-validation.md`
- `docs/README.md`
- `docs/legacy-adoption-mode.md` (new)

Patch intent:

1. Add a “legacy adoption mode” with explicit stages:
   - Stage A: enforce on new/touched files.
   - Stage B: ratchet target modules by milestone.
   - Stage C: full-repo strict enforcement.
2. Define required tracking artifact for temporary waivers (can be PR evidence + expiration condition).
3. Provide sample rollout timeline and recommended branch protection progression (warn-only -> blocking).
4. Clarify that adoption mode does not waive TDD/evidence requirements.

Acceptance criteria:

- First-time adopters have a deterministic migration path.
- Teams can be strict without forcing all-at-once refactors.

---

### TSV11-06 (P2): Recursion Policy Reframing

Problem:

- Current rule (“no recursion unless design note”) is operationally rigid for AST/parser-heavy code.

Files to change:

- `contracts/languages/RUST_CODING_CONTRACT.md`
- `contracts/languages/PYTHON_CODING_CONTRACT.md`
- `contracts/languages/TYPESCRIPT_CODING_CONTRACT.md`
- `docs/language-contracts.md`

Patch intent:

Replace blanket prohibition with bounded recursion policy:

1. Recursion allowed when one of:
   - input structure is finite and acyclic by construction, or
   - explicit depth/size limit is enforced.
2. Must document rationale and failure mode in design note/evidence.
3. Must include boundary tests around depth and malformed structures.
4. Must provide iterative alternative rationale when recursion depth risk is non-trivial.

Acceptance criteria:

- Contract preserves rigor but avoids unnecessary exception churn.
- Parser/AST use-cases are supportable under explicit controls.

---

### TSV11-07 (P3): Test-Code Allowance Clarity

Problem:

- Production-path prohibitions can be read as applying identically to tests/tooling due to scope wording.

Files to change:

- `contracts/languages/RUST_CODING_CONTRACT.md`
- `contracts/languages/PYTHON_CODING_CONTRACT.md`
- `contracts/languages/TYPESCRIPT_CODING_CONTRACT.md`
- `docs/language-contracts.md`

Patch intent:

Add explicit “test code allowances” subsection in each language contract:

1. Allow limited test-only convenience patterns when failure is intentional and local to test setup/assertion.
2. Disallow using test allowances to mask production-path error handling defects.
3. Require clear separation between test helpers and production modules.

Acceptance criteria:

- Reviewers can consistently evaluate test code without ad-hoc interpretation.

---

### TSV11-08 (P2): PR Template and Checklist Quality Signals

Problem:

- Current template/checklist structure is good, but does not strongly signal tier-dependent artifact exceptions and explicit exception attestation.

Files to change:

- `.github/pull_request_template.md`
- `checklists/PR_CONTRACT_CHECKLIST.md`
- `docs/ci-and-validation.md`

Patch intent:

1. Add explicit checkboxes for:
   - Tier chosen and rationale present.
   - Task packet/test plan present (or Tier 0 optionality justification).
   - Exception section completed if any rule waived.
2. Add one short “evidence completeness” checklist row:
   - Red includes failing command + expected failure reason.
   - Green includes passing command.
   - Refactor includes unchanged behavior justification.
3. Keep template concise; avoid adding long prose.

Acceptance criteria:

- PR authors and reviewers share the same quality threshold before validator runs.

## Suggested Commit Sequence (Upstream Repo)

Use contract-compliant subjects:

1. `DOCS: define Tiger Style v1.1 policy decisions and scope`
2. `DOCS: align tier-0 artifact requirements across contracts and docs`
3. `BUILD: split contract-gates workflow into core and language gates`
4. `GREEN: strengthen validate_tdd_cycle semantics and diagnostics`
5. `GREEN: strengthen validate_evidence_packet semantic checks`
6. `DOCS: add active language manifest and adoption guidance`
7. `DOCS: publish legacy adoption mode and ratchet plan`
8. `DOCS: reframe recursion and test-code allowance rules across language contracts`
9. `DOCS: tighten PR template and checklist attestations`

## v1.1 Release Validation Checklist

Run in upstream repository:

```bash
bash -n scripts/validate_tdd_cycle.sh
bash -n scripts/validate_evidence_packet.sh
bash scripts/validate_tdd_cycle.sh --base origin/main --strict-doc-only
bash scripts/validate_evidence_packet.sh --pr-body .github/pull_request_template.md
```

Plus language-gate smoke runs in representative fixture repos:

```bash
# rust fixture
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings -D clippy::unwrap_used -D clippy::expect_used -D clippy::undocumented_unsafe_blocks
cargo test --workspace --all-features

# python fixture
ruff format --check .
ruff check . --output-format=full
mypy .
pytest -q

# typescript fixture
npx tsc --noEmit
npx eslint . --max-warnings 0
npx prettier --check .
npm test
```

## What I Would Keep / Change / Remove

Keep:

- Strict TDD cycle.
- Risk-tier escalation discipline.
- Evidence-first merge culture.

Change:

- Contract internal consistency.
- CI and validator enforcement depth.
- Operational guidance for legacy adoption.

Remove (or replace):

- Implicit assumption that heading/prefix checks are sufficient proof.
- Blanket recursion prohibition phrasing.

## Open Decisions For You (Before Upstream Execution)

1. Confirm canonical rule choice:
   - `Option A (recommended)`: Tier 0 task packet/test plan optional by default.
   - `Option B`: task packet/test plan required for all tiers.
2. Confirm CI delivery pattern:
   - single workflow with conditional language jobs, or
   - core workflow + per-language workflow templates.
3. Confirm whether validator hardening should be:
   - strict by default immediately, or
   - introduced with temporary warn-only mode for one release.
