#!/bin/sh
# Replay a coverage mismatch (build/coverage_fails/<file>) with the Rust engine of this checkout:
# builds ygo-diff with only the cards of the two decks (YGO_CARDS_ONLY: a small, fast build) and
# plays the failing game. Run from rust/.
#   tools/repro.sh <mismatch file> [extra ygo-diff args, e.g. -v]
set -e
f=$1
shift
M=${YGO_ROOT:-$(cd "$(dirname "$0")/../.." && pwd)}
line=$(head -1 "$f")
decks=$(echo "$line" | sed -E 's/.*--decks ([^ ]+).*/\1/')
seed=$(echo "$line" | sed -E 's/.*--seed ([0-9]+).*/\1/')
codes=$(python3 "$(dirname "$0")/deck_codes.py" $(echo "$decks" | tr ',' ' '))
YGO_CARDS_ONLY="$codes" cargo build --release -p ygo-diff 2>&1 | grep -E "^(error|warning)" -A5 | head -40
YGO_ROOT=$M ./target/release/ygo-diff --decks "$decks" --games 1 --seed "$seed" "$@" 2>&1 | grep -v "^  [0-9]*: [0-9]* activations"
