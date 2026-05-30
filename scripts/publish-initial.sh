#!/usr/bin/env bash
# Initial publish of all workspace crates to crates.io.
#
# Usage:
#   export CARGO_REGISTRY_TOKEN="your-token"
#   ./scripts/publish-initial.sh              # uses version from local Cargo.toml
#   ./scripts/publish-initial.sh 0.2.0        # override — use when CI bumped crates.io
#                                             # but the bump commit never landed locally
#
# Re-runnable: already-published versions are skipped automatically.
# Rate-limit aware: waits 660 s after each brand-new crate (crates.io limit).

set -euo pipefail

# ── Colours ───────────────────────────────────────────────────────────────────
BLD='\033[1m'; DIM='\033[2m'; RED='\033[0;31m'
GRN='\033[0;32m'; YLW='\033[0;33m'; CYN='\033[0;36m'; RST='\033[0m'

# ── Config ────────────────────────────────────────────────────────────────────
# Seconds to wait between brand-new crates (crates.io: 1 new crate / 10 min).
NEW_CRATE_COOLDOWN=660
# Seconds to wait between versions of already-known crates.
UPDATE_COOLDOWN=40
# Max publish attempts per crate before giving up.
MAX_ATTEMPTS=6
# Initial retry delay (doubles each attempt: 60 120 240 480 960).
RETRY_BASE=60

# ── Publish order (leaves first, facade last) ─────────────────────────────────
CRATES=(
  graphitepdf-errors
  graphitepdf-primitives
  graphitepdf-utils
  graphitepdf-svg
  graphitepdf-stylesheet
  graphitepdf-font
  graphitepdf-math
  graphitepdf-textkit
  graphitepdf-image
  graphitepdf-kit
  graphitepdf-layout
  graphitepdf-render
  graphitepdf-renderer
  graphitepdf-style
  graphitepdf-document
  graphitepdf
)

# ── Preflight ─────────────────────────────────────────────────────────────────
if [ -z "${CARGO_REGISTRY_TOKEN:-}" ]; then
  echo -e "${RED}error:${RST} CARGO_REGISTRY_TOKEN is not set."
  echo "  export CARGO_REGISTRY_TOKEN=\"your-token\" and re-run."
  exit 1
fi

# Version: use CLI arg if supplied, otherwise read from local Cargo.toml.
if [ -n "${1:-}" ]; then
  VERSION="$1"
  echo -e "  ${YLW}version overridden via argument${RST} (local Cargo.toml may differ)"
else
  VERSION=$(cargo metadata --no-deps --format-version 1 \
    | jq -r '.packages[] | select(.name=="graphitepdf") | .version')
fi

echo ""
echo -e "  ${BLD}graphitepdf${RST}  initial publish"
echo -e "  ${DIM}version: ${RST}${CYN}${VERSION}${RST}"
echo -e "  ${DIM}crates:  ${RST}${#CRATES[@]}"
echo -e "  ${DIM}────────────────────────────────────────${RST}"
echo ""

TOTAL=${#CRATES[@]}
PUBLISHED=0
SKIPPED=0
FAILED=0

# ── Helpers ───────────────────────────────────────────────────────────────────
cio_http() {
  curl -sf -o /dev/null -w "%{http_code}" \
    -H "User-Agent: graphite-pdf-publish-script" \
    "https://crates.io/api/v1/crates/$1/$2" 2>/dev/null || echo "000"
}

cio_version_count() {
  curl -sf \
    -H "User-Agent: graphite-pdf-publish-script" \
    "https://crates.io/api/v1/crates/$1" 2>/dev/null \
    | jq '.versions | length // 0' 2>/dev/null || echo "0"
}

countdown() {
  local secs=$1 label=$2
  while [ $secs -gt 0 ]; do
    local mins=$(( secs / 60 ))
    local s=$(( secs % 60 ))
    printf "\r  ${DIM}%s — %02d:%02d remaining${RST}  " "$label" "$mins" "$s"
    sleep 1
    secs=$(( secs - 1 ))
  done
  printf "\r%-60s\r" " "   # clear the line
}

# ── Main loop ─────────────────────────────────────────────────────────────────
IDX=0
for crate in "${CRATES[@]}"; do
  IDX=$(( IDX + 1 ))
  PREFIX="${DIM}[${IDX}/${TOTAL}]${RST}"

  printf "  %s ${BLD}%-30s${RST}" "$PREFIX" "$crate"

  # ── Already published? ──────────────────────────────────────────────────────
  HTTP=$(cio_http "$crate" "$VERSION")
  if [ "$HTTP" = "200" ]; then
    echo -e " ${GRN}✓${RST} ${DIM}already at $VERSION — skipped${RST}"
    SKIPPED=$(( SKIPPED + 1 ))
    continue
  fi

  # ── Attempt publish with retry ──────────────────────────────────────────────
  IS_NEW=$(cio_version_count "$crate")
  echo ""
  SUCCESS=false
  delay=$RETRY_BASE

  for attempt in $(seq 1 $MAX_ATTEMPTS); do
    printf "    ${DIM}attempt %d/%d${RST} … " "$attempt" "$MAX_ATTEMPTS"

    output=$(cargo publish --package "$crate" --no-verify --allow-dirty 2>&1) && {
      echo -e "${GRN}✓ published${RST}"
      SUCCESS=true
      break
    }

    # Parse error
    if echo "$output" | grep -q "429\|Too Many Requests"; then
      # Try to extract the retry-after timestamp from cargo output
      retry_after=$(echo "$output" | grep -oE 'after [^(]+' | head -1 || true)
      echo -e "${YLW}rate limited${RST} ${DIM}(${retry_after:-try again later})${RST}"
    elif echo "$output" | grep -q "already exists"; then
      echo -e "${GRN}✓ already exists${RST}"
      SUCCESS=true
      break
    else
      echo -e "${RED}error${RST}"
      echo ""
      echo "$output" | sed 's/^/    /'
      echo ""
    fi

    if [ $attempt -lt $MAX_ATTEMPTS ]; then
      countdown $delay "retry in"
      delay=$(( delay * 2 ))
    fi
  done

  if [ "$SUCCESS" = false ]; then
    echo -e "  ${RED}✗ ${BLD}$crate failed after $MAX_ATTEMPTS attempts — stopping.${RST}"
    echo ""
    echo -e "  ${DIM}Re-run the script to resume (already-published crates are skipped).${RST}"
    FAILED=$(( FAILED + 1 ))
    break
  fi

  PUBLISHED=$(( PUBLISHED + 1 ))

  # ── Rate-limit cooldown before next crate ───────────────────────────────────
  if [ $IDX -lt $TOTAL ]; then
    if [ "$IS_NEW" -le 1 ]; then
      echo -e "    ${DIM}new crate → waiting ${NEW_CRATE_COOLDOWN}s (crates.io rate limit)${RST}"
      countdown $NEW_CRATE_COOLDOWN "cooldown"
    else
      countdown $UPDATE_COOLDOWN "index propagation"
    fi
  fi
done

# ── Summary ───────────────────────────────────────────────────────────────────
echo ""
echo -e "  ${DIM}────────────────────────────────────────${RST}"
echo -e "  ${GRN}published${RST}  $PUBLISHED"
echo -e "  ${DIM}skipped${RST}    $SKIPPED  ${DIM}(already on crates.io)${RST}"
[ $FAILED -gt 0 ] && echo -e "  ${RED}failed${RST}     $FAILED"
echo ""

if [ $FAILED -eq 0 ]; then
  echo -e "  ${GRN}${BLD}All crates published.${RST}"
  echo -e "  ${DIM}Future releases are handled automatically by the CI release workflow.${RST}"
else
  echo -e "  ${YLW}Re-run this script to resume from where it stopped.${RST}"
  exit 1
fi
echo ""
