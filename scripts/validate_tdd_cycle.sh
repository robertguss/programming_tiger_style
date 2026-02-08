#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: scripts/validate_tdd_cycle.sh [--base <commit-ish>] [--strict-doc-only]

Validates commit history from <base>..HEAD for Red -> Green -> Refactor sequencing.

Options:
  --base <commit-ish>   Base revision for commit range (default: origin/main or repo root commit)
  --strict-doc-only     Enforce TDD commit prefixes even for docs/contracts-only changes
USAGE
}

BASE_REF=""
STRICT_DOC_ONLY=0

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

RANGE="${BASE_REF}..HEAD"
COMMITS=()
while IFS= read -r commit; do
  COMMITS+=("$commit")
done < <(git rev-list --reverse "$RANGE")

if [[ ${#COMMITS[@]} -eq 0 ]]; then
  echo "No commits in range $RANGE. Nothing to validate."
  exit 0
fi

if [[ "$STRICT_DOC_ONLY" -eq 0 ]]; then
  DOC_ONLY=1
  while IFS= read -r path; do
    case "$path" in
      *.txt|*.rst|LICENSE|README.md|.github/*|contracts/*|templates/*|checklists/*|.evidence/*|*.md)
        ;;
      *)
        DOC_ONLY=0
        break
        ;;
    esac
  done < <(git diff --name-only "$RANGE")

  if [[ "$DOC_ONLY" -eq 1 ]]; then
    echo "Docs/contracts-only change detected. Skipping strict TDD commit prefix check."
    exit 0
  fi
fi

SEEN_RED=0
SEEN_GREEN=0
SEEN_REFACTOR=0

for commit in "${COMMITS[@]}"; do
  # Ignore merge commits (for example, GitHub synthetic PR merge commits) because
  # they are integration artifacts, not authored TDD lifecycle commits.
  parent_line="$(git rev-list --parents -n 1 "$commit")"
  parent_count=$(( $(wc -w <<<"$parent_line") - 1 ))
  if [[ "$parent_count" -gt 1 ]]; then
    continue
  fi

  subject="$(git log -1 --format=%s "$commit")"
  prefix="${subject%%:*}"

  case "$prefix" in
    RED)
      SEEN_RED=1
      ;;
    GREEN)
      if [[ "$SEEN_RED" -eq 0 ]]; then
        echo "Invalid sequence: GREEN before RED in commit $commit ($subject)" >&2
        exit 1
      fi
      SEEN_GREEN=1
      ;;
    REFACTOR)
      if [[ "$SEEN_GREEN" -eq 0 ]]; then
        echo "Invalid sequence: REFACTOR before GREEN in commit $commit ($subject)" >&2
        exit 1
      fi
      SEEN_REFACTOR=1
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

if [[ "$SEEN_RED" -eq 0 || "$SEEN_GREEN" -eq 0 || "$SEEN_REFACTOR" -eq 0 ]]; then
  echo "Missing required TDD sequence in range $RANGE" >&2
  echo "Observed: RED=$SEEN_RED GREEN=$SEEN_GREEN REFACTOR=$SEEN_REFACTOR" >&2
  exit 1
fi

echo "TDD sequence validation passed for range $RANGE"
