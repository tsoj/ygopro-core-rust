# ygo-arbiter

`ygo-arbiter` plays matches between two Yu-Gi-Oh! bots, like fastchess for chess engines. It runs
the duels on the engine of this repository, gives each bot only what its player may see, and reports
W/D/L, Elo and SPRT results.

A bot is a program that speaks the bot protocol: JSON lines on its standard input and output.
`PROTOCOL.md` specifies it. `ygo-random-bot` is a complete bot that gives random valid responses.

## Building

From the root of this repository:

```sh
cargo build --release -p ygo-arbiter
```

This builds `target/release/ygo-arbiter` and `target/release/ygo-random-bot`.

The arbiter needs the card databases of [ProjectIgnis/BabelCDB](https://github.com/ProjectIgnis/BabelCDB).
It reads them from `$YGO_ROOT/external/BabelCDB`. Without `YGO_ROOT`, it uses the folder that
contains this repository.

## Running a match

```sh
YGO_ROOT=/path/to/data ./target/release/ygo-arbiter \
    --bot "new=/path/to/my-bot --my-flag" \
    --bot "base=./target/release/ygo-random-bot" \
    --book book.jsonl -n 100 --tc 10+0.1 -c 8 --records games.jsonl
```

A match goes like this:

1. The arbiter reads the opening book and checks each line it uses (see
   [Opening books](#opening-books)).
2. It starts each bot once. It prints the bot's identity and checks the options you set.
3. It plays each opening twice. The bots change seats, and each seat keeps its deck. So each bot
   plays each deck once, with the same draws.
4. It starts a new bot process for each game.
5. After each pair of games, it prints the score of the first bot:
   `games 40: +20 =0 -20  elo -0.0 +/- 98.9  penta [4, 0, 12, 0, 4]`. `penta` counts the pairs by
   their score, from 0 to 2 in steps of 0.5. With `--sprt`, the line also shows the LLR.

| Option | Meaning |
|---|---|
| `--bot NAME=COMMAND` | A bot: its name, and the command that starts it. Give exactly two. The score is for the first. |
| `--option NAME.OPTION=VALUE` | Set an option that the bot declares (`PROTOCOL.md`), for example `--option new.Hash=256`. |
| `--book FILE` | The opening book. Required. |
| `-n N` | The number of openings (default 100). The match has 2N games. |
| `--shuffle SEED` | Shuffle the openings with this seed first. Without it, the arbiter uses the first openings of the book. |
| `--pool N` | Use only the first N lines of the book. |
| `--book-min N` | Use only openings in which each seat won at least N test games (`wins`; default 2). |
| `--tc BASE+INC` | Time control: BASE seconds, plus INC seconds after each decision. Without it, there is no clock. |
| `--max-turns N` | A game is a draw after N turns (default 100). |
| `--sprt ELO0 ELO1` | Stop when the SPRT accepts H0 (Elo ELO0) or H1 (Elo ELO1). `--alpha` and `--beta` set the error rates (default 0.05). |
| `-c N` | The number of games at the same time (default: the number of CPU threads). |
| `--records FILE` | Write each game to FILE as one JSON line (see [Records](#records)). |
| `--resume FILE` | Continue a stopped match. FILE is its `--records` file; its finished pairs count. |
| `--nps-nodes N` | A node limit for each decision: the second bot searches N nodes, the first N times the ratio of their speeds (nodes per second). |

## Opening books

An opening book is a text file with one opening on each line. An opening is two decks and a seed.
Empty lines are ignored.

```json
{"seed": 4207, "decks": [{"name": "Blue-Eyes", "main": [89631139, 89631139, 89631139, ...], "extra": [...]}, {"name": "Dark Magician", "main": [...], "extra": [...]}], "wins": [6, 4]}
```

| Member | Required | Meaning |
|---|---|---|
| `seed` | yes | An integer. It sets the order of both Main Decks and both Extra Decks, which copy of a card is where, and the random numbers of the duel (coin tosses, dice). Both games of an opening use it, so each seat gets the same draws in both games. |
| `decks` | yes | Exactly two decks: the deck of player 0, then the deck of player 1. Player 0 starts the duel. |
| `wins` | no | Two integers: the wins of player 0 and player 1 in test games. `--book-min` uses them (see below). |

Other members are ignored.

Each deck is an object:

| Member | Required | Meaning |
|---|---|---|
| `main` | yes | The Main Deck: 40 to 60 card codes. |
| `extra` | no | The Extra Deck: at most 15 card codes, only Fusion, Synchro, Xyz and Link Monsters. |
| `name` | no | A name. The records keep it. |

Card codes are the codes of the card databases (the passcodes). The order of the codes in `main` and
`extra` has no effect, because the seed shuffles them. A deck has at most 3 copies of a card; an
alternate artwork counts as its original card. The arbiter does not check a banlist, and there is no
Side Deck.

The arbiter checks each line that it uses when it starts. If a line is not a valid opening, the
arbiter stops with the line number and the problem, for example
`book.jsonl line 12: deck 0: 30 cards in the Main Deck (40 to 60)`.

Which openings a match uses:

1. The arbiter reads the first `--pool` lines (default: all).
2. It removes the openings in which a seat won fewer than `--book-min` test games. An opening without
   `wins` is never removed. With the default `--book-min 2`, an opening with `"wins": [1, 9]` is
   removed; use `--book-min 0` to keep all openings.
3. With `--shuffle SEED`, it shuffles the openings that are left.
4. It plays the first `-n` of them.

A good book has openings that neither seat wins too often: the game result then shows the
difference between the bots, not between the decks or seats. `wins` from test games with two equal
bots can show these openings.

To make a book from `.ydk` files (`#main` and `#extra` sections with one card code on each line):

```python
import json, random

def ydk(path):
    deck, part = {"main": [], "extra": []}, None
    for line in open(path):
        line = line.strip()
        if line in ("#main", "#extra"):
            part = line[1:]
        elif line.startswith(("#", "!")):
            part = None
        elif part and line.isdigit():
            deck[part].append(int(line))
    return deck

pairs = [("a.ydk", "b.ydk"), ("c.ydk", "d.ydk")]
with open("book.jsonl", "w") as f:
    for a, b in pairs:
        f.write(json.dumps({"seed": random.randrange(2**62), "decks": [ydk(a), ydk(b)]}) + "\n")
```

## Records

With `--records FILE`, the arbiter writes one JSON object for each game:

| Member | Meaning |
|---|---|
| `decks`, `seed` | The opening. |
| `core_seed`, `deck_orders`, `config` | The duel's random seed, the order of the Main Decks and the duel settings, as derived from `seed`. |
| `responses` | Each decision: the player and its response. With the members above, they repeat the game exactly. |
| `result` | `winner` (0, 1 or `null` for a draw), `reason`, `turns`, `decisions`, `time` (seconds each player used) and more. `PROTOCOL.md` lists them. |
| `agents` | The names of the bots, by seat. |
| `labels` | The searches the bots reported (`info` in `PROTOCOL.md`), by decision. |

`ygo-diff-replays` converts records into `ygo-diff` replays, to replay the games on the C++ core
(`tools/replay_records.py`).

## Writing a bot

Any program that speaks `PROTOCOL.md` can play. A bot in Rust can use this crate: `bot::serve`
handles the protocol, and the bot implements the trait `game::Bot`. `src/bin/ygo-random-bot.rs` is a
complete example in 30 lines.
