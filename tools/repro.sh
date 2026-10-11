#!/bin/sh
# Replay a coverage mismatch (build/coverage_fails/<file>, or a recorded game's build/replay_fails/<file>) with the Rust engine of this checkout:
# builds ygo-diff with only the cards of the two decks (YGO_CARDS_ONLY: a small, fast build, in
# target/repro so the full target/release binary stays) and
# plays the failing game. Run from rust/.
#   tools/repro.sh <mismatch file> [extra ygo-diff args, e.g. -v]
set -e
f=$1
shift
M=${YGO_ROOT:-$(cd "$(dirname "$0")/../.." && pwd)}
line=$(head -1 "$f")
case "$line" in
*--replay*)
	# a recorded game (build/replay_fails): its cards are the `card p loc code` lines
	replay=$(echo "$line" | sed -E 's/.*--replay ([^ ]+).*/\1/')
	codes=$(python3 "$(dirname "$0")/deck_codes.py" --replay "$replay")
	YGO_CARDS_ONLY="$codes" CARGO_TARGET_DIR=target/repro cargo build --release -p ygo-diff 2>&1 | grep -E "^(error|warning)" -A5 | head -40
	YGO_ROOT=$M ./target/repro/release/ygo-diff --replay "$replay" --max-decisions 100000 "$@" 2>&1 | grep -v "^  [0-9]*: [0-9]* activations"
	;;
*)
	decks=$(echo "$line" | sed -E 's/.*--decks ([^ ]+).*/\1/')
	seed=$(echo "$line" | sed -E 's/.*--seed ([0-9]+).*/\1/')
	codes=$(python3 "$(dirname "$0")/deck_codes.py" $(echo "$decks" | tr ',' ' '))
	YGO_CARDS_ONLY="$codes" CARGO_TARGET_DIR=target/repro cargo build --release -p ygo-diff 2>&1 | grep -E "^(error|warning)" -A5 | head -40
	YGO_ROOT=$M ./target/repro/release/ygo-diff --decks "$decks" --games 1 --seed "$seed" "$@" 2>&1 | grep -v "^  [0-9]*: [0-9]* activations"
	;;
esac
