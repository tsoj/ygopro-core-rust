#!/bin/sh
# Run ygo-diff on every planned deck pair: rust/tools/test_pairs.sh [games] [first] [last]  (from rust/)
G=${1:-200}; P=../build/deck_pairs
for a in $P/pair*_a.ydk; do
  i=$(basename $a _a.ydk | sed 's/pair//')
  [ -n "$2" ] && [ "$i" -lt "$2" ] && continue
  [ -n "$3" ] && [ "$i" -gt "$3" ] && continue
  r=$(./target/release/ygo-diff --decks $a,$P/pair${i}_b.ydk --games $G --seed 1 2>&1 | grep -v "games identical$" | grep -E "identical|MISMATCH|panicked|without a translated" | head -2 | paste -sd' ')
  echo "pair$i: $r"
done
