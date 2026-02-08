# Test Plan Template

## Objective

- Behavior being verified.

## Risk Tier

- Tier: `0 | 1 | 2 | 3`
- Rationale:

## Invariants

1. Invariant 1
2. Invariant 2

## Red Tests (Must Fail First)

| Test ID | Type             | Scenario | Expected Failure |
| ------- | ---------------- | -------- | ---------------- |
| RED-1   | unit/integration |          |                  |

## Green Validation Tests

| Test ID | Type             | Scenario | Expected Pass Condition |
| ------- | ---------------- | -------- | ----------------------- |
| GREEN-1 | unit/integration |          |                         |

## Refactor Safety Net

- Which tests guard behavior during refactor?
- Which acceptance tests must remain unchanged?

## Boundary And Negative Tests

| Test ID | Boundary/Abuse Case | Expected Behavior |
| ------- | ------------------- | ----------------- |
| NEG-1   |                     |                   |

## Performance Tests

- Benchmarks/load tests required:
- Budgets asserted:

## Security Tests

- Auth/authz cases:
- Malformed input cases:
- Data exposure cases:

## Commands

```bash
# Red

# Green

# Full suite
```

## Exit Criteria

- All required tests pass.
- No flaky/unresolved failures.
