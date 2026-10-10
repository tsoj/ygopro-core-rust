//! The replay scope of a card function that waits for a player (see DESIGN.md).

use crate::duel::Duel;
use crate::types::Response;

#[derive(Clone)]
pub struct Scope {
    /// The duel before the card function started.
    pub snapshot: Box<Duel>,
    /// The responses given inside the scope so far.
    pub log: Vec<Response>,
    /// The next response to take from the log in the current run.
    pub cursor: usize,
    /// Messages already sent in earlier runs of this scope.
    pub delivered_messages: usize,
    /// Messages to drop in the current run (sent before by an earlier run).
    pub skip_messages: usize,
    /// Whether the card function is running (nested calls run inline).
    pub running: bool,
}

/// The payload of the unwind that interrupts a card function waiting for a response.
pub struct Interrupt;
