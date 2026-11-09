//! Which cards a player shows to the opponent when a card selection moves them from the hand,
//! the Deck or the Extra Deck to a place that is hidden from the opponent (a search, a card from
//! the hand to the Deck, a face-down Summon or Set from the Deck, ...). (Face-down cards on the
//! field that move to a hidden place stay hidden: the audit cannot test them.)
//!
//! The rule: when the selection has a requirement ("1 Spellcaster monster", "1 \"Labrynth\"
//! card"), the player shows the card, so that the opponent can make sure that it meets the
//! requirement. When the selection is free ("1 card from your hand"), the card stays hidden.
//!
//! The core does not know which selections have a requirement. `ygo-reveal-audit` finds it for
//! the selections in recorded games: it adds every official card to the source and looks if the
//! selection accepts all of them. Its results are in the table `data/reveals.tsv`, per effect
//! and move (`SelKey`):
//!
//! * `reveal`: a requirement, and the card script does not show the card. We show it here: a
//!   `ConfirmCards` to the opponent before the move.
//! * `script`: a requirement, and the card script shows the card itself (most searches).
//! * `hide`: free. The card stays hidden.
//!
//! The core notes the cards of each accepted card selection (`Duel::ext.selected`, with the
//! effect that asked). [`annotate`] reads the messages in order: it matches each move of a hidden
//! card against these notes and adds the confirmations. At each action prompt it drops the notes
//! that are left. Thus the result does not depend on how the process calls split the messages.
//!
//! A selection that is not in the table is shown (as `reveal`), and [`annotate`] reports it. The
//! arbiter then stops, unless the user lets it continue (`set_ignore_unknown`).

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, Ordering};

use ygo_core::constants::*;
use ygo_core::duel::Duel;
use ygo_core::ext::EffectKey;

use crate::messages::{Card, Msg};
use crate::observe::li_visible;

/// The table: one line per audited card and per known selection (see `data/reveals.tsv`).
const TABLE: &str = include_str!("../data/reveals.tsv");

/// A selection's effect and move: `effect` None is a selection by the rules (no effect).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SelKey {
    pub effect: Option<EffectKey>,
    pub from: u32,
    pub to: u32,
    pub cost: bool,
}

impl std::fmt::Display for SelKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let e = self.effect.unwrap_or(EffectKey { card: 0, desc: 0, typ: 0 });
        write!(f, "card {} desc {} type {:#x} from {:#x} to {:#x} {}", e.card, e.desc, e.typ, self.from, self.to, if self.cost { "cost" } else { "effect" })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Decision {
    Reveal,
    Script,
    Hide,
}

impl Decision {
    pub fn name(self) -> &'static str {
        match self {
            Decision::Reveal => "reveal",
            Decision::Script => "script",
            Decision::Hide => "hide",
        }
    }
    fn parse(s: &str) -> Option<Decision> {
        [Decision::Reveal, Decision::Script, Decision::Hide].into_iter().find(|d| d.name() == s)
    }
}

pub struct Table {
    /// the cards of the audited games' decks
    pub cards: BTreeSet<u32>,
    pub selections: BTreeMap<SelKey, Decision>,
}

impl Table {
    /// Parses the table's text (`data/reveals.tsv`).
    pub fn parse(text: &str) -> Result<Table, String> {
        let mut t = Table { cards: BTreeSet::new(), selections: BTreeMap::new() };
        for (i, line) in text.lines().enumerate() {
            let f: Vec<&str> = line.split('\t').collect();
            let bad = || format!("reveals.tsv line {}: {line:?}", i + 1);
            let n = |s: &str| s.parse::<u64>().map_err(|_| bad());
            match f[0] {
                "" | "#" => {}
                s if s.starts_with('#') => {}
                "card" if f.len() == 2 => {
                    t.cards.insert(n(f[1])? as u32);
                }
                "sel" if f.len() == 8 => {
                    let card = n(f[1])? as u32;
                    let effect = (card != 0).then_some(EffectKey { card, desc: n(f[2])?, typ: n(f[3])? as u16 });
                    let cost = match f[6] {
                        "cost" => true,
                        "effect" => false,
                        _ => return Err(bad()),
                    };
                    let key = SelKey { effect, from: n(f[4])? as u32, to: n(f[5])? as u32, cost };
                    t.selections.insert(key, Decision::parse(f[7]).ok_or_else(bad)?);
                }
                _ => return Err(bad()),
            }
        }
        Ok(t)
    }

    /// The table line of a selection.
    pub fn line(key: &SelKey, d: Decision) -> String {
        let e = key.effect.unwrap_or(EffectKey { card: 0, desc: 0, typ: 0 });
        format!("sel\t{}\t{}\t{}\t{}\t{}\t{}\t{}", e.card, e.desc, e.typ, key.from, key.to, if key.cost { "cost" } else { "effect" }, d.name())
    }
}

/// The built-in table.
pub fn table() -> &'static Table {
    static T: OnceLock<Table> = OnceLock::new();
    T.get_or_init(|| Table::parse(TABLE).expect("data/reveals.tsv"))
}

static IGNORE_UNKNOWN: AtomicBool = AtomicBool::new(false);

/// Lets games continue with cards and selections that are not in the table (shown, as `reveal`).
pub fn set_ignore_unknown(on: bool) {
    IGNORE_UNKNOWN.store(on, Ordering::Relaxed);
}

pub fn ignore_unknown() -> bool {
    IGNORE_UNKNOWN.load(Ordering::Relaxed)
}

/// What the error messages tell the user to do.
pub const HELP: &str = "Run the discovery (ygopro-core-rust/ygo-arbiter/README.md, \"Shown cards\"): play games with \
     these decks and write the records, run ygo-reveal-audit on the records and tools/reveal_table.py --merge to add \
     the results to ygo-arbiter/data/reveals.tsv, then build again. Or pass --ignore-unknown-reveals: \
     unknown selections are then shown to the opponent. This can rarely give a player information that \
     the rules do not allow (a card shown although its selection was free).";

/// The cards of `codes` that the table does not know.
pub fn unknown_cards(codes: impl IntoIterator<Item = u32>) -> Vec<u32> {
    let t = table();
    let set: BTreeSet<u32> = codes.into_iter().filter(|c| !t.cards.contains(c)).collect();
    set.into_iter().collect()
}

fn is_action_prompt(m: &Msg) -> bool {
    matches!(m, Msg::SelectIdlecmd { .. } | Msg::SelectBattlecmd { .. } | Msg::SelectChain { .. })
}

/// For each message of `msgs` (god-view messages of the duel's last process calls, in order): the
/// selection of the move of a card that its controller selected, when the move is hidden from the
/// opponent. Removes the matched notes from the duel, and at each action prompt the others.
pub fn selected_moves(duel: &mut Duel, msgs: &[Msg]) -> Vec<Option<SelKey>> {
    let mut keys = vec![None; msgs.len()];
    if duel.ext.selected.is_empty() {
        return keys;
    }
    for (k, m) in msgs.iter().enumerate() {
        if is_action_prompt(m) {
            duel.ext.selected.clear();
        }
        let Msg::Move { code, from, to, reason } = m else { continue };
        let sel = &mut duel.ext.selected;
        let Some(i) = sel.iter().position(|s| s.code == *code && s.con as u32 == from.con && s.loc as u32 == from.loc && from.loc != 0) else {
            continue;
        };
        let s = sel.remove(i);
        let opp = 1 - from.con;
        // (from the hand, Deck or Extra Deck: the places where ygo-reveal-audit can add test
        // cards; a face-down card on the field stays hidden, as before)
        let pile = from.loc & (LOCATION_HAND | LOCATION_DECK | LOCATION_EXTRA) != 0;
        if pile && s.player as u32 == from.con && !li_visible(from, opp) && to.con != opp && !li_visible(to, opp) {
            // (a move to the field keeps the card's stale reason, see observe::filter_raw: no cost)
            let cost = reason & REASON_COST != 0 && to.loc & (LOCATION_MZONE | LOCATION_SZONE) == 0;
            keys[k] = Some(SelKey { effect: s.effect, from: from.loc, to: to.loc, cost });
        }
    }
    keys
}

/// Adds the confirmations of the selected cards whose selection has a requirement (see the module
/// documentation) to the god-view messages `msgs` of the duel's last process calls, and removes
/// the matched notes from the duel (`selected_moves`). Returns the selections that the table does
/// not know.
pub fn annotate(duel: &mut Duel, msgs: &mut Vec<Msg>) -> Vec<SelKey> {
    let mut unknown = Vec::new();
    if duel.ext.selected.is_empty() {
        return unknown;
    }
    let keys = selected_moves(duel, msgs);
    if keys.iter().all(Option::is_none) {
        return unknown;
    }
    let mut out = Vec::with_capacity(msgs.len() + 1);
    for (m, key) in msgs.drain(..).zip(keys) {
        if let (Some(key), Msg::Move { code, from, .. }) = (key, &m) {
            let d = table().selections.get(&key).copied().unwrap_or_else(|| {
                unknown.push(key);
                Decision::Reveal
            });
            if d == Decision::Reveal {
                let card = Card { code: *code, con: from.con, loc: from.loc, seq: from.seq, ..Card::default() };
                out.push(Msg::ConfirmCards { player: 1 - from.con, cards: vec![card] });
            }
        }
        out.push(m);
    }
    *msgs = out;
    unknown
}
