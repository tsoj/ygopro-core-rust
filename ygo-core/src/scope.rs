//! The replay scope of a card function that waits for a player (see DESIGN.md).

use std::sync::Arc;

use crate::duel::Duel;
use crate::types::Response;

/// A change made to the duel from outside (an API call, e.g. a forcing of the belief replay) while
/// the scope waits: it is made again at the same question when the scope runs again.
pub type ScopeOp = Arc<dyn Fn(&mut Duel) + Send + Sync>;

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
    /// The same for log messages (`Duel::logs`).
    pub delivered_logs: usize,
    pub skip_logs: usize,
    /// Whether the card function is running (nested calls run inline).
    pub running: bool,
    /// The question the scope waits at (its index among the scope's questions).
    pub waiting_at: usize,
    /// Changes made from outside while the scope waited, with the question they were made at:
    /// made again there when the scope runs again from the snapshot.
    pub ops: Vec<(usize, ScopeOp)>,
}

/// The payload of the unwind that interrupts a card function waiting for a response.
pub struct Interrupt;
