# AGENTS.md Integration

`AGENTS.md` is a repository-level instruction file for coding agents. It works best when it extends,
not replaces, Contract System v2.

## Where AGENTS.md Fits

Use AGENTS.md for repository-specific execution details such as:

- module layout and local architecture orientation
- exact build/test commands
- repo-specific dogfooding expectations
- naming or style conventions unique to that repository

Use Contract System v2 for cross-repository quality controls such as strict TDD, evidence,
risk-tiered controls, and review requirements.

## Precedence and Conflict Handling

Apply instructions in this order:

1. Project safety and compliance requirements
2. Contract System v2 core contracts and risk-tier policy
3. Language contract(s)
4. `AGENTS.md` repository instructions

If two rules conflict, the stricter rule wins. Record the decision and rationale in the evidence
packet and, if needed, in an ADR.

## Integration Pattern for Downstream Repositories

1. Add Contract System v2 files to the target repository.
2. Create or update `AGENTS.md` so it references those files directly.
3. State required command sequence for local validation.
4. State required planning/evidence artifacts.
5. Keep the file focused on repository specifics, not generic process duplication.

## Recommended AGENTS.md Clauses

- "Use task packet + test plan before implementation for Tier 1+."
- "No production code before failing test per feature slice."
- "PRs must include evidence sections required by validator."
- "Run validator scripts before opening PR."
- "If uncertain between risk tiers, choose higher tier."

## Example Agent Startup Checklist

When an agent session starts in a repository:

1. Read `AGENTS.md`.
2. Read relevant files in `contracts/core/` and selected language contract(s).
3. Confirm risk tier for requested change.
4. Create task packet and test plan.
5. Execute strict Red -> Green -> Refactor.
6. Capture evidence and run validators.

## Starter Template

Use [AGENTS.md Starter Template](./templates/AGENTS_TEMPLATE.md) as a base and adapt it to your
repository.

## Related References

- [Using with Coding Agents](./using-with-coding-agents.md)
- [Adopting in a New Project](./adopting-in-a-new-project.md)
- [Contract Reference Map](./contract-reference-map.md)
