//! Audit of reveals: which moves of hidden cards come from a selection with a requirement.
//!
//! Rule: a player who moves a card between places hidden from the opponent (e.g. a search from
//! the Deck to the hand, a cost from the hand to the Deck) reveals the card only when the
//! selection had a requirement ("add 1 Spellcaster monster"); a free selection ("add 1 card")
//! stays hidden. The core does not report a requirement. This tool finds it: it replays recorded
//! games, and at each such move it goes back to the action that led to the selection (the
//! activation, or the chain window before the resolution: scripts collect their candidates after
//! it), adds every
//! card of the card database that can be there to the selection's source (the hand or the Deck),
//! plays the same responses again (by meaning) up to the selection, and counts the added cards among its candidates.
//! All added cards selectable: no requirement; some not: a requirement.
//!
//! A choice among the top cards of the Deck ("look at the top 3 cards, add 1"; "deck_top") is
//! checked with test cards on top of the Deck, a sample of them, as many as the candidates at a
//! time. Test cards that the rules alone keep from the destination (a card that cannot be Set, a
//! monster that the effect cannot Special Summon) do not count as rejected ("excluded").
//!
//! One JSON line per game with its decks' cards ("deck_cards"), and one per move: the game, the
//! move, the selecting player, the arbiter's key of the selection (`reveal::SelKey`), the
//! verdict ("requirement", "free", "no_selection", "skipped" or "undecided" with the reason),
//! the added cards accepted, the time of the check, whether the card's script confirms the card to
//! the opponent and whether the arbiter's filter shows it. tools/reveal_table.py makes the
//! arbiter's table (ygo-arbiter/data/reveals.tsv) from these lines.
//!
//!   ygo-reveal-audit RECORDS.jsonl [--games N] [--threads T] [--codes N] [--discover] > OUT.jsonl
//!
//! --discover: check only the selections that the built-in table does not know, each at most a
//! few times ("skipped" for the others).
//!
//! --codes N: only every k-th test card (about N cards), for a quick run. A move from the Extra
//! Deck is checked with the Extra Deck cards. "script_confirms": the card's code was confirmed to
//! the opponent since the action started (the step's messages when there is no selection).

use std::io::{BufRead, Write};
use std::collections::BTreeMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::Instant;

use serde_json::{Value, json};
use ygo_arbiter::actions::{Response, encode};
use ygo_arbiter::game::{GameRecord, card_db, run_until_prompt, run_until_prompt_raw};
use ygo_arbiter::reveal;
use ygo_core::ext::EffectKey;
use ygo_arbiter::messages::{Card, Loc, Msg};
use ygo_arbiter::observe::{filter_message, li_visible, to_core_response};
use ygo_core::constants::*;
use ygo_core::duel::Duel;

/// How many prompts back the selection of a moved card may be.
const MAX_LOOKBACK: usize = 8;
/// How many prompts the check answers (the selections in between and extra prompts of the test
/// cards) before it gives up.
const MAX_CHECK_PROMPTS: usize = 64;
/// The test cards added in one check.
const CHUNK: usize = 1000;
/// The prompts kept per game (each with a copy of the duel).
const HISTORY: usize = 48;
/// A row lists the rejected test cards when there are at most this many.
const MAX_REJECTED_LISTED: usize = 64;
/// --discover: check only the selections that the built-in table does not know (`reveal`), and
/// each at most this many times (decided rows, in this run).
static DISCOVER: AtomicBool = AtomicBool::new(false);
const DISCOVER_ROWS: usize = 3;
static DECIDED: Mutex<BTreeMap<reveal::SelKey, usize>> = Mutex::new(BTreeMap::new());

fn decided(k: &reveal::SelKey) -> usize {
    DECIDED.lock().unwrap().get(k).copied().unwrap_or(0)
}

/// The test cards of a choice among the top cards of the Deck (a sample of all).
const TOP_SAMPLE: usize = 120;

/// One prompt of the game: the duel before the response, the prompt, the response in core order.
struct Past {
    duel: Duel,
    /// (the game's messages before this prompt)
    msg_at: usize,
    prompt: Msg,
    core: Response,
    raw: Vec<u8>,
}

/// The cards of the official card pool, one per name: the Main Deck cards (`extra` false) or
/// the Extra Deck cards (`extra` true).
fn test_codes(extra: bool) -> Vec<u32> {
    let db = card_db();
    let mut by_name = std::collections::BTreeMap::new();
    for &code in &db.order {
        let Some(c) = db.cards.get(&code) else { continue };
        let ot = db.ots.get(&code).copied().unwrap_or(0);
        let extra_type = c.type_ & (TYPE_FUSION | TYPE_SYNCHRO | TYPE_XYZ | TYPE_LINK) != 0;
        if ot & 0x3 == 0 || c.type_ & TYPE_TOKEN != 0 || extra_type != extra {
            continue;
        }
        if c.alias != 0 && c.alias < code + 10 && code < c.alias + 10 {
            continue; // an alternative artwork
        }
        let name = db.names.get(&code).cloned().unwrap_or_default();
        by_name.entry(name).or_insert(code);
    }
    let mut codes: Vec<u32> = by_name.into_values().collect();
    codes.sort();
    codes
}

fn same_card(c: &Card, code: u32, l: &Loc) -> bool {
    c.code == code && c.con == l.con && c.loc == l.loc && c.seq == l.seq
}

/// The cards a card selection chose (core-order response), with the selection's candidates.
fn chosen(p: &Msg, core: &Response) -> Option<(Vec<Card>, Vec<Card>)> {
    let pick = |cards: &[Card]| -> Vec<Card> {
        core.indices.iter().flatten().filter_map(|&i| cards.get(i as usize).cloned()).collect()
    };
    Some(match p {
        Msg::SelectCard { cards, .. } | Msg::SelectTribute { cards, .. } => (pick(cards), cards.clone()),
        Msg::SelectSum { cards, must_cards, .. } => {
            let mut all = must_cards.clone();
            all.extend(cards.iter().cloned());
            (pick(&all), all)
        }
        Msg::SelectUnselectCard { select_cards, unselect_cards, .. } => {
            let mut all = select_cards.clone();
            all.extend(unselect_cards.iter().cloned());
            let one = core.index.and_then(|i| all.get(i as usize).cloned());
            (one.into_iter().collect(), select_cards.clone())
        }
        _ => return None,
    })
}

fn is_card_select(p: &Msg) -> bool {
    matches!(p, Msg::SelectCard { .. } | Msg::SelectTribute { .. } | Msg::SelectSum { .. } | Msg::SelectUnselectCard { .. })
}

/// The card an idle, battle or chain prompt's response activates (its code), if any.
fn activated(p: &Msg, core: &Response) -> Option<u32> {
    match p {
        Msg::SelectChain { chains, .. } => core.index.filter(|&i| i >= 0).and_then(|i| chains.get(i as usize)).map(|c| c.code),
        Msg::SelectIdlecmd { .. } | Msg::SelectBattlecmd { .. } => {
            let kind = core.kind?;
            kind.cards(p)?.get(core.index? as usize).map(|c| c.code)
        }
        _ => None,
    }
}

/// A card's identity in the check: in the hand, Deck and Extra Deck without the position (the
/// test cards there shift the positions).
fn key(c: &Card) -> (u32, u32, u32, u32) {
    let pile = c.loc & (LOCATION_HAND | LOCATION_DECK | LOCATION_EXTRA) != 0;
    (c.code, c.con, c.loc, if pile { 0 } else { c.seq })
}

fn same(a: &Card, b: &Card) -> bool {
    key(a) == key(b) && a.desc == b.desc
}

/// Whether every card of `a` is in `b` (as a multiset by `key`).
fn all_in(a: &[Card], b: &[Card]) -> bool {
    let mut used = vec![false; b.len()];
    a.iter().all(|c| match (0..b.len()).find(|&j| !used[j] && key(&b[j]) == key(c)) {
        Some(j) => {
            used[j] = true;
            true
        }
        None => false,
    })
}

/// The original's response in a prompt of the check, by meaning: chosen cards and activated
/// effects by identity (the test cards can add candidates and options), other answers as given.
fn same_choice(orig: &Past, now: &Msg) -> Result<Response, String> {
    let mut r = orig.core.clone();
    match (&orig.prompt, now) {
        (Msg::SelectChain { chains: a, .. }, Msg::SelectChain { chains: b, .. }) => {
            if let Some(i) = orig.core.index.filter(|&i| i >= 0) {
                let c = &a[i as usize];
                r.index = Some(b.iter().position(|x| same(x, c)).ok_or("the chained effect is not an option in the check")? as i64);
            }
        }
        (Msg::SelectIdlecmd { .. }, Msg::SelectIdlecmd { .. }) | (Msg::SelectBattlecmd { .. }, Msg::SelectBattlecmd { .. }) => {
            if let (Some(kind), Some(i)) = (orig.core.kind, orig.core.index) {
                if let Some(a) = kind.cards(&orig.prompt) {
                    let c = &a[i as usize];
                    let b = kind.cards(now).ok_or("no such command in the check")?;
                    r.index = Some(b.iter().position(|x| same(x, c)).ok_or("the command's card is not an option in the check")? as i64);
                }
            }
        }
        _ if is_card_select(now) => {
            let (picked, _) = chosen(&orig.prompt, &orig.core).ok_or("not a card selection")?;
            let list: Vec<Card> = match now {
                Msg::SelectSum { cards, must_cards, .. } => must_cards.iter().chain(cards.iter()).cloned().collect(),
                Msg::SelectUnselectCard { select_cards, unselect_cards, .. } => {
                    select_cards.iter().chain(unselect_cards.iter()).cloned().collect()
                }
                Msg::SelectCard { cards, .. } | Msg::SelectTribute { cards, .. } => cards.clone(),
                _ => unreachable!(),
            };
            let mut idx: Vec<i64> = Vec::new();
            for c in &picked {
                let i = (0..list.len())
                    .find(|&j| key(&list[j]) == key(c) && !idx.contains(&(j as i64)))
                    .ok_or("a chosen card is not a candidate in the check")?;
                idx.push(i as i64);
            }
            if matches!(now, Msg::SelectUnselectCard { .. }) {
                if orig.core.finish != Some(true) {
                    r.index = idx.first().copied();
                }
            } else if orig.core.cancel != Some(true) {
                r.indices = Some(idx);
            }
        }
        _ => {}
    }
    Ok(r)
}

/// Whether `m` confirms this card to the opponent: its code in the place it moves from or to
/// (not another copy elsewhere).
fn reveals(m: &Msg, opp: u32, code: u32, from: &Loc, to: &Loc) -> bool {
    let Msg::ConfirmCards { player, cards } = m else { return false };
    *player == opp
        && cards.iter().any(|c| c.code == code && ((c.con, c.loc) == (from.con, from.loc) || (c.con, c.loc) == (to.con, to.loc)))
}

/// A prompt that starts an action: an activation (or a pass) in an idle, battle or chain prompt.
/// Scripts collect their candidates after it (at the activation for costs, at the resolution
/// for effects), so the check starts there.
fn is_action(p: &Msg) -> bool {
    matches!(p, Msg::SelectIdlecmd { .. } | Msg::SelectBattlecmd { .. } | Msg::SelectChain { .. })
}

/// The candidates of a card selection in one place (player, location).
fn candidates_in(p: &Msg, con: u32, loc: u32) -> usize {
    let cards: Vec<&Card> = match p {
        Msg::SelectCard { cards, .. } | Msg::SelectTribute { cards, .. } => cards.iter().collect(),
        Msg::SelectSum { cards, must_cards, .. } => must_cards.iter().chain(cards.iter()).collect(),
        Msg::SelectUnselectCard { select_cards, .. } => select_cards.iter().collect(),
        _ => vec![],
    };
    cards.iter().filter(|c| c.con == con && c.loc == loc).count()
}

/// A check's result: the accepted test cards, those that the rules do not allow at the
/// destination anyway (`rule_excluded`), and (some of) the others, which the selection rejects.
#[derive(Default)]
struct Checked {
    accepted: usize,
    excluded: usize,
    rejected: Vec<u32>,
}

/// Whether the rules alone keep the test card from the move's destination: a Set Spell/Trap
/// that cannot be Set (e.g. Spirit Message "I"), a monster that this effect cannot Special
/// Summon or that cannot be Set. Such a limit is no requirement of the selection: the player
/// does not have to show that the card meets it (like a Set from the hand by the rules).
fn rule_excluded(d: &mut Duel, id: ygo_core::types::CardId, con: u32, to: &Loc, reason: u32) -> bool {
    use ygo_core::api::card;
    match to.loc {
        LOCATION_SZONE => !card::is_s_setable(d, id, true, Some(con as u8)),
        LOCATION_MZONE if reason & REASON_SPSUMMON != 0 => match d.core.reason_effect {
            Some(e) => !card::is_can_be_special_summoned(d, id, e, 0, con as u8, false, false, Some(to.pos as u8), None, None),
            None => false,
        },
        LOCATION_MZONE => !card::is_m_setable(d, id, true, None, 0, None),
        _ => false,
    }
}

/// The codes of a card selection's candidates in one place (player, location).
fn candidate_codes(p: &Msg, con: u32, loc: u32) -> Vec<u32> {
    let cards: Vec<&Card> = match p {
        Msg::SelectCard { cards, .. } | Msg::SelectTribute { cards, .. } => cards.iter().collect(),
        Msg::SelectSum { cards, must_cards, .. } => must_cards.iter().chain(cards.iter()).collect(),
        Msg::SelectUnselectCard { select_cards, .. } => select_cards.iter().collect(),
        _ => vec![],
    };
    cards.iter().filter(|c| c.con == con && c.loc == loc).map(|c| c.code).collect()
}

/// The check: from `hist[base]`, with the test cards in (con, loc), up to the selection
/// `hist[sel]` (the selections `hist[base+1..sel]` answered as in the game). Returns the
/// accepted test cards.
fn check(hist: &[Past], base: usize, sel: usize, con: u32, loc: u32, to: &Loc, reason: u32, codes: &[u32], top: bool) -> Result<Checked, String> {
    let mut d = hist[base].duel.clone();
    // (at the bottom of the Deck: effects that take or look at its top cards stay the same; `top`:
    // on top, for a choice among the top cards)
    let seq = if loc == LOCATION_DECK && !top { 1 } else { 0 };
    let mut added = Vec::with_capacity(codes.len());
    for &code in codes {
        let id = ygo_core::api::debug::add_card(&mut d, code, con as u8, con as u8, loc, seq, POS_FACEDOWN_DEFENSE as u8, false)
            .ok_or("cannot add a test card")?;
        added.push((code, id));
    }
    d.ocg_set_response(&hist[base].raw);
    let mut prev = hist[base].prompt.clone();
    let mut next = base + 1;
    for _ in 0..MAX_CHECK_PROMPTS {
        let step = run_until_prompt(&mut d, Some(&prev)).map_err(|e| e.0)?;
        let Some(p) = step.prompt else { return Err("the duel ended".into()) };
        if step.ended {
            return Err("the duel ended".into());
        }
        let orig = &hist[next];
        let resp = if std::mem::discriminant(&p) == std::mem::discriminant(&orig.prompt) && p.player() == orig.prompt.player() {
            if next == sel {
                let (_, before) = chosen(&orig.prompt, &orig.core).ok_or("not a card selection")?;
                let (_, after) = chosen(&p, &orig.core).ok_or("not a card selection")?;
                // (on top, the test cards take the original candidates' places: count them)
                let acc = if top {
                    let cand = candidate_codes(&p, con, loc);
                    codes.iter().filter(|c| cand.contains(c)).count()
                } else {
                    if !all_in(&before, &after) {
                        return Err("another selection: an original candidate is missing".into());
                    }
                    let n0 = candidates_in(&orig.prompt, con, loc);
                    let n1 = candidates_in(&p, con, loc);
                    n1.checked_sub(n0).ok_or_else(|| "fewer candidates with the test cards".to_string())?
                };
                // (the test cards that are not candidates, by code: few when the requirement is
                // only what the rules allow anyway)
                let mut out = Checked { accepted: acc, ..Default::default() };
                if acc < codes.len() {
                    let mut cand: Vec<u32> = candidate_codes(&p, con, loc);
                    for c in candidate_codes(&orig.prompt, con, loc) {
                        if let Some(i) = cand.iter().position(|&x| x == c) {
                            cand.swap_remove(i);
                        }
                    }
                    for &(code, id) in &added {
                        if cand.contains(&code) {
                            continue;
                        }
                        if rule_excluded(&mut d, id, con, to, reason) {
                            out.excluded += 1;
                        } else if out.rejected.len() < MAX_REJECTED_LISTED {
                            out.rejected.push(code);
                        }
                    }
                }
                return Ok(out);
            }
            next += 1;
            same_choice(orig, &p)?
        } else {
            // a prompt of the test cards (e.g. a hand trap's chain window): decline it
            match &p {
                Msg::SelectChain { forced: 0, .. } => Response { index: Some(-1), ..Default::default() },
                Msg::SelectEffectyn { .. } | Msg::SelectYesno { .. } => Response { yes: Some(false), ..Default::default() },
                _ => return Err(format!("extra prompt {} (expected {})", p.name(), orig.prompt.name())),
            }
        };
        let raw = encode(&p, &resp).map_err(|e| e.0)?;
        d.ocg_set_response(&raw);
        prev = p;
    }
    Err("too many prompts".into())
}

/// Whether the selection's candidates in the player's Deck are exactly its top cards (a choice
/// among the top N, by position, not by a requirement).
fn deck_top_only(hist: &Past, con: u32) -> bool {
    let Some((_, all)) = chosen(&hist.prompt, &hist.core) else { return false };
    let seqs: Vec<u32> = all.iter().filter(|c| c.con == con && c.loc == LOCATION_DECK).map(|c| c.seq).collect();
    let size = hist.duel.ext_get_location_cards(con as u8, LOCATION_DECK).len() as u32;
    let k = seqs.len() as u32;
    // (the whole Deck is no choice by position)
    k > 0 && k < size && seqs.iter().all(|&s| s >= size - k)
}

fn audit_game(gi: usize, rec: &GameRecord, main_codes: &[u32], extra_codes: &[u32], out: &mut Vec<Value>) -> Result<(), String> {
    // (the audited cards: the table's "card" lines)
    let deck: std::collections::BTreeSet<u32> = rec.decks.iter().flat_map(|d| d.main.iter().chain(&d.extra).copied()).collect();
    out.push(json!({"game": gi, "deck_cards": deck}));
    let mut all_msgs: Vec<Msg> = Vec::new();
    let mut duel = rec.setup(None).map_err(|e| e.0)?;
    duel.ocg_start_duel();
    let mut hist: Vec<Past> = Vec::new();
    let mut prompt: Option<Msg> = None;
    let mut chain: Vec<u32> = Vec::new();
    let mut turn = 0u32;
    for entry in rec.responses.iter().map(Some).chain([None]) {
        let step = run_until_prompt_raw(&mut duel, prompt.as_ref()).map_err(|e| e.0)?;
        // (the arbiter's key of each move of a selected hidden card: reveal::selected_moves)
        // (with the prompt: at an action prompt the arbiter drops the notes that are left)
        let with_prompt: Vec<Msg> = step.messages.iter().chain(step.prompt.iter()).cloned().collect();
        let keys = reveal::selected_moves(&mut duel, &with_prompt);
        for (m, key) in step.messages.iter().zip(&keys) {
            match m {
                Msg::NewTurn { .. } => turn += 1,
                Msg::Chaining { code, .. } => chain.push(*code),
                Msg::ChainEnd {} => chain.clear(),
                _ => {}
            }
            let Msg::Move { code, from, to, reason } = m else { continue };
            let opp = 1 - from.con;
            let hidden_from = from.loc & (LOCATION_HAND | LOCATION_DECK) != 0
                || (from.loc & LOCATION_EXTRA != 0 && from.pos & POS_FACEDOWN != 0);
            if !hidden_from || to.loc == 0 || li_visible(to, opp) || *code == 0 {
                continue;
            }
            // the selection that chose the card
            let n = hist.len();
            let sel = (n.saturating_sub(MAX_LOOKBACK)..n).rev().find(|&k| {
                chosen(&hist[k].prompt, &hist[k].core).is_some_and(|(picked, _)| picked.iter().any(|c| same_card(c, *code, from)))
            });
            let confirm_step = step.messages
                .iter()
                .any(|x| reveals(x, opp, *code, from, to));
            let shown = filter_message(m, opp).is_some_and(|x| matches!(x, Msg::Move { code, .. } if code != 0));
            let mut row = json!({
                "game": gi, "turn": turn, "code": code, "from": [from.con, from.loc, from.seq], "to": [to.con, to.loc, to.pos],
                "reason": reason, "chain_top": chain.last(), "script_confirms": confirm_step,
                "filter_shows": shown,
            });
            if let Some(k) = key {
                let e = k.effect.unwrap_or(EffectKey { card: 0, desc: 0, typ: 0 });
                row["key"] = json!([e.card, e.desc, e.typ, k.from, k.to, k.cost]);
            }
            let Some(sel) = sel else {
                row["verdict"] = json!("no_selection");
                out.push(row);
                continue;
            };
            let selector = hist[sel].prompt.player();
            row["selector"] = json!(selector);
            row["selection"] = json!(hist[sel].prompt.name());
            let n_base = chosen(&hist[sel].prompt, &hist[sel].core).map(|(_, all)| all.len()).unwrap_or(0);
            row["candidates"] = json!(n_base);
            let Some(base) = (0..sel).rev().find(|&k| is_action(&hist[k].prompt)) else {
                row["verdict"] = json!("undecided");
                row["why"] = json!("no action prompt before the selection in the history");
                out.push(row);
                continue;
            };
            // (a choice among the top cards of the Deck: test cards on top, as many as the
            // candidates at a time, from a sample of the test cards that are not in the duel)
            let top = from.loc == LOCATION_DECK && deck_top_only(&hist[sel], from.con);
            // (to the field face-down: only the cards that can be Set there)
            let codes: Vec<u32> = if from.loc == LOCATION_EXTRA {
                extra_codes.to_vec()
            } else if to.loc & (LOCATION_MZONE | LOCATION_SZONE) != 0 {
                let db = card_db();
                let monster = to.loc == LOCATION_MZONE;
                main_codes.iter().copied().filter(|c| db.cards.get(c).is_some_and(|d| (d.type_ & TYPE_MONSTER != 0) == monster)).collect()
            } else {
                main_codes.to_vec()
            };
            let codes: Vec<u32> = if top {
                let in_duel: std::collections::BTreeSet<u32> = hist[sel].duel.ext_get_all_cards().iter().map(|c| c[4]).collect();
                let free: Vec<u32> = codes.into_iter().filter(|c| !in_duel.contains(c)).collect();
                let step = free.len().div_ceil(TOP_SAMPLE).max(1);
                free.into_iter().step_by(step).collect()
            } else {
                codes
            };
            let chunk_size = if top { candidates_in(&hist[sel].prompt, from.con, LOCATION_DECK).max(1) } else { CHUNK };
            let codes = &codes[..];
            // the reveals since the action started
            let confirmed = all_msgs[hist[base].msg_at..]
                .iter()
                .chain(step.messages.iter())
                .any(|x| reveals(x, opp, *code, from, to));
            row["script_confirms"] = json!(confirmed);
            row["activated"] = json!(activated(&hist[base].prompt, &hist[base].core));
            // (--discover: only the arbiter's selections that the table does not know, each at
            // most DISCOVER_ROWS times)
            if DISCOVER.load(Ordering::Relaxed) {
                let known = key.is_none_or(|k| reveal::table().selections.contains_key(&k) || decided(&k) >= DISCOVER_ROWS);
                if known {
                    row["verdict"] = json!("skipped");
                    out.push(row);
                    continue;
                }
            }
            let t0 = Instant::now();
            // (in chunks: the time of one check grows about with the square of the added cards)
            let (mut accepted, mut tested, mut rejected, mut error) = (0, 0, false, None);
            let mut rejected_codes: Vec<u32> = Vec::new();
            let mut excluded = 0;
            for chunk in codes.chunks(chunk_size) {
                match check(&hist, base, sel, from.con, from.loc, to, *reason, chunk, top) {
                    Ok(c) => {
                        accepted += c.accepted;
                        excluded += c.excluded;
                        tested += chunk.len();
                        rejected |= c.accepted + c.excluded < chunk.len();
                        rejected_codes.extend(c.rejected);
                    }
                    Err(e) => error = error.or(Some(e)),
                }
            }
            row["ms"] = json!((t0.elapsed().as_secs_f64() * 1000.0).round());
            row["accepted"] = json!(accepted);
            row["tested"] = json!(tested);
            row["added"] = json!(codes.len());
            if top {
                row["deck_top"] = json!(true);
            }
            row["excluded"] = json!(excluded);
            if rejected && tested - accepted - excluded <= MAX_REJECTED_LISTED {
                row["rejected"] = json!(rejected_codes);
            }
            if let Some(e) = &error {
                row["why"] = json!(e);
            }
            // (a rejected test card shows a requirement even if other chunks failed)
            row["verdict"] = json!(if rejected {
                "requirement"
            } else if error.is_none() {
                "free"
            } else {
                "undecided"
            });
            if let Some(k) = key
                && (rejected || error.is_none())
            {
                *DECIDED.lock().unwrap().entry(*k).or_insert(0) += 1;
            }
            out.push(row);
        }
        all_msgs.extend(step.messages.iter().cloned());
        let Some(entry) = entry else { break };
        if step.ended {
            break;
        }
        let p = step.prompt.ok_or("no prompt")?;
        let core = to_core_response(&p, entry.player, &entry.response).map_err(|e| e.0)?;
        let raw = encode(&p, &core).map_err(|e| e.0)?;
        hist.push(Past { duel: duel.clone(), msg_at: all_msgs.len(), prompt: p.clone(), core, raw: raw.clone() });
        if hist.len() > HISTORY {
            hist.remove(0);
        }
        duel.ocg_set_response(&raw);
        prompt = Some(p);
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).expect("usage: ygo-reveal-audit RECORDS.jsonl [--games N] [--threads T] [--codes N] [--discover]");
    DISCOVER.store(args.iter().any(|a| a == "--discover"), Ordering::Relaxed);
    let opt = |name: &str| args.iter().position(|a| a == name).and_then(|i| args.get(i + 1)).map(|v| v.parse::<usize>().expect(name));
    let max_games = opt("--games").unwrap_or(usize::MAX);
    let threads = opt("--threads").unwrap_or(8);
    let thin = |codes: Vec<u32>| match opt("--codes") {
        Some(n) => {
            let k = codes.len().div_ceil(n.max(1));
            codes.into_iter().step_by(k).collect()
        }
        None => codes,
    };
    let (main_codes, extra_codes) = (thin(test_codes(false)), thin(test_codes(true)));
    eprintln!("{} Main Deck and {} Extra Deck test cards", main_codes.len(), extra_codes.len());
    let file = std::io::BufReader::new(std::fs::File::open(path).expect("records"));
    let lines: Vec<String> = file.lines().map_while(Result::ok).filter(|l| !l.trim().is_empty()).take(max_games).collect();
    let next = AtomicUsize::new(0);
    let stdout = Mutex::new(std::io::stdout());
    std::thread::scope(|s| {
        for _ in 0..threads {
            s.spawn(|| {
                loop {
                    let gi = next.fetch_add(1, Ordering::Relaxed);
                    let Some(line) = lines.get(gi) else { break };
                    let rec: GameRecord = match serde_json::from_str(line) {
                        Ok(r) => r,
                        Err(e) => {
                            eprintln!("game {gi}: {e}");
                            continue;
                        }
                    };
                    let mut rows = Vec::new();
                    if let Err(e) = audit_game(gi, &rec, &main_codes, &extra_codes, &mut rows) {
                        rows.push(json!({"game": gi, "error": e}));
                    }
                    let mut o = stdout.lock().unwrap();
                    for r in rows {
                        writeln!(o, "{r}").unwrap();
                    }
                    o.flush().unwrap();
                }
            });
        }
    });
}
