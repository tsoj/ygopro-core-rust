# Regression test

The engine plays a fixed set of games and compares each game with the result it recorded before.
The reference is the engine itself, not the C++ core. When a game changes, the rules decide which
result is right. The C++ core (ygo-diff without `--rust-only`) can help to find the cause.

```sh
cargo build --release -p ygo-diff
tools/regress.py --quick          # every 20th job and all cases (~2 min)
tools/regress.py                  # all jobs (~180k games, ~35 min on 28 threads)
tools/regress.py --filter '^case_'
tools/regress.py --bless          # record the digests of the jobs run as the new reference
```

`tools/regress.py` needs Python 3.14 (it uses `compression.zstd`) and `$YGO_ROOT` with the card
data (default: the parent project, see `README.md`).

## Files

| File | Contents |
| --- | --- |
| `suite.txt` | The jobs: one per line, a name and the arguments for `ygo-diff --rust-only --digest`. |
| `expected.txt.zst` | Per job, per game: the low 32 bits of the game's digest and its number of decisions. |
| `data.tar.zst` | The decks and replays that the jobs use (unpacked to `target/regress/data`). |

A game's digest is a hash of all its messages, the statuses of the duel and the queried states at
each decision (`ygo-diff --digest`). A game that changes anywhere changes its digest.

## The jobs

- `case_*`: the games of the bugs found so far, one job per bug (the comment above each names
  it). A new bug fix adds its game here.
- `r50_*`, `r100_*`, `r80long_*`: random decks with 50%, 100% and 80% scripted cards (the last
  with long games), random decisions.
- `human_*`: 1000 pairs of human decks.
- `cov1_*`, `hcov_*`, `hcov2_*`: decks with cards that the other jobs rarely play (random decks of
  these cards, and human decks that contain them), with `--focus` on these cards.
- `rep_*`, `sprt_*`: 10,000 recorded bot games (matches and SPRT runs), chosen so that each card
  that the bots played is in at least 3 of them.

Together the jobs play 13,633 of the 13,671 scripted cards at least 3 times. The cards that are
not played are prerelease cards without database entries and cards that need a specific setup
(for example Holactie the Creator of Light).

## When a job differs

`tools/regress.py` prints the first changed game and the command that plays it alone, for
example `ygo-diff --rust-only -v --from 357 --games 358 --seed 2018 --scripted 1.0`. Run it from
`target/regress/data` for jobs with decks or replays. When the change is correct, record it with
`--bless` (with `--filter` for only the affected jobs) and commit `expected.txt.zst`.
