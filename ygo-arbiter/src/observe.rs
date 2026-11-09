//! Per-player filtering of god-view core messages, the "fastchess protocol" view.
//!
//! [`filter_message`] returns what `viewer` is allowed to see of a core message, or None if the
//! message is not shown to them at all. The rules follow EDOPro's server (Multirole) and are
//! stricter where Multirole leaks information (card hints, confirmations shown to the other
//! player, face-down position changes, ...). Every message type is handled explicitly (the match
//! is exhaustive), so that new messages cannot leak silently.
//!
//! Hidden card codes are replaced by 0; effect descriptions of hidden cards (which encode the
//! card code) are replaced by 0 as well.

use std::collections::BTreeMap;

use ygo_core::constants::*;

use crate::actions::{InvalidResponse, Response};
use crate::messages::{Card, Loc, Msg};

const PILES: u32 = LOCATION_DECK | LOCATION_HAND | LOCATION_EXTRA | LOCATION_REMOVED;

/// The core gives face-down cards in piles different positions depending on how they got there
/// (e.g. 8 for the starting deck, 10 for cards returned to it). That would let players track
/// cards inside shuffled piles, so every face-down card in a pile gets the same position.
pub fn canon_pos(loc: u32, pos: u32) -> u32 {
    if loc & PILES != 0 && loc & LOCATION_OVERLAY == 0 && pos & POS_FACEUP == 0 {
        return POS_FACEDOWN_DEFENSE;
    }
    pos
}

/// Whether the identity of a card at `li` is visible to `viewer`.
pub fn loc_visible(con: u32, loc: u32, pos: u32, viewer: u32) -> bool {
    if loc == 0 || loc & (LOCATION_GRAVE | LOCATION_OVERLAY) != 0 || pos & POS_FACEUP != 0 {
        return true;
    }
    con == viewer && loc & LOCATION_DECK == 0
}

pub fn li_visible(li: &Loc, viewer: u32) -> bool {
    loc_visible(li.con, li.loc, li.pos, viewer)
}

fn canon_loc(l: &mut Loc) {
    l.pos = canon_pos(l.loc, l.pos);
}

fn canon_card(c: &mut Card) {
    if let Some(p) = c.pos.as_mut() {
        *p = canon_pos(c.loc, *p);
    }
}

fn canon_cards(cards: &mut [Card]) {
    cards.iter_mut().for_each(canon_card);
}

/// Canonicalizes the positions of all places in a message (observe._canon: every dict with
/// "loc" and "pos").
fn canon(m: &mut Msg) {
    use Msg::*;
    match m {
        SelectBattlecmd { activatable, attackable, .. } => {
            canon_cards(activatable);
            canon_cards(attackable);
        }
        SelectIdlecmd { summonable, spsummonable, repositionable, msetable, ssetable, activatable, .. } => {
            for l in [summonable, spsummonable, repositionable, msetable, ssetable, activatable] {
                canon_cards(l);
            }
        }
        SelectCard { cards, .. }
        | SelectChain { chains: cards, .. }
        | SelectTribute { cards, .. }
        | SortChain { cards, .. }
        | SortCard { cards, .. }
        | SelectCounter { cards, .. }
        | ConfirmDecktop { cards, .. }
        | ConfirmCards { cards, .. }
        | ConfirmExtratop { cards, .. } => canon_cards(cards),
        SelectSum { must_cards, cards, .. } => {
            canon_cards(must_cards);
            canon_cards(cards);
        }
        SelectUnselectCard { select_cards, unselect_cards, .. } => {
            canon_cards(select_cards);
            canon_cards(unselect_cards);
        }
        SelectEffectyn { loc, pos, .. }
        | Set { loc, pos, .. }
        | Summoning { loc, pos, .. }
        | Spsummoning { loc, pos, .. }
        | Flipsummoning { loc, pos, .. }
        | Chaining { loc, pos, .. }
        | PosChange { loc, pos, .. }
        | MissedEffect { loc, pos, .. }
        | CardHint { loc, pos, .. } => *pos = canon_pos(*loc, *pos),
        ShuffleSetCard { old, overlay, .. } => {
            old.iter_mut().chain(overlay.iter_mut()).for_each(canon_loc);
        }
        Move { from, to, .. } => {
            canon_loc(from);
            canon_loc(to);
        }
        Swap { loc1, loc2, .. } => {
            canon_loc(loc1);
            canon_loc(loc2);
        }
        CardSelected { cards } | RandomSelected { cards, .. } | BecomeTarget { cards } | RemoveCards { cards } => {
            cards.iter_mut().for_each(canon_loc)
        }
        Equip { card, target } | CardTarget { card, target } | CancelTarget { card, target } => {
            canon_loc(card);
            canon_loc(target);
        }
        Attack { attacker, target } | Battle { attacker, target, .. } => {
            canon_loc(attacker);
            canon_loc(target);
        }
        ReloadField { chains, .. } => chains.iter_mut().for_each(|c| canon_loc(&mut c.handler)),
        Retry {} | Hint { .. } | Win { .. } | SelectYesno { .. } | SelectOption { .. } | SelectPlace { .. }
        | SelectDisfield { .. } | SelectPosition { .. } | ShuffleDeck { .. } | ShuffleHand { .. } | ShuffleExtra { .. }
        | SwapGraveDeck { .. } | ReverseDeck {} | DeckTop { .. } | NewTurn { .. } | NewPhase { .. }
        | FieldDisabled { .. } | Summoned {} | Spsummoned {} | Flipsummoned {} | Chained { .. }
        | ChainSolving { .. } | ChainSolved { .. } | ChainEnd {} | ChainNegated { .. } | ChainDisabled { .. }
        | Draw { .. } | Damage { .. } | Recover { .. } | Lpupdate { .. } | PayLpcost { .. } | AddCounter { .. }
        | RemoveCounter { .. } | AttackDisabled {} | DamageStepStart {} | DamageStepEnd {} | TossCoin { .. }
        | TossDice { .. } | RockPaperScissors { .. } | HandRes { .. } | AnnounceRace { .. }
        | AnnounceAttrib { .. } | AnnounceCard { .. } | AnnounceNumber { .. } | TagSwap { .. } | AiName { .. }
        | ShowHint { .. } | PlayerHint { .. } | MatchKill { .. } => {}
    }
}

/// Selection prompts: keep own cards and public cards, hide everything else.
fn strip_prompt_cards(cards: &mut [Card], viewer: u32, has_pos: bool) {
    for c in cards {
        if c.con == viewer {
            continue;
        }
        if has_pos && loc_visible(c.con, c.loc, c.pos.unwrap_or(0), viewer) {
            continue;
        }
        c.code = 0;
        if c.desc.is_some() {
            c.desc = Some(0);
        }
    }
}

/// Hint types shown only to the named player.
fn hint_private(ht: u32) -> bool {
    matches!(ht, HINT_EVENT | HINT_MESSAGE | HINT_SELECTMSG | HINT_EFFECT)
}

/// Hint types shown to everyone but the named player.
fn hint_others(ht: u32) -> bool {
    matches!(ht, HINT_OPSELECTED | HINT_RACE | HINT_ATTRIB | HINT_CODE | HINT_NUMBER | HINT_ZONE)
}

/// Prompts the arbiter answers itself, without the player seeing them.
///
/// An empty, non-mandatory chain window leaves no choice, but whether such a window opens can
/// depend on hidden cards (e.g. a trigger of a card in the opponent's deck that turns out not to
/// be activatable). Clients auto-pass these windows; here the player does not see them at all.
pub fn is_hidden_prompt(m: &Msg) -> bool {
    matches!(m, Msg::SelectChain { chains, forced: 0, .. } if chains.is_empty())
}

/// The response to a hidden prompt.
pub fn auto_response() -> Response {
    Response::pass()
}

/// The option lists of a prompt that the core sorts by effect id (or builds from effect
/// containers): their order reflects hidden history (card creation order, when effects were last
/// enabled), so the arbiter presents them in a canonical order and maps the chosen index back
/// ([`to_core_response`]). Card selections list their cards in group order (a script may sort
/// them).
fn ordered_lists(m: &mut Msg) -> Vec<(&'static str, &mut Vec<Card>)> {
    use Msg::*;
    match m {
        SelectIdlecmd { summonable, spsummonable, repositionable, msetable, ssetable, activatable, .. } => vec![
            ("summonable", summonable),
            ("spsummonable", spsummonable),
            ("repositionable", repositionable),
            ("msetable", msetable),
            ("ssetable", ssetable),
            ("activatable", activatable),
        ],
        SelectBattlecmd { activatable, attackable, .. } => vec![("activatable", activatable), ("attackable", attackable)],
        SelectChain { chains, .. } => vec![("chains", chains)],
        SelectCard { cards, .. } | SelectTribute { cards, .. } => vec![("cards", cards)],
        SelectUnselectCard { select_cards, unselect_cards, .. } => {
            vec![("select_cards", select_cards), ("unselect_cards", unselect_cards)]
        }
        _ => vec![],
    }
}

fn option_key(c: &Card) -> (u32, u32, u32, u32, u32, u64) {
    let pos = if c.loc & LOCATION_OVERLAY != 0 { c.pos.unwrap_or(0) } else { 0 };
    (c.con, c.loc, c.seq, pos, c.code, c.desc.unwrap_or(0))
}

/// For each ordered option list of a prompt: the core indices in the order the player sees them.
pub fn option_order(m: &Msg, viewer: u32) -> BTreeMap<&'static str, Vec<usize>> {
    let mut out = BTreeMap::new();
    if let Msg::AnnounceNumber { values, .. } = m {
        // (scripts may collect the numbers from cards, in their order)
        let mut order: Vec<usize> = (0..values.len()).collect();
        order.sort_by_key(|&i| (values[i], i));
        out.insert("values", order);
        return out;
    }
    let mut stripped = strip_prompt(m, viewer);
    for (key, list) in ordered_lists(&mut stripped) {
        let mut order: Vec<usize> = (0..list.len()).collect();
        order.sort_by_key(|&i| (option_key(&list[i]), i));
        out.insert(key, order);
    }
    out
}

/// `list[i]` as Python indexes (negative from the end); an error out of range.
fn py_index(list: &[usize], i: i64) -> Result<usize, InvalidResponse> {
    let j = if i < 0 { i + list.len() as i64 } else { i };
    usize::try_from(j).ok().and_then(|j| list.get(j).copied()).ok_or_else(|| InvalidResponse(format!("index {i} out of range")))
}

/// Translates a response to the prompt as the player saw it ([`filter_prompt`]) into the core's
/// option order. An error where the Python function raises (an index out of range).
pub fn to_core_response(m: &Msg, viewer: u32, resp: &Response) -> Result<Response, InvalidResponse> {
    use Msg::*;
    let mut out = resp.clone();
    match m {
        SelectCard { .. } | SelectTribute { .. } => {
            if let Some(idx) = &resp.indices {
                let order = &option_order(m, viewer)["cards"];
                out.indices = Some(idx.iter().map(|&i| py_index(order, i).map(|x| x as i64)).collect::<Result<_, _>>()?);
            }
        }
        SelectUnselectCard { .. } => {
            if let Some(i) = resp.index {
                let order = option_order(m, viewer);
                let (sel, unsel) = (&order["select_cards"], &order["unselect_cards"]);
                let n = sel.len() as i64;
                out.index = Some(if i < n { py_index(sel, i)? as i64 } else { n + py_index(unsel, i - n)? as i64 });
            }
        }
        AnnounceNumber { .. } => {
            if let Some(i) = resp.index {
                out.index = Some(py_index(&option_order(m, viewer)["values"], i)? as i64);
            }
        }
        SelectChain { .. } | SelectIdlecmd { .. } | SelectBattlecmd { .. } => {
            let Some(i) = resp.index else { return Ok(out) };
            let key = match m {
                SelectChain { .. } => (i >= 0).then_some("chains"),
                _ => resp.kind.filter(|k| k.cards(m).is_some()).and_then(|k| k.list_name()),
            };
            if let Some(key) = key {
                out.index = Some(py_index(&option_order(m, viewer)[key], i)? as i64);
            }
        }
        _ => {}
    }
    Ok(out)
}

/// Strips a selection prompt addressed to `viewer` (option lists in canonical order).
pub fn filter_prompt(m: &Msg, viewer: u32) -> Msg {
    let orders = option_order(m, viewer);
    let mut s = strip_prompt(m, viewer);
    if let Msg::AnnounceNumber { values, .. } = &mut s {
        let order = &orders["values"];
        *values = order.iter().map(|&i| values[i]).collect();
    }
    for (key, list) in ordered_lists(&mut s) {
        let order = &orders[key];
        *list = order.iter().map(|&i| list[i].clone()).collect();
    }
    s
}

fn strip_prompt(m: &Msg, viewer: u32) -> Msg {
    use Msg::*;
    let mut m = m.clone();
    canon(&mut m);
    match &mut m {
        SelectIdlecmd { summonable, spsummonable, repositionable, msetable, ssetable, activatable, .. } => {
            for l in [summonable, spsummonable, repositionable, msetable, ssetable, activatable] {
                strip_prompt_cards(l, viewer, false);
            }
        }
        SelectBattlecmd { activatable, attackable, .. } => {
            strip_prompt_cards(activatable, viewer, false);
            strip_prompt_cards(attackable, viewer, false);
        }
        SelectChain { chains, hint_timing, hint_timing_other, spe_count, .. } => {
            strip_prompt_cards(chains, viewer, true);
            // (the core's timing hints for the client's auto-pass: what happened since the windows
            // were last passed, including windows only the opponent's hidden cards opened, e.g. a
            // hand trap's; the play gives the player's own, `Timing`)
            *hint_timing = None;
            *hint_timing_other = None;
            // (spe_count counts the offered effects whose hint timing matches those timings: the
            // same information, a hint for clients only)
            *spe_count = None;
        }
        SelectPosition { code, .. } => {
            // (the code of the card whose position is chosen: it can be an opponent's face-down
            // card, e.g. "change a monster's battle position", and the prompt does not say where
            // the card is; a summoned card is named by the hint before the prompt anyway)
            *code = 0;
        }
        SelectEffectyn { con, loc, pos, code, desc, .. } => {
            if *con != viewer && !loc_visible(*con, *loc, *pos, viewer) {
                *code = 0;
                *desc = 0;
            }
        }
        SelectCard { cards, .. } => strip_prompt_cards(cards, viewer, true),
        SelectUnselectCard { select_cards, unselect_cards, .. } => {
            strip_prompt_cards(select_cards, viewer, true);
            strip_prompt_cards(unselect_cards, viewer, true);
        }
        SelectSum { must_cards, cards, .. } => {
            strip_prompt_cards(must_cards, viewer, true);
            strip_prompt_cards(cards, viewer, true);
        }
        SelectTribute { cards, .. } | SelectCounter { cards, .. } | SortCard { cards, .. } | SortChain { cards, .. } => {
            strip_prompt_cards(cards, viewer, false)
        }
        AnnounceCard { values, .. } => *values = canonical_opcodes(values),
        _ => {}
    }
    m
}

// ANNOUNCE_CARD filters are RPN programs (see actions::is_declarable) that scripts often assemble
// by iterating over cards (e.g. one "code ISCODE OR" term per card name in the deck, by way of a
// Lua table whose iteration order follows insertion order). Which names can be declared is what
// the player sees; the order of the terms would tell the order of the cards. Chains of the
// commutative boolean operators are flattened and their operands sorted.

fn is_unary(op: u64) -> bool {
    matches!(
        op,
        OPCODE_NEG | OPCODE_NOT | OPCODE_BNOT | OPCODE_ISCODE | OPCODE_ISTYPE | OPCODE_ISRACE | OPCODE_ISATTRIBUTE | OPCODE_ISSETCARD
    )
}

pub(crate) fn is_binary(op: u64) -> bool {
    matches!(
        op,
        OPCODE_ADD
            | OPCODE_SUB
            | OPCODE_MUL
            | OPCODE_DIV
            | OPCODE_AND
            | OPCODE_OR
            | OPCODE_BAND
            | OPCODE_BOR
            | OPCODE_BXOR
            | OPCODE_LSHIFT
            | OPCODE_RSHIFT
    )
}

fn is_flag(op: u64) -> bool {
    matches!(op, OPCODE_ALLOW_ALIASES | OPCODE_ALLOW_TOKENS)
}

/// An expression tree (the Python tuples (op, operands...); they compare the same way).
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Expr(Vec<ExprItem>);

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
enum ExprItem {
    Op(u64),
    Sub(Expr),
}

/// An equivalent ANNOUNCE_CARD filter in canonical form (unchanged if it is not a well formed
/// program).
pub fn canonical_opcodes(values: &[u64]) -> Vec<u64> {
    // (a tuple (op, operands...) is compared element by element: the op first, then the operands,
    // which are tuples themselves; Expr keeps that order with the op as the first item)
    let mut stack: Vec<Expr> = Vec::new();
    let mut flags = Vec::new();
    let op_of = |e: &Expr| match e.0[0] {
        ExprItem::Op(o) => o,
        ExprItem::Sub(_) => unreachable!(),
    };
    for &op in values {
        if is_flag(op) {
            flags.push(op);
        } else if is_binary(op) {
            if stack.len() < 2 {
                return values.to_vec();
            }
            let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap());
            if matches!(op, OPCODE_AND | OPCODE_OR) {
                let mut operands = Vec::new();
                for e in [a, b] {
                    if op_of(&e) == op && e.0.len() > 2 {
                        operands.extend(e.0[1..].iter().cloned());
                    } else {
                        operands.push(ExprItem::Sub(e));
                    }
                }
                operands.sort();
                let mut items = vec![ExprItem::Op(op)];
                items.extend(operands);
                stack.push(Expr(items));
            } else {
                stack.push(Expr(vec![ExprItem::Op(op), ExprItem::Sub(a), ExprItem::Sub(b)]));
            }
        } else if is_unary(op) {
            let Some(a) = stack.pop() else { return values.to_vec() };
            stack.push(Expr(vec![ExprItem::Op(op), ExprItem::Sub(a)]));
        } else {
            // (a constant or a GET operator)
            stack.push(Expr(vec![ExprItem::Op(op)]));
        }
    }
    if stack.len() != 1 {
        return values.to_vec();
    }
    fn emit(e: &Expr, out: &mut Vec<u64>) {
        let ExprItem::Op(op) = e.0[0] else { unreachable!() };
        let operands: Vec<&Expr> = e.0[1..]
            .iter()
            .map(|x| match x {
                ExprItem::Sub(s) => s,
                ExprItem::Op(_) => unreachable!(),
            })
            .collect();
        if operands.is_empty() {
            out.push(op);
        } else if matches!(op, OPCODE_AND | OPCODE_OR) {
            emit(operands[0], out);
            for x in &operands[1..] {
                emit(x, out);
                out.push(op);
            }
        } else {
            for x in &operands {
                emit(x, out);
            }
            out.push(op);
        }
    }
    let mut out = Vec::new();
    emit(&stack[0], &mut out);
    flags.sort();
    flags.dedup();
    out.extend(flags);
    out
}

fn loc_key(l: &Loc) -> (u32, u32, u32, u32) {
    (l.con, l.loc, l.seq, l.pos)
}

fn card_loc_key(c: &Card) -> (u32, u32, u32, u32) {
    (c.con, c.loc, c.seq, c.pos.unwrap_or(0))
}

/// Filters a non-prompt god-view message for `viewer` (None = not shown).
pub fn filter_message(m: &Msg, viewer: u32) -> Option<Msg> {
    use Msg::*;
    let mut out = filter_raw(m, viewer)?;
    canon(&mut out);
    // multi-card listings: the viewer gets them sorted by location (the patched core lists them
    // in location order too; this keeps the order independent of it)
    match &mut out {
        ConfirmCards { cards, .. } => cards.sort_by_key(card_loc_key),
        RandomSelected { cards, .. } | CardSelected { cards } | BecomeTarget { cards } | RemoveCards { cards } => {
            cards.sort_by_key(loc_key)
        }
        ShuffleSetCard { old, overlay, new, .. } => {
            // (where each card went only their controller knows)
            let mut new = new.take();
            if old.first().is_none_or(|l| l.con != viewer) {
                new = None;
            }
            let mut idx: Vec<usize> = (0..old.len()).collect();
            idx.sort_by_key(|&i| loc_key(&old[i]));
            *old = idx.iter().map(|&i| old[i]).collect();
            *overlay = idx.iter().map(|&i| overlay[i]).collect();
            out_new(new.map(|n| idx.iter().map(|&i| n[i]).collect()), &mut out);
        }
        _ => {}
    }
    Some(out)
}

fn out_new(v: Option<Vec<u32>>, m: &mut Msg) {
    if let Msg::ShuffleSetCard { new, .. } = m {
        *new = v;
    }
}

fn filter_raw(m: &Msg, viewer: u32) -> Option<Msg> {
    use Msg::*;
    let mut m = m.clone();
    match &mut m {
        DeckTop { pos, .. } => {
            // (the position of a face-down deck card tells how it got there, see canon_pos)
            *pos = canon_pos(LOCATION_DECK, *pos);
        }
        Win { .. } | ShuffleDeck { .. } | SwapGraveDeck { .. } | ShuffleSetCard { .. } | ReverseDeck {} | NewTurn { .. }
        | NewPhase { .. } | FieldDisabled { .. } | Summoning { .. } | Summoned {} | Spsummoned {} | Flipsummoning { .. }
        | Flipsummoned {} | Chaining { .. } | Chained { .. } | ChainSolving { .. } | ChainSolved { .. } | ChainEnd {}
        | ChainNegated { .. } | ChainDisabled { .. } | CardSelected { .. } | RandomSelected { .. } | BecomeTarget { .. }
        | Damage { .. } | Recover { .. } | Equip { .. } | Lpupdate { .. } | CardTarget { .. } | CancelTarget { .. }
        | PayLpcost { .. } | AddCounter { .. } | RemoveCounter { .. } | Attack { .. } | Battle { .. } | AttackDisabled {}
        | DamageStepStart {} | DamageStepEnd {} | TossCoin { .. } | TossDice { .. } | HandRes { .. } | PlayerHint { .. }
        | MatchKill { .. } | RemoveCards { .. } | ConfirmDecktop { .. } | ConfirmExtratop { .. } => {}
        // prompts are delivered separately, only to the deciding player
        SelectBattlecmd { .. } | SelectIdlecmd { .. } | SelectEffectyn { .. } | SelectYesno { .. } | SelectOption { .. }
        | SelectCard { .. } | SelectChain { .. } | SelectPlace { .. } | SelectPosition { .. } | SelectTribute { .. }
        | SortChain { .. } | SelectCounter { .. } | SelectSum { .. } | SelectDisfield { .. } | SortCard { .. }
        | SelectUnselectCard { .. } | RockPaperScissors { .. } | AnnounceRace { .. } | AnnounceAttrib { .. }
        | AnnounceCard { .. } | AnnounceNumber { .. } | Retry {} => return None,
        Hint { hint_type, player, .. } => {
            if hint_private(*hint_type) && *player != viewer || hint_others(*hint_type) && *player == viewer {
                return None;
            }
        }
        Move { code, from, to, reason } => {
            if to.loc & (LOCATION_MZONE | LOCATION_SZONE) != 0 && *reason & REASON_SPSUMMON == 0 {
                // Placing a card on the field (summon, set, activation, equipping, moving it to
                // another zone) does not update its reason (move_to_field), so the message would
                // carry the stale reason of how the card entered e.g. the hand (drawn by rule or
                // by an effect, searched, ...), even much later when it moves on from the field.
                // The viewer cannot know that for hidden cards, and for its own hand it tells
                // apart copies of a card after a shuffle. (A special summon sets its own reason.)
                *reason = 0;
            }
            if (from.con, from.loc) == (to.con, to.loc) && !li_visible(from, viewer) {
                // reordering within a hidden pile (e.g. placing a card on top of the deck)
                // reports the card's stale reason as well
                *reason = 0;
            }
            if to.con == viewer || li_visible(to, viewer) {
                return Some(m);
            }
            // (a selected card that the player has to show, e.g. a cost with a requirement, gets a
            // confirmation before the move: reveal::annotate)
            *code = 0;
        }
        Draw { player, cards } => {
            for c in cards.iter_mut() {
                if c.pos & POS_FACEUP == 0 {
                    c.pos = POS_FACEDOWN_DEFENSE;
                    if *player != viewer {
                        c.code = 0;
                    }
                }
            }
        }
        ShuffleHand { player, codes, public, prev } => {
            if *player != viewer {
                // (a face-up card, e.g. one kept revealed in the hand, stays visible; where the
                // cards went only their owner knows)
                *prev = None;
                hide_unpublic(codes, public);
            }
        }
        ShuffleExtra { player, codes, public } => {
            if *player != viewer {
                hide_unpublic(codes, public);
            }
        }
        Set { con, code, .. } => {
            if *con != viewer {
                *code = 0;
            }
        }
        Spsummoning { con, pos, code, .. } => {
            if *con != viewer && *pos & POS_FACEDOWN != 0 {
                *code = 0;
            }
        }
        PosChange { con, pos, prev_pos, code, .. } => {
            if *con != viewer && *pos & POS_FACEUP == 0 && *prev_pos & POS_FACEUP == 0 {
                *code = 0;
            }
        }
        Swap { code1, loc1, code2, loc2 } => {
            let (l1, l2) = (*loc1, *loc2);
            if !(li_visible(&l1, viewer) || l2.con == viewer) {
                *code1 = 0;
            }
            if !(li_visible(&l2, viewer) || l1.con == viewer) {
                *code2 = 0;
            }
        }
        ConfirmCards { player, cards } => {
            if *player != viewer {
                // The viewer sees its own cards that are shown to the opponent, but not its own
                // Main Deck cards: the message is the same when the opponent looks at them (e.g.
                // the top card), and then the owner does not see them. When the owner reveals a
                // Main Deck card itself, it knows the card from its own selection.
                for c in cards.iter_mut().filter(|c| c.con != viewer || c.loc == LOCATION_DECK) {
                    c.code = 0;
                }
            }
        }
        CardHint { con, loc, pos, .. } => {
            // hints emitted while cards are created (location 0) would reveal deck contents
            if *loc == 0 || !loc_visible(*con, *loc, *pos, viewer) {
                return None;
            }
        }
        MissedEffect { con, .. } => {
            if *con != viewer {
                return None;
            }
        }
        AiName { .. } | ShowHint { .. } | TagSwap { .. } | ReloadField { .. } => return None,
    }
    Some(m)
}

fn hide_unpublic(codes: &mut [u32], public: &[bool]) {
    for (i, c) in codes.iter_mut().enumerate() {
        if !public.get(i).copied().unwrap_or(false) {
            *c = 0;
        }
    }
}

/// The chain prompts' timing hints (SELECT_CHAIN's `hint_timing`, `hint_timing_other`: what
/// happened since the chain windows were last passed, TIMING_* bits per player) as a player can
/// know them. The core's own hints also reset at windows only the opponent's hidden cards opened
/// (they are stripped, `strip_prompt`); these follow the core's rules (where it sets the bits)
/// but see only the player's messages (`filter_message`), its own prompts and responses: a
/// window ends when the player passed and no chain link follows.
#[derive(Debug, Clone, Default)]
pub struct Timing {
    /// the bits of each player's actions
    hints: [u32; 2],
    turn_player: u32,
    /// the player passed a chain window: it ends unless a chain link follows
    passed: bool,
    /// a chain is open (CHAINING to CHAIN_END); a link resolves (CHAIN_SOLVING to CHAIN_SOLVED)
    chain: bool,
    solving: bool,
    /// the players of the summons under way (a summon's bit is set once it succeeded)
    summoning: Vec<u32>,
}

impl Timing {
    /// What survives the end of a chain window (`QuickEffect`, `break_effect`).
    const KEPT: u32 = TIMING_DAMAGE_STEP | TIMING_DAMAGE_CAL;

    /// A message as the player sees it (`filter_message`'s).
    pub fn message(&mut self, m: &Msg) {
        use Msg::*;
        if std::mem::take(&mut self.passed) && !matches!(m, Chaining { .. }) {
            self.window_end();
        }
        let tp = self.turn_player as usize;
        let h = &mut self.hints;
        match m {
            NewTurn { player } => {
                h[tp] = 0;
                self.turn_player = *player;
            }
            // (a phase's window: at its start, the Draw Phase's after the draw; the end of a
            // phase clears the turn player's hints)
            NewPhase { phase } => {
                h[tp] = match *phase {
                    PHASE_STANDBY => TIMING_STANDBY_PHASE,
                    PHASE_BATTLE_START => TIMING_BATTLE_START,
                    PHASE_END => TIMING_END_PHASE,
                    _ => 0,
                }
            }
            // (the events the core announces where it sets the hints: a phase's last window,
            // the end of the Main Phase, the steps of a battle)
            Hint { hint_type: HINT_EVENT, data, .. } => match data {
                20 => h[tp] = TIMING_DRAW_PHASE,
                21 => h[tp] = TIMING_STANDBY_PHASE,
                28 => h[tp] = TIMING_BATTLE_START,
                25 => h[tp] = TIMING_BATTLE_END,
                26 => h[tp] = TIMING_END_PHASE,
                23 => h[tp] = TIMING_MAIN_END,
                24 => *h = [TIMING_BATTLE_PHASE; 2],
                29 => h[tp] = TIMING_BATTLE_STEP_END,
                41 => h[tp] = TIMING_DAMAGE_STEP,
                42 | 45 => h[tp] = TIMING_DAMAGE_CAL,
                43 => h.iter_mut().for_each(|x| *x |= TIMING_BATTLED),
                _ => {}
            },
            Attack { .. } => h[tp] = TIMING_ATTACK,
            DamageStepStart {} | Battle { .. } => h[tp] = 0,
            Chaining { .. } => self.chain = true,
            // (after an activation's costs and targets: `break_effect`)
            Chained { .. } => h.iter_mut().for_each(|x| *x &= Self::KEPT),
            ChainSolving { .. } => {
                self.solving = true;
                h.iter_mut().for_each(|x| *x &= Self::KEPT);
            }
            ChainSolved { .. } => self.solving = false,
            ChainEnd {} => {
                (self.chain, self.solving) = (false, false);
                h.iter_mut().for_each(|x| *x |= TIMING_CHAIN_END);
            }
            // (a summon by an effect: at once; by the rules: once it succeeded, outside a chain)
            // (a summon by the rules starts after its tributes and materials: `break_effect`)
            Summoning { con, .. } | Flipsummoning { con, .. } => {
                h.iter_mut().for_each(|x| *x &= Self::KEPT);
                self.summoning = vec![*con];
            }
            Spsummoning { con, .. } if self.solving => h[*con as usize & 1] |= TIMING_SPSUMMON,
            Spsummoning { con, .. } => {
                if self.summoning.is_empty() {
                    h.iter_mut().for_each(|x| *x &= Self::KEPT);
                }
                self.summoning.push(*con);
            }
            Summoned {} | Spsummoned {} | Flipsummoned {} => {
                let bit = match m {
                    Summoned {} => TIMING_SUMMON,
                    Spsummoned {} => TIMING_SPSUMMON,
                    _ => TIMING_FLIPSUMMON,
                };
                for c in std::mem::take(&mut self.summoning) {
                    if !self.chain {
                        h[c as usize & 1] |= bit;
                    }
                }
            }
            // (by the rules, not by an effect)
            Set { con, loc, .. } if !self.solving => h[*con as usize & 1] |= if *loc == LOCATION_MZONE { TIMING_MSET } else { TIMING_SSET },
            // (a Set Spell or Trap turned face-up to activate it: not a change of position)
            PosChange { con, loc, .. } if *loc != LOCATION_SZONE => h[*con as usize & 1] |= TIMING_POS_CHANGE,
            Draw { player, .. } => h[*player as usize & 1] |= TIMING_DRAW | TIMING_TOHAND,
            Damage { player, .. } => h[*player as usize & 1] |= TIMING_DAMAGE,
            Recover { player, .. } => h[*player as usize & 1] |= TIMING_RECOVER,
            Equip { target, .. } => h[target.con as usize & 1] |= TIMING_EQUIP,
            Move { from, to, reason, .. } if from.loc != 0 => {
                let c = from.con as usize & 1;
                if reason & REASON_DESTROY != 0 {
                    h[c] |= TIMING_DESTROY;
                }
                h[c] |= match to.loc {
                    LOCATION_GRAVE => TIMING_TOGRAVE,
                    LOCATION_HAND => TIMING_TOHAND,
                    LOCATION_DECK => TIMING_TODECK,
                    LOCATION_REMOVED => TIMING_REMOVE,
                    _ => 0,
                };
            }
            _ => {}
        }
    }

    fn window_end(&mut self) {
        self.hints.iter_mut().for_each(|x| *x &= Self::KEPT);
    }

    /// The player's chain prompt with its hints, from its own view: (its, the opponent's).
    pub fn prompt(&mut self, m: &mut Msg, player: u32) {
        if let Msg::SelectChain { hint_timing, hint_timing_other, .. } = m {
            // (a new window of the player's after it passed: the last one ended)
            if std::mem::take(&mut self.passed) {
                self.window_end();
            }
            *hint_timing = Some(self.hints[player as usize & 1]);
            *hint_timing_other = Some(self.hints[1 - (player as usize & 1)]);
        }
    }

    /// The player's response to its prompt `m`.
    pub fn response(&mut self, m: &Msg, r: &Response) {
        match m {
            Msg::SelectChain { .. } if r.index.is_none_or(|i| i < 0) => self.passed = true,
            Msg::SelectIdlecmd { player, .. } if matches!(r.kind, Some(crate::actions::Command::ToBp | crate::actions::Command::ToEp)) => {
                self.hints[*player as usize & 1] = TIMING_MAIN_END;
            }
            _ => {}
        }
    }
}
