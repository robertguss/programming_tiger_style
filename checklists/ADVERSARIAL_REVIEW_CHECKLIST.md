# Adversarial Review Checklist

Use this checklist to intentionally search for failure modes, not to confirm intent.

## Correctness Attacks

- [ ] Try invalid, malformed, and boundary inputs.
- [ ] Test state transition edge cases and illegal transitions.
- [ ] Verify behavior under retries, duplicate requests, and partial failures.

## Concurrency And Ordering

- [ ] Probe race conditions and ordering assumptions.
- [ ] Verify idempotency where operations may be replayed.
- [ ] Check for deadlock/starvation/resource contention patterns.

## Security Attacks

- [ ] Attempt authorization bypass and privilege escalation paths.
- [ ] Probe injection vectors (query, shell, template, deserialization).
- [ ] Check for sensitive data leaks in logs/errors.

## Performance Attacks

- [ ] Evaluate worst-case input sizes and pathological patterns.
- [ ] Verify queue, retry, and batch bounds under stress.
- [ ] Check for unbounded memory/CPU growth.

## Failure Recovery

- [ ] Validate rollback/fallback behavior.
- [ ] Ensure errors are actionable and preserve context.
- [ ] Confirm observability signals are sufficient for diagnosis.

## Evidence Quality

- [ ] Evidence packet includes concrete command outputs and rationale.
- [ ] Assumptions and unresolved questions are explicitly listed.
- [ ] Residual risks are documented and accepted by reviewers.
