# Security Contract

## Purpose

This contract enforces secure-by-default development and review practices.

## Baseline Requirements

1. Threat model required for Tier 2/Tier 3 changes.
2. Validate and sanitize untrusted inputs at boundaries.
3. Enforce least privilege for credentials and access controls.
4. Never hard-code secrets.
5. Avoid unsafe deserialization and dynamic code execution.

## Identity And Access

1. Authentication and authorization logic must be explicit and test-covered.
2. Deny-by-default authorization policy for restricted actions.
3. Sensitive operations require audit-friendly logging.

## Data Protection

1. Classify sensitive data and enforce handling rules.
2. Minimize collection/retention of sensitive data.
3. Encrypt sensitive data in transit and at rest where applicable.
4. Redact secrets and sensitive values from logs.

## Dependency Security

1. New dependencies require explicit justification (`contracts/core/DEPENDENCY_POLICY.md`).
2. Run dependency vulnerability checks in CI where supported.
3. Pin or constrain versions to reduce supply-chain risk.

## Secure Coding Controls

1. No `eval`/`exec`/runtime code generation on untrusted input.
2. Parameterized queries only for database access.
3. Output encoding must match sink context (HTML, SQL, shell, etc.).
4. Avoid broad exception swallowing that can mask security failures.

## Security Testing Requirements

Tier 2/Tier 3 changes must include:

1. Negative tests for unauthorized/invalid access.
2. Boundary tests for malformed input.
3. Explicit abuse-case tests where relevant.

## Evidence Requirements

Security impact must be documented in the evidence packet:

1. Threats considered.
2. Mitigations applied.
3. Residual risks.

## Incident Readiness

1. Security-relevant changes must include rollback guidance.
2. Operational detection hooks should be updated when threat profile changes.
