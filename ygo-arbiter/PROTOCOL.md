# Engine protocol, version 1

This document specifies how the arbiter (`ygo-arbiter`) and an engine communicate.

## Terms

| Term | Meaning |
|---|---|
| arbiter | The program that runs the duel and starts the engines. |
| engine | A program that plays one side of a duel. |
| message | A line from the arbiter to the engine. It always has the member `"type"`. |
| reply | A line from the engine to the arbiter. It has no member `"type"`. |
| player | The side the engine plays in a game: 0 or 1. |
| prompt | A question of the duel to the player, for example "select a card". |
| response | The answer of the engine to a prompt. |
| observation | A duel event that the player can see, for example a card draw. |

Card codes, locations, positions, phases and other numbers have the values of ygopro-core
(`ygo-core/src/constants.rs`).

## Transport

1. The arbiter starts the engine as a child process.
2. The arbiter writes messages to the standard input of the engine.
3. The engine writes replies to its standard output.
4. Each message and each reply is one JSON object on one line, in UTF-8, with a line feed at the end.
5. The engine must flush its standard output after each reply.
6. The engine must not write other lines to its standard output.
7. The engine can write logs to its standard error. The arbiter does not read them.

## Compatibility

Later versions of this protocol can add message types and members. To stay compatible:

1. The engine must ignore a message with an unknown type. It must not reply to it.
2. The engine must ignore unknown members in a message.
3. The arbiter ignores unknown members in a reply.

## Process lifetime

1. Before the match, the arbiter starts each engine once. It reads the identity and checks the
   options. Then it stops the engine.
2. For each game, the arbiter starts a new engine process.
3. At the end of a game, the arbiter sends `quit`. If the engine does not stop within 5 seconds, the
   arbiter kills it.
4. The engine must stop when its standard input closes.

## Sequence

```
arbiter                                  engine
  hello                         ->
                                <-       {"id": ..., "options": [...]}
  setoption (zero or more)      ->
  isready                       ->
                                <-       {"ready": true}
  init                          ->
  prompt                        ->
                                <-       {"response": ...}
  (more prompts)
  end                           ->
  quit                          ->
```

The engine must reply to `hello` and to `isready` within 30 seconds. If it does not, the arbiter
stops with an error. This time is not on the clock of the engine.

## Messages

### hello

```json
{"type": "hello", "protocol": 1}
```

`protocol` is the version of this protocol. If the engine cannot use this version, it must write the
reason to its standard error and stop.

The engine replies with its identity and its options:

```json
{"id": {"name": "my-engine", "author": "A. Author", "version": "2.1"},
 "options": [{"type": "spin", "name": "Hash", "default": 64, "min": 1, "max": 4096}]}
```

| Member | Meaning |
|---|---|
| `id.name` | The name of the engine. Required. |
| other members of `id` | Optional. Their values are strings, for example `author` or `version`. |
| `options` | The options of the engine. Optional. Each option is one object (see the next table). |

| Option type | Other members | Valid values |
|---|---|---|
| `check` | `name`, `default` | `true` or `false` |
| `spin` | `name`, `default`, `min`, `max` | An integer from `min` to `max` |
| `combo` | `name`, `default`, `values` | One of the strings in `values` |
| `string` | `name`, `default` | Any string |

### setoption

```json
{"type": "setoption", "name": "Hash", "value": 256}
```

1. The arbiter sends `setoption` only after `hello` and before `isready`.
2. The arbiter sends `setoption` only for an option that the engine declared.
3. The arbiter sends only valid values. The type of `value` is the JSON type of the option: boolean,
   integer or string.
4. Options that the arbiter does not set keep their default values.
5. The engine does not reply.

To set an option, give the arbiter `--option ENGINE.NAME=VALUE`, for example
`--option new.Hash=256`.

### isready

```json
{"type": "isready"}
```

The engine prepares for play, for example it loads its data. Then it replies:

```json
{"ready": true}
```

### init

The arbiter sends `init` at the start of each game.

```json
{"type": "init", "player": 0,
 "deck": {"name": "Blue-Eyes", "main": [89631139, ...], "extra": [...]},
 "extra_order": [...], "opponent_deck_size": 40, "opponent_extra_size": 15,
 "config": {"flags": 190464, "start_lp": 8000, "start_hand": 5, "draw_per_turn": 1}}
```

| Member | Meaning |
|---|---|
| `player` | The player of the engine: 0 or 1. Player 0 starts the duel. |
| `deck` | The deck list of the engine. The order of `main` is not the order of the cards in the deck. |
| `extra_order` | The card codes of the Extra Deck of the engine, in their order in the duel. |
| `opponent_deck_size` | The number of cards in the Main Deck of the opponent. |
| `opponent_extra_size` | The number of cards in the Extra Deck of the opponent. |
| `config` | The duel settings: the duel flags of ygopro-core, the start Life Points, the number of cards in the start hand, the number of cards to draw each turn. |

The engine does not reply.

### prompt

The arbiter sends `prompt` when the player must make a decision.

```json
{"type": "prompt", "obs": [...], "prompt": {...}, "state": {...},
 "clock": {"remaining": 41.3, "increment": 0.3}, "nodes": 20000}
```

| Member | Meaning |
|---|---|
| `obs` | The observations since the last prompt, in duel order. See [Observations](#observations). |
| `prompt` | The prompt. See [Prompts and responses](#prompts-and-responses). |
| `state` | What the player can see of the duel now. See [State](#state). |
| `clock` | Only with a time control. The seconds that the engine has for this decision (`remaining`) and the seconds that it gets after each decision (`increment`). |
| `nodes` | Only with NPS-adjusted node limits (`--nps-nodes`). The node limit for this decision. An engine without a search can ignore it. |

The engine replies with a response:

```json
{"response": {"kind": "activate", "index": 2},
 "info": {"value": 0.31, "depth": 9, "nodes": 18250, "secs": 1.42}}
```

`info` is optional. It describes the search of the engine for this decision: its value, its depth,
the number of nodes and the seconds. The arbiter writes `info` to the game record. The arbiter uses
`nodes` and `secs` for NPS-adjusted node limits (`--nps-nodes`).

### end

The arbiter sends `end` when the game is over.

```json
{"type": "end", "obs": [...],
 "result": {"winner": 1, "reason": 1, "turns": 7, "decisions": 112, "retries": 0, "script_errors": 0, "time": [12.1, 9.8]}}
```

| Member of `result` | Meaning |
|---|---|
| `winner` | The player that won, or `null` for a draw. |
| `reason` | A number: the win reason of the duel. Or a string: `time_forfeit`, `illegal_response`, `agent_error: ...` (the engine failed), `turn_limit`, `decision_limit`, `retry_limit`, `ended`. |
| `turns`, `decisions`, `retries` | Counts for the game. |
| `script_errors` | The number of card script errors in the duel. |
| `time` | The seconds that each player used. |

The engine does not reply.

### quit

```json
{"type": "quit"}
```

The engine stops. It does not reply.

## Observations

An observation is a message of ygopro-core as a JSON object:

1. The member `type` is the name of the message without the prefix `MSG_`, for example `"DRAW"`.
2. The other members are the fields of the message. `src/messages.rs` (`enum Msg`) lists them.
3. A card place has the members `con` (controller), `loc` (location), `seq` (sequence) and `pos`
   (position).

The arbiter removes what the player must not know:

1. The code of a hidden card is 0. A hidden card is, for example, a card in the hand of the
   opponent or a face-down card on the field of the opponent.
2. The effect description of a hidden card is 0, because it contains the card code.
3. All face-down cards in the Deck, the hand, the Extra Deck and the banished cards have the
   position face-down defense. The original positions show how a card got there.
4. Some messages are not sent to the player, for example hints for the opponent.

The rules are those of the EDOPro server, with fewer leaks. `src/observe.rs` contains them.

The arbiter answers some prompts itself. The player does not see them. These are chain windows in
which the player can only pass.

## Prompts and responses

A prompt is a selection message of ygopro-core for the player: its type starts with `SELECT_`,
`SORT_` or `ANNOUNCE_`, or it is `ROCK_PAPER_SCISSORS`.

The arbiter sorts the option lists of a prompt: by the place of the card, then by the card code,
then by the effect description. Numbers to announce are sorted by value. The order of the core could
show hidden information. An index in a response refers to the
list as the engine received it. A negative index counts from the end of the list.

A response is a JSON object. Its members depend on the prompt type:

| Prompt type | Response members |
|---|---|
| `SELECT_IDLECMD` | `kind`: `summon`, `spsummon`, `repos`, `mset`, `sset`, `activate`, `to_bp`, `to_ep` or `shuffle`. `index`: the card in the list of that kind (`summonable`, `spsummonable`, `repositionable`, `msetable`, `ssetable`, `activatable`). No `index` for `to_bp`, `to_ep` and `shuffle`. |
| `SELECT_BATTLECMD` | `kind`: `activate`, `attack`, `to_m2` or `to_ep`. `index`: the card in `activatable` or `attackable`. |
| `SELECT_EFFECTYN`, `SELECT_YESNO` | `yes`: `true` or `false`. |
| `SELECT_OPTION` | `index`: the option in `options`. |
| `ANNOUNCE_NUMBER` | `index`: the number in `values`. |
| `SELECT_CHAIN` | `index`: the effect in `chains`, or -1 to not chain. |
| `SELECT_CARD`, `SELECT_TRIBUTE`, `SELECT_SUM` | `indices`: the selected cards in `cards`. Or `cancel: true`, if the prompt is cancelable. |
| `SELECT_UNSELECT_CARD` | `index`: a card. Indices from 0 refer to `select_cards`; the indices after them refer to `unselect_cards`. Or `finish: true` to end the selection. |
| `SELECT_PLACE`, `SELECT_DISFIELD` | `places`: a list of `[player, location, sequence]`, one for each zone to select. |
| `SELECT_POSITION` | `position`: one position. |
| `SELECT_COUNTER` | `counts`: the number of counters to remove from each card in `cards`. |
| `SORT_CARD`, `SORT_CHAIN` | `order`: the new position of each card. No `order` keeps the order of the core. |
| `ANNOUNCE_RACE`, `ANNOUNCE_ATTRIB` | `mask`: the selected races or attributes as bits. |
| `ANNOUNCE_CARD` | `code`: the card code. |
| `ROCK_PAPER_SCISSORS` | `hand`: 1, 2 or 3. |

`yes`, `cancel` and `finish` can also be 0 or 1.

If a response does not have the necessary members, or if an index is out of range, the engine loses
the game (`agent_error`). If the duel does not accept the response, the engine loses the game
(`illegal_response`).

## State

`state` is what the player can see of the whole duel at the prompt.

| Member | Meaning |
|---|---|
| `turn`, `turn_player`, `phase` | The turn number, the player of the turn and the phase. |
| `lp` | The Life Points of player 0 and player 1. |
| `chain` | The links of the current chain. |
| `players` | For player 0 and player 1: the cards in `deck`, `hand`, `mzone`, `szone`, `grave`, `removed` and `extra`. |

A card is an object with the query fields of ygopro-core: `code`, `position`, `type`, `level`,
`attack`, `defense` and more (`src/state.rs`, `struct CardQuery`). An empty zone is `null`. For a
hidden card, the object has only `position`, `owner`, `counters`, `overlays`, `equip_card` and
`target_cards`.

## Time

1. Without a time control, the prompt has no `clock`. The engine has no time limit.
2. With a time control, each player has a clock (Fischer). The arbiter measures the time from the
   prompt to the response.
3. The arbiter subtracts this time from the clock. Then it adds the increment.
4. If the clock is below zero, the engine loses the game (`time_forfeit`).
5. If no response comes within the remaining time plus 10 seconds, the engine also loses the game
   (`time_forfeit`).

## Reference engine

`ygo-random-engine` (`src/bin/ygo-random-engine.rs`) is a complete engine. It gives random valid
responses. Its option `Seed` (`spin`) sets its random numbers. The Rust crate gives an engine its
side of the protocol in `ygo_arbiter::engine::serve`.
