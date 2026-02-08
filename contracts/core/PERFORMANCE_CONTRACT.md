# Performance Contract

## Purpose

This contract enforces performance-aware design and measurable performance validation.

## Baseline Rules

1. Define expected workload and target latency/throughput before implementation.
2. Apply explicit bounds on batch size, queue depth, retries, and concurrency.
3. Optimize bottlenecks in order of impact, not intuition.
4. Prefer predictable performance over fragile micro-optimizations.

## Performance Budgets

For Tier 2/Tier 3 changes, define budgets for:

1. Latency (p50/p95/p99 where relevant).
2. Throughput.
3. Memory growth.
4. CPU utilization.

If budgets are unknown, the task remains in Discovery mode.

## Design-Time Requirements

1. Include back-of-the-envelope resource estimates.
2. Identify hot paths and likely contention points.
3. State expected failure behavior under load.

## Implementation Requirements

1. Preserve asymptotic complexity expectations.
2. Avoid hidden unbounded loops and unbounded data growth.
3. Make expensive operations explicit at call sites.

## Test Requirements

1. Add or update micro/benchmark tests for changed hot paths.
2. Include boundary-load tests for queueing/retry/concurrency behavior.
3. Keep performance tests deterministic enough for trend tracking.

## Regression Policy

A measurable regression against declared budget requires one of:

1. A fix before merge.
2. An approved temporary exception with expiration condition.

## Evidence Requirements

Performance impact section in the evidence packet must include:

1. Baseline measurement.
2. Post-change measurement.
3. Explanation of deltas.
