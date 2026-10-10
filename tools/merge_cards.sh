#!/bin/sh
# Copy the new card files of an agent worktree into the main checkout (no overwrite).
# usage: rust/tools/merge_cards.sh <agent-id>   (from the repository root)
W=.claude/worktrees/agent-$1/rust/ygo-cards/src/cards
for f in $(cd .claude/worktrees/agent-$1 && git status --porcelain | awk '$1=="??"{print $2}'); do
  case $f in rust/ygo-cards/src/cards/c*.rs) ;; *) echo "UNEXPECTED $f"; continue;; esac
  b=$(basename $f)
  if [ -e rust/ygo-cards/src/cards/$b ]; then echo "EXISTS $b"; else cp $W/$b rust/ygo-cards/src/cards/$b; echo "$b"; fi
done
