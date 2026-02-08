# Contract System v2

## Objective

Provide an executable, auditable contract system for high-quality AI-assisted coding.

## What v2 Adds

1. Cross-language core contracts in `contracts/core/`.
2. Mandatory Red -> Green -> Refactor enforcement.
3. Risk-tier policy to scale rigor by blast radius.
4. Evidence packet requirements for every meaningful change.
5. Adversarial review model for high-risk work.
6. CI automation for TDD/evidence validation.

## Enforcement Path

1. PR template collects structured evidence.
2. `scripts/validate_tdd_cycle.sh` checks commit history sequence.
3. `scripts/validate_evidence_packet.sh` checks required evidence sections.
4. `.github/workflows/contract-gates.yml` runs checks on pull requests.

## Adoption Sequence

1. Require task packets and test plans for all Tier 1+ work.
2. Require evidence packets for all Tier 1+ PRs.
3. Enforce adversarial review for Tier 2/Tier 3.
4. Monitor defects and update contracts based on escaped failures.

## Success Criteria

1. Lower escaped defect rate.
2. Lower change failure rate.
3. Higher confidence in AI-generated code.
4. Better review quality through structured evidence.
