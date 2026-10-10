#!/bin/sh
# Run ygo-diff on the planned deck pairs, in parallel, with the core games cached:
#   rust/tools/test_pairs.sh [games] [first] [last] [jobs]   (from rust/)
# Full outputs (with coverage) go to ../build/pair_results/pairNNN.txt.
G=${1:-300}; FIRST=${2:-0}; LAST=${3:-99999}; JOBS=${4:-16}
P=../build/deck_pairs; R=../build/pair_results; C=../build/diff-cache
mkdir -p $R
for a in $P/pair*_a.ydk; do
  i=$(basename $a _a.ydk | sed 's/pair//')
  [ "$((10#$i))" -lt "$FIRST" ] && continue
  [ "$((10#$i))" -gt "$LAST" ] && continue
  echo $i
done | xargs -P $JOBS -I{} sh -c "./target/release/ygo-diff --decks $P/pair{}_a.ydk,$P/pair{}_b.ydk --games $G --seed 1 --cache $C > $R/pair{}.txt 2>&1"
for a in $P/pair*_a.ydk; do
  i=$(basename $a _a.ydk | sed 's/pair//')
  [ "$((10#$i))" -lt "$FIRST" ] && continue
  [ "$((10#$i))" -gt "$LAST" ] && continue
  r=$(grep -v "games identical$" $R/pair$i.txt | grep -E "identical|MISMATCH|panicked|without a translated" | head -2 | sed 's/(seed \[[0-9, ]*\])//; s/ (\([0-9]*\) ended, the others reached --max-decisions)//; s/; core.*//' | paste -sd' ')
  echo "pair$i: $r"
done
