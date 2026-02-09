#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
VALIDATOR="$ROOT_DIR/scripts/validate_tdd_cycle.sh"
FIXTURE_ROOT="$ROOT_DIR/scripts/test-fixtures/tdd-cycle"

if [[ ! -x "$VALIDATOR" ]]; then
  echo "Validator is not executable: $VALIDATOR" >&2
  exit 2
fi

failures=0

run_fixture() {
  local fixture_dir="$1"
  local fixture_name
  fixture_name="$(basename "$fixture_dir")"
  local expected_exit
  expected_exit="$(<"$fixture_dir/expect_exit")"
  local expected_pattern=""

  if [[ -f "$fixture_dir/expect_pattern" ]]; then
    expected_pattern="$(<"$fixture_dir/expect_pattern")"
  fi

  local sandbox
  sandbox="$(mktemp -d)"
  trap 'rm -rf "$sandbox"' RETURN

  git init -q "$sandbox/repo"
  (
    cd "$sandbox/repo"
    git config user.name "Fixture Runner"
    git config user.email "fixtures@example.com"
    git checkout -q -b main

    echo "fixture baseline" > README.md
    git add README.md
    git commit -q -m "DOCS: fixture baseline"
    local base_sha
    base_sha="$(git rev-parse HEAD)"

    while IFS='|' read -r subject path content; do
      [[ -z "$subject" ]] && continue
      mkdir -p "$(dirname "$path")"
      printf '%s\n' "$content" > "$path"
      git add "$path"
      git commit -q -m "$subject"
    done < "$fixture_dir/commits.txt"

    local output
    local status
    set +e
    output="$($VALIDATOR --base "$base_sha" 2>&1)"
    status=$?
    set -e

    if [[ "$status" -ne "$expected_exit" ]]; then
      echo "FAIL [$fixture_name]: expected exit $expected_exit, got $status" >&2
      echo "$output" >&2
      return 1
    fi

    if [[ -n "$expected_pattern" ]] && ! grep -Eq "$expected_pattern" <<<"$output"; then
      echo "FAIL [$fixture_name]: expected output pattern '$expected_pattern'" >&2
      echo "$output" >&2
      return 1
    fi

    echo "PASS [$fixture_name]"
  ) || failures=$((failures + 1))
}

for fixture in "$FIXTURE_ROOT"/*; do
  [[ -d "$fixture" ]] || continue
  run_fixture "$fixture"
done

if [[ "$failures" -ne 0 ]]; then
  echo "validate_tdd_cycle fixture tests failed: $failures scenario(s)" >&2
  exit 1
fi

echo "All validate_tdd_cycle fixtures passed."
