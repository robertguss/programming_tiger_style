#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: scripts/validate_tdd_cycle.sh [--base <commit-ish>] [--strict-doc-only] [--allow-empty-range]

Validates commit history from <base>..HEAD for Red -> Green -> Refactor sequencing.

Options:
  --base <commit-ish>   Base revision for commit range (default: origin/main or repo root commit)
  --strict-doc-only     Enforce TDD commit prefixes even for docs-only changes
  --allow-empty-range   Allow empty commit ranges (default in local runs, opt-in in CI)
USAGE
}

BASE_REF=""
STRICT_DOC_ONLY=0
ALLOW_EMPTY_RANGE=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    --base)
      BASE_REF="${2:-}"
      shift 2
      ;;
    --strict-doc-only)
      STRICT_DOC_ONLY=1
      shift
      ;;
    --allow-empty-range)
      ALLOW_EMPTY_RANGE=1
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Unknown argument: $1" >&2
      usage
      exit 2
      ;;
  esac
done

if [[ -z "$BASE_REF" ]]; then
  if git rev-parse --verify --quiet origin/main >/dev/null; then
    BASE_REF="origin/main"
  else
    BASE_REF="$(git rev-list --max-parents=0 HEAD | tail -n 1)"
  fi
fi

if ! git rev-parse --verify --quiet "${BASE_REF}^{commit}" >/dev/null; then
  echo "Base ref not found: $BASE_REF" >&2
  exit 2
fi

is_non_executable_path() {
  local path="$1"
  case "$path" in
    *.md|*.rst|*.txt|LICENSE)
      return 0
      ;;
    docs/*|contracts/*|templates/*|checklists/*|.evidence/*)
      return 0
      ;;
    resources/*.md|resources/*.txt|resources/*.pdf)
      return 0
      ;;
    .github/pull_request_template.md|.gitignore|.prettierrc.json)
      return 0
      ;;
    README.md|CHANGELOG.md|PLANS.md)
      return 0
      ;;
    *)
      return 1
      ;;
  esac
}

RANGE="${BASE_REF}..HEAD"
COMMITS=()
while IFS= read -r commit; do
  COMMITS+=("$commit")
done < <(git rev-list --reverse "$RANGE")

if [[ ${#COMMITS[@]} -eq 0 ]]; then
  if [[ "$ALLOW_EMPTY_RANGE" -eq 1 || -z "${CI:-}" ]]; then
    echo "No commits in range $RANGE. Nothing to validate."
    exit 0
  fi

  echo "No commits in range $RANGE and empty ranges are blocked in CI without --allow-empty-range." >&2
  exit 1
fi

if [[ "$STRICT_DOC_ONLY" -eq 0 ]]; then
  DOC_ONLY=1
  while IFS= read -r path; do
    [[ -z "$path" ]] && continue
    if ! is_non_executable_path "$path"; then
      DOC_ONLY=0
      break
    fi
  done < <(git diff --name-only "$RANGE")

  if [[ "$DOC_ONLY" -eq 1 ]]; then
    echo "Docs-only change detected. Skipping strict TDD commit prefix check."
    exit 0
  fi
fi

STATE="READY_FOR_RED"
COMPLETED_CYCLES=0
LAST_STAGE_COMMIT=""
LAST_STAGE_SUBJECT=""

expected_prefix_for_state() {
  case "$STATE" in
    READY_FOR_RED)
      echo "RED (or non-cycle commit: DOCS/CHORE/BUILD/TEST)"
      ;;
    READY_FOR_GREEN)
      echo "GREEN (or non-cycle commit: DOCS/CHORE/BUILD/TEST)"
      ;;
    READY_FOR_REFACTOR)
      echo "REFACTOR (or non-cycle commit: DOCS/CHORE/BUILD/TEST)"
      ;;
    *)
      echo "RED"
      ;;
  esac
}

print_sequence_error() {
  local commit="$1"
  local subject="$2"
  local message="$3"

  echo "$message" >&2
  echo "Commit: $commit ($subject)" >&2
  echo "State before commit: $STATE" >&2
  echo "Expected next prefix: $(expected_prefix_for_state)" >&2
  exit 1
}

for commit in "${COMMITS[@]}"; do
  parent_line="$(git rev-list --parents -n 1 "$commit")"
  parent_count=$(( $(wc -w <<<"$parent_line") - 1 ))
  if [[ "$parent_count" -gt 1 ]]; then
    continue
  fi

  subject="$(git log -1 --format=%s "$commit")"
  prefix="${subject%%:*}"

  case "$prefix" in
    RED)
      if [[ "$STATE" != "READY_FOR_RED" ]]; then
        print_sequence_error "$commit" "$subject" \
          "Invalid sequence: RED starts a new cycle before the current cycle is complete."
      fi
      STATE="READY_FOR_GREEN"
      LAST_STAGE_COMMIT="$commit"
      LAST_STAGE_SUBJECT="$subject"
      ;;
    GREEN)
      if [[ "$STATE" != "READY_FOR_GREEN" ]]; then
        print_sequence_error "$commit" "$subject" "Invalid sequence: GREEN requires an open RED stage."
      fi
      STATE="READY_FOR_REFACTOR"
      LAST_STAGE_COMMIT="$commit"
      LAST_STAGE_SUBJECT="$subject"
      ;;
    REFACTOR)
      if [[ "$STATE" != "READY_FOR_REFACTOR" ]]; then
        print_sequence_error "$commit" "$subject" \
          "Invalid sequence: REFACTOR requires an open GREEN stage."
      fi
      STATE="READY_FOR_RED"
      LAST_STAGE_COMMIT="$commit"
      LAST_STAGE_SUBJECT="$subject"
      COMPLETED_CYCLES=$((COMPLETED_CYCLES + 1))
      ;;
    DOCS|CHORE|BUILD|TEST)
      ;;
    *)
      echo "Invalid commit prefix in $commit: '$subject'" >&2
      echo "Allowed prefixes: RED, GREEN, REFACTOR, DOCS, CHORE, BUILD, TEST" >&2
      exit 1
      ;;
  esac
done

if [[ "$COMPLETED_CYCLES" -eq 0 ]]; then
  echo "Missing required complete Red -> Green -> Refactor cycle in range $RANGE" >&2
  exit 1
fi

if [[ "$STATE" != "READY_FOR_RED" ]]; then
  case "$STATE" in
    READY_FOR_GREEN)
      echo "Incomplete TDD cycle: RED stage was opened but GREEN/REFACTOR were not completed." >&2
      ;;
    READY_FOR_REFACTOR)
      echo "Incomplete TDD cycle: GREEN stage was completed but REFACTOR is missing." >&2
      ;;
    *)
      echo "Incomplete TDD cycle detected." >&2
      ;;
  esac

  if [[ -n "$LAST_STAGE_COMMIT" ]]; then
    echo "Last completed stage commit: $LAST_STAGE_COMMIT ($LAST_STAGE_SUBJECT)" >&2
  fi
  exit 1
fi

echo "TDD sequence validation passed for range $RANGE (completed cycles: $COMPLETED_CYCLES)"
