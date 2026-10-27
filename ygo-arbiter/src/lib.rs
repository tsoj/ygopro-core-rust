//! Matches between Yu-Gi-Oh engines (AIs as separate processes, like chess engines under
//! fastchess): the engine protocol (PROTOCOL.md) on both sides, what each player may see of a
//! duel, and the records of the games played. The binary `ygo-arbiter` runs the matches.
//!
//! * [`messages`]: parsing of the core's binary messages
//! * [`observe`]: what each player may see of them, prompts in canonical order
//! * [`actions`]: responses to prompts and their binary form
//! * [`state`]: god view snapshots of a duel and each player's view
//! * [`game`]: setting up, playing and replaying games, game records
//! * [`protocol`]: an engine process, for the arbiter; [`engine`]: the engine side
//! * [`matchrun`]: matches: openings, clocks, statistics, the SPRT
//! * [`pyrandom`]: Python's random.Random (the games' random choices, as when they were first
//!   played in Python)
//!
//! Every type serializes (serde) to JSON under the keys the protocol uses.

pub mod actions;
pub mod engine;
pub mod game;
pub mod matchrun;
pub mod messages;
pub mod observe;
pub mod protocol;
pub mod pyrandom;
pub mod state;
