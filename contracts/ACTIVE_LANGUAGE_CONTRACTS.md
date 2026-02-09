# Active Language Contracts Manifest

This file declares which language contracts are active for CI enforcement and reviewer expectations.

## Rules

1. Core contracts (`contracts/core/*`) are always active.
2. Language contracts are active only when marked `active` below.
3. If this manifest is absent, CI falls back to language autodetection by tracked file extensions.
4. Keep statuses explicit to avoid silent scope drift.

## Status

- rust: active
- python: inactive
- typescript: inactive
