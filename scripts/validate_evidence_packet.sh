#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: scripts/validate_evidence_packet.sh [--file <path> | --pr-body <path>]

Validates that required evidence packet sections and semantic fields exist.

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

errors=0

for heading in "${REQUIRED_HEADINGS[@]}"; do
  if ! grep -qxF "$heading" "$NORMALIZED_PATH"; then
    echo "Missing required heading: $heading" >&2
    errors=1
  fi
done

if grep -Eqi '<fill|tbd|todo|replace me>' "$NORMALIZED_PATH"; then
  echo "Evidence packet contains unresolved placeholders (e.g., TBD/TODO/<fill>)." >&2
  errors=1
fi

trim_value() {
  local value="$1"
  value="${value#${value%%[![:space:]]*}}"
  value="${value%${value##*[![:space:]]}}"
  value="${value#\`}"
  value="${value%\`}"
  echo "$value"
}

get_section_body() {
  local heading="$1"
  awk -v heading="$heading" '
    $0 == heading {in_section = 1; next}
    in_section && /^## / {exit}
    in_section {print}
  ' "$NORMALIZED_PATH"
}

contains_placeholder_value() {
  local value
  value="$(tr '[:upper:]' '[:lower:]' <<<"$1")"
  [[ "$value" =~ ^(tbd|todo|replace[[:space:]]+me|<fill.*>|none|n/a|na)$ ]]
}

require_field() {
  local section_name="$1"
  local section_body="$2"
  local field_regex="$3"
  local field_display="$4"

  local line
  line="$(grep -Eim1 "^[[:space:]]*[-*][[:space:]]*${field_regex}[[:space:]]*:" <<<"$section_body" || true)"

  if [[ -z "$line" ]]; then
    echo "$section_name section must include $field_display." >&2
    errors=1
    return
  fi

  local value
  value="$(trim_value "${line#*:}")"
  if [[ -z "$value" ]] || contains_placeholder_value "$value"; then
    echo "$section_name section must include $field_display." >&2
    errors=1
  fi
}

validate_section_fields() {
  local section_name="$1"
  local section_body="$2"
  shift 2

  local spec
  for spec in "$@"; do
    local field_regex="${spec%%|*}"
    local field_display="${spec##*|}"
    require_field "$section_name" "$section_body" "$field_regex" "$field_display"
  done
}

RISK_SECTION="$(get_section_body "## Risk Tier")"
RED_SECTION="$(get_section_body "## Red")"
GREEN_SECTION="$(get_section_body "## Green")"
REFACTOR_SECTION="$(get_section_body "## Refactor")"

if [[ -n "$RISK_SECTION" ]]; then
  validate_section_fields "Risk Tier" "$RISK_SECTION" \
    "Tier|Tier" \
    "Rationale|Rationale"

  tier_line="$(grep -Eim1 '^[[:space:]]*[-*][[:space:]]*Tier[[:space:]]*:' <<<"$RISK_SECTION" || true)"
  if [[ -n "$tier_line" ]]; then
    tier_value="$(trim_value "${tier_line#*:}")"
    if [[ -n "$tier_value" ]]; then
      tier_clean="$(tr -d '[:space:]' <<<"$tier_value")"
      tier_clean="${tier_clean//\`/}"
      if ! [[ "$tier_clean" =~ ^[0-3]$ ]]; then
        echo "Risk Tier section must include Tier as one of 0, 1, 2, or 3." >&2
        errors=1
      fi
    fi
  fi
fi

if [[ -n "$RED_SECTION" ]]; then
  validate_section_fields "Red" "$RED_SECTION" \
    "Failing[[:space:]]+test\\(s\\)|Failing test(s)" \
    "Command\\(s\\)|Command(s)" \
    "Failure[[:space:]]+summary|Failure summary" \
    "Expected[[:space:]]+failure[[:space:]]+rationale|Expected failure rationale"
fi

if [[ -n "$GREEN_SECTION" ]]; then
  validate_section_fields "Green" "$GREEN_SECTION" \
    "Command\\(s\\)|Command(s)" \
    "Passing[[:space:]]+summary|Passing summary"
fi

if [[ -n "$REFACTOR_SECTION" ]]; then
  validate_section_fields "Refactor" "$REFACTOR_SECTION" \
    "Why[[:space:]]+behavior[[:space:]]+is[[:space:]]+unchanged|Why behavior is unchanged" \
    "Confirmation[[:space:]]+commands|Confirmation commands"
fi

if [[ "$errors" -ne 0 ]]; then
  echo "Evidence packet validation failed. Resolve missing headings/fields listed above." >&2
  exit 1
fi

if [[ "$MODE" == "pr" ]]; then
  echo "PR body evidence validation passed."
else
  echo "Evidence packet file validation passed: $INPUT_PATH"
fi
