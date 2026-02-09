# Changelog

This changelog records notable repository-level changes for `programming_tiger_style`.

## 2026-02-09 - Tiger Style v1.1 Hardening + CLI Rollout

### Added

- Active language manifest contract: `contracts/ACTIVE_LANGUAGE_CONTRACTS.md`.
- Legacy rollout playbook: `docs/getting-started/legacy-adoption-mode.md`.
- Rust CLI crate: `tooling/tiger-style-cli` with `install`, `configure`, `doctor`, and `bootstrap`.
- CLI CI/release workflows:
  - `.github/workflows/cli-ci.yml`
  - `.github/workflows/cli-release.yml`
- mdBook documentation support:
  - `book.toml`
  - `docs/SUMMARY.md`

### Changed

- Tier-0 planning guidance aligned across contracts/docs: Tier 1-3 require task packet + test plan; Tier 0 optional unless repo policy is stricter.
- Contract gates expanded to include active-language quality jobs (Rust/Python/TypeScript) alongside core validators.
- Validator semantics tightened:
  - `scripts/validate_tdd_cycle.sh`
  - `scripts/validate_evidence_packet.sh`
- PR evidence/checklist quality signals strengthened in:
  - `.github/pull_request_template.md`
  - `checklists/PR_CONTRACT_CHECKLIST.md`
- Language-contract clarifications for recursion and test-code allowances.
- Documentation reorganized into:
  - `docs/getting-started/`
  - `docs/guides/`
  - `docs/tooling/`
  - `docs/reference/`
  - `docs/decisions/`

### Source For This Entry

- Migrated from `09-upstream-tiger-style-v1.1-patch-list.md` (plus resulting implemented files).

## 2026-02-07 - Contract System v2 Baseline

### Added

- Core cross-language contract model under `contracts/core/`.
- Risk-tiered controls and evidence-first process.
- Strict Red -> Green -> Refactor enforcement expectations.
- CI enforcement path via:
  - `scripts/validate_tdd_cycle.sh`
  - `scripts/validate_evidence_packet.sh`
  - `.github/workflows/contract-gates.yml`

### Source For This Entry

- Migrated from `CONTRACT_SYSTEM_V2.md`.

## Legacy Source Documents (Retired)

Retired on 2026-02-09 after migration into this changelog:

- `09-upstream-tiger-style-v1.1-patch-list.md`
- `CONTRACT_SYSTEM_V2.md`
