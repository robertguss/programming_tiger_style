#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
VALIDATOR="$ROOT_DIR/scripts/validate_evidence_packet.sh"
FIXTURE_ROOT="$ROOT_DIR/scripts/test-fixtures/evidence"

if [[ ! -x "$VALIDATOR" ]]; then
  echo "Validator is not executable: $VALIDATOR" >&2
  exit 2
fi

failures=0

run_fixture() {
  local fixture_dir="$1"
  local fixture_name
  fixture_name="$(basename "$fixture_dir")"
  local packet_path="$fixture_dir/packet.md"
  local expected_exit
  expected_exit="$(<"$fixture_dir/expect_exit")"
  local expected_pattern=""

  if [[ -f "$fixture_dir/expect_pattern" ]]; then
    expected_pattern="$(<"$fixture_dir/expect_pattern")"
  fi

  local output
  local status
  set +e
  output="$($VALIDATOR --file "$packet_path" 2>&1)"
  status=$?
  set -e

  if [[ "$status" -ne "$expected_exit" ]]; then
    echo "FAIL [$fixture_name]: expected exit $expected_exit, got $status" >&2
    echo "$output" >&2
    failures=$((failures + 1))
    return
  fi

  if [[ -n "$expected_pattern" ]] && ! grep -Eq "$expected_pattern" <<<"$output"; then
    echo "FAIL [$fixture_name]: expected output pattern '$expected_pattern'" >&2
    echo "$output" >&2
    failures=$((failures + 1))
    return
  fi

  echo "PASS [$fixture_name]"
}

for fixture in "$FIXTURE_ROOT"/*; do
  [[ -d "$fixture" ]] || continue
  run_fixture "$fixture"
done

if [[ "$failures" -ne 0 ]]; then
  echo "validate_evidence_packet fixture tests failed: $failures scenario(s)" >&2
  exit 1
fi

echo "All validate_evidence_packet fixtures passed."
