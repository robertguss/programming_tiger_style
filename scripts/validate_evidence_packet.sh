#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: scripts/validate_evidence_packet.sh [--file <path> | --pr-body <path>]

Validates that required evidence packet sections exist.

Options:
  --file <path>      Path to evidence packet markdown file
  --pr-body <path>   Path to file containing pull request body markdown
USAGE
}

MODE=""
INPUT_PATH=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --file)
      MODE="file"
      INPUT_PATH="${2:-}"
      shift 2
      ;;
    --pr-body)
      MODE="pr"
      INPUT_PATH="${2:-}"
      shift 2
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

if [[ -z "$MODE" ]]; then
  if [[ -f ".evidence/EVIDENCE_PACKET.md" ]]; then
    MODE="file"
    INPUT_PATH=".evidence/EVIDENCE_PACKET.md"
  else
    echo "No input provided and .evidence/EVIDENCE_PACKET.md not found." >&2
    usage
    exit 2
  fi
fi

if [[ -z "$INPUT_PATH" || ! -f "$INPUT_PATH" ]]; then
  echo "Input file not found: $INPUT_PATH" >&2
  exit 2
fi

NORMALIZED_PATH="$(mktemp)"
trap 'rm -f "$NORMALIZED_PATH"' EXIT
tr -d '\r' < "$INPUT_PATH" > "$NORMALIZED_PATH"

REQUIRED_HEADINGS=()
while IFS= read -r heading; do
  REQUIRED_HEADINGS+=("$heading")
done <<'EOF_HEADINGS'
## Objective
## Risk Tier
## Scope
## Red
## Green
## Refactor
## Invariants
## Security Impact
## Performance Impact
## Assumptions
## Open Questions
## Rollback Plan
## Validation Commands
EOF_HEADINGS

missing=0
for heading in "${REQUIRED_HEADINGS[@]}"; do
  if ! grep -qxF "$heading" "$NORMALIZED_PATH"; then
    echo "Missing required heading: $heading" >&2
    missing=1
  fi
done

if [[ "$missing" -ne 0 ]]; then
  echo "Evidence packet validation failed." >&2
  exit 1
fi

if grep -Eqi '<fill|tbd|todo|replace me>' "$NORMALIZED_PATH"; then
  echo "Evidence packet contains unresolved placeholders (e.g., TBD/TODO/<fill>)." >&2
  exit 1
fi

if [[ "$MODE" == "pr" ]]; then
  echo "PR body evidence validation passed."
else
  echo "Evidence packet file validation passed: $INPUT_PATH"
fi
