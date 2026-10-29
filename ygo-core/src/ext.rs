//! Extensions for the belief replay of the yugioh_AI_2 project (`core_ext/ocgext.cpp` of the C++
//! core, with the hooks of `core_ext/ocgcore.patch`).
//!
//! The belief replay "cheats": it forces the outcome of shuffles and random events so that its
//! simulated duel reproduces what a player observed, makes effects activatable or cards match
//! filters where hidden cards decide, and reorders or places hidden cards. When nothing has been
//! queued the hooks are no-ops, so a duel that never calls the `ext_*` API behaves as upstream.
//!
//! The state lives in the duel (`Duel::ext`), so a copy of a duel copies it too.

use std::collections::{BTreeMap, BTreeSet};

use crate::constants::*;
use crate::duel::{Deque, Duel};
use crate::types::*;
use crate::units::Unit;

pub const FORCED_COIN: usize = 0;
pub const FORCED_DICE: usize = 1;
const FORCED_KIND_COUNT: usize = 2;

/// A card of a forced random selection: controller, location, sequence, overlay sequence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ForcedLoc {
    pub con: u8,
    pub loc: u8,
    pub seq: u32,
    pub overlay_seq: u32,
}

/// The queued forcings (`State` of ocgext.cpp; `ext_clear_forced` resets all of it).
#[derive(Clone, Debug)]
pub struct ExtState {
    forced_ints: [Deque<i32>; FORCED_KIND_COUNT],
    /// [player][0 = deck, 1 = hand, 2 = extra]: wanted order by card id (may be empty) and by
    /// code (the fallback)
    forced_shuffles: [[Deque<(Vec<u32>, Vec<u32>)>; 3]; 2],
    forced_random_selects: Deque<Vec<ForcedLoc>>,
    /// (ids, codes)
    forced_set_card_shuffles: Deque<(Vec<u32>, Vec<u32>)>,
    /// (player, card code, effect description) whose cost/target checks are skipped
    forced_activatable: BTreeSet<(u8, u32, u64)>,
    /// (card code, effect description) of mandatory triggers only the forcing made activatable
    forced_used: BTreeSet<(u32, u64)>,
    /// card ids that count as matching every card filter
    forced_match: BTreeSet<u32>,
    forced_nomatch: BTreeSet<u32>,
    /// the filter read log: filters evaluated on cards the viewer cannot see (-1: off)
    read_viewer: i32,
    read_log: Vec<(u32, u8)>,
    errors: u32,
    consumed: u32,
}

impl Default for ExtState {
    fn default() -> Self {
        ExtState {
            forced_ints: Default::default(),
            forced_shuffles: Default::default(),
            forced_random_selects: Deque::default(),
            forced_set_card_shuffles: Deque::default(),
            forced_activatable: BTreeSet::new(),
            forced_used: BTreeSet::new(),
            forced_match: BTreeSet::new(),
            forced_nomatch: BTreeSet::new(),
            read_viewer: -1,
            read_log: Vec::new(),
            errors: 0,
            consumed: 0,
        }
    }
}

/// The property read log (card id -> mask), writable through `&Duel`.
#[derive(Debug, Default)]
pub struct PropReadLog(pub std::sync::Mutex<BTreeMap<u32, u32>>);

impl Clone for PropReadLog {
    fn clone(&self) -> Self {
        PropReadLog(std::sync::Mutex::new(self.0.lock().unwrap().clone()))
    }
}

/// `Duel::ext`
#[derive(Clone, Debug, Default)]
pub struct Ext {
    pub state: ExtState,
    /// The property read log (an analysis aid, `duel::read_log_on`/`read_log` of the patch): per
    /// card id a mask of the properties the game read (see `ext_read_log`).
    pub prop_read_on: bool,
    /// (behind a lock: reads are noted where the engine only reads the duel)
    pub prop_read_log: PropReadLog,
    /// Set while a rule helper (`proc_workaround.lua`) passes its own filters to the engine: forced
    /// matches and the read log do not apply to them (`is_rule_filter` of the patch; see
    /// `api::rule_filters`). `check_matching` clears it while the filter runs: the filters that
    /// one uses are card filters again.
    pub rule_filter: bool,
}

fn shuffle_index(location: u32) -> Option<usize> {
    match location {
        LOCATION_DECK => Some(0),
        LOCATION_HAND => Some(1),
        LOCATION_EXTRA => Some(2),
        _ => None,
    }
}

fn is_sequenced_list(location: u32) -> bool {
    location & (LOCATION_DECK | LOCATION_HAND | LOCATION_EXTRA | LOCATION_GRAVE | LOCATION_REMOVED) != 0
}

/// The locations `OCG_ExtGetAllCards` lists, in its order.
const ALL_LOCATIONS: [u32; 7] = [LOCATION_DECK, LOCATION_HAND, LOCATION_MZONE, LOCATION_SZONE, LOCATION_GRAVE, LOCATION_REMOVED, LOCATION_EXTRA];

impl Duel {
    /// The card with this unique id (`cardid`), if any.
    pub fn card_by_id(&self, id: u32) -> Option<CardId> {
        if id >= 1 && (id as usize) <= self.cards.len() && self.card(CardId(id - 1)).cardid == id {
            return Some(CardId(id - 1));
        }
        (0..self.cards.len() as u32).map(CardId).find(|&c| self.card(c).cardid == id)
    }

    /// `location_list` of ocgext.cpp: the slots of a location (`None` for an empty field zone).
    fn ext_location_list(&self, playerid: u8, location: u32) -> Option<Vec<Option<CardId>>> {
        if playerid > 1 {
            return None;
        }
        let p = &self.player[playerid as usize];
        let pile = |v: &Vec<CardId>| v.iter().map(|&c| Some(c)).collect();
        Some(match location {
            LOCATION_DECK => pile(&p.list_main),
            LOCATION_HAND => pile(&p.list_hand),
            LOCATION_EXTRA => pile(&p.list_extra),
            LOCATION_GRAVE => pile(&p.list_grave),
            LOCATION_REMOVED => pile(&p.list_remove),
            LOCATION_MZONE => p.list_mzone.clone(),
            LOCATION_SZONE => p.list_szone.clone(),
            _ => return None,
        })
    }

    /// Permutes the first `count` cards of a pile so that entry i has `data.code == wanted[i]`;
    /// among cards with equal codes the current relative order is kept. False (pile unchanged)
    /// if the multisets differ.
    fn ext_reorder_by_code(&mut self, playerid: u8, location: u8, count: usize, wanted: &[u32]) -> bool {
        let list = self.pile_mut(playerid, location).clone();
        if wanted.len() != count || count > list.len() {
            return false;
        }
        let mut used = vec![false; count];
        let mut result = Vec::with_capacity(count);
        for &code in wanted {
            let Some(i) = (0..count).find(|&i| !used[i] && self.card(list[i]).data.code == code) else { return false };
            used[i] = true;
            result.push(list[i]);
        }
        self.pile_mut(playerid, location)[..count].copy_from_slice(&result);
        true
    }
    fn ext_reorder_by_id(&mut self, playerid: u8, location: u8, count: usize, wanted: &[u32]) -> bool {
        let list = self.pile_mut(playerid, location).clone();
        if wanted.len() != count || count > list.len() {
            return false;
        }
        let mut result: Vec<CardId> = Vec::with_capacity(count);
        for &id in wanted {
            let Some(&c) = list[..count].iter().find(|&&c| self.card(c).cardid == id) else { return false };
            if result.contains(&c) {
                return false;
            }
            result.push(c);
        }
        self.pile_mut(playerid, location)[..count].copy_from_slice(&result);
        true
    }
    fn ext_reorderable_count(&self, playerid: u8, location: u32, len: usize) -> usize {
        if location == LOCATION_EXTRA { len - self.player[playerid as usize].extra_p_count as usize } else { len }
    }

    // --- hooks (called from the engine) ------------------------------------------------------

    /// `ocgext::forced_int`: a queued forced coin/dice result, else `fallback` (already drawn from
    /// the random generator, as in the C++).
    pub fn ext_forced_int(&mut self, kind: usize, fallback: i32) -> i32 {
        match self.ext.state.forced_ints[kind].pop_front() {
            Some(v) => {
                self.ext.state.consumed += 1;
                v
            }
            None => fallback,
        }
    }

    /// `ocgext::on_shuffle`: right before the shuffle message; `count` leading cards take part.
    pub fn ext_on_shuffle(&mut self, playerid: u8, location: u8, count: usize) {
        let Some(idx) = shuffle_index(location as u32) else { return };
        if playerid > 1 {
            return;
        }
        let Some((ids, codes)) = self.ext.state.forced_shuffles[playerid as usize][idx].pop_front() else { return };
        self.ext.state.consumed += 1;
        // by card id if these are the cards in the pile, else by code
        if !(!ids.is_empty() && self.ext_reorder_by_id(playerid, location, count, &ids)) && !self.ext_reorder_by_code(playerid, location, count, &codes) {
            self.ext.state.errors += 1;
            return;
        }
        self.reset_sequence(playerid, location);
    }

    /// `ocgext::on_random_select`: at the end of `Group.RandomSelect`, may replace the selection.
    pub fn ext_on_random_select(&mut self, from: &CardSet, selected: &mut CardSet) {
        let Some(wanted) = self.ext.state.forced_random_selects.pop_front() else { return };
        self.ext.state.consumed += 1;
        let mut result = CardSet::new();
        for w in &wanted {
            let found = from.iter().find(|&c| {
                let info = c.get_info_location(self);
                info.controler == w.con
                    && info.location == w.loc
                    && info.sequence == w.seq
                    && (w.loc as u32 & LOCATION_OVERLAY == 0 || info.position == w.overlay_seq)
            });
            let Some(c) = found else {
                self.ext.state.errors += 1;
                return;
            };
            result.insert(c);
        }
        if result.len() != selected.len() {
            self.ext.state.errors += 1;
            return;
        }
        *selected = result;
    }

    /// `ocgext::on_shuffle_set_card`: after `Duel.ShuffleSetCard` computed its permutation (`ms[i]`
    /// goes to sequence `seq[i]`); may rewrite `ms`.
    pub fn ext_on_shuffle_set_card(&mut self, ms: &mut [CardId], seq: &[u8]) {
        let Some((ids, codes)) = self.ext.state.forced_set_card_shuffles.pop_front() else { return };
        self.ext.state.consumed += 1;
        let ct = ms.len();
        if codes.len() != ct {
            self.ext.state.errors += 1;
            return;
        }
        // wanted[k] is the card (by id if these are the cards, else by code) that must end up at
        // the k-th smallest sequence
        let mut sorted_seq = seq[..ct].to_vec();
        sorted_seq.sort();
        for by_id in [true, false] {
            if by_id && ids.len() != ct {
                continue;
            }
            let mut pool: Vec<Option<CardId>> = ms.iter().map(|&c| Some(c)).collect();
            let mut result = Vec::with_capacity(ct);
            let mut ok = true;
            for &s in &seq[..ct] {
                let rank = sorted_seq.iter().position(|&x| x == s).unwrap();
                let hit = pool.iter().position(|c| {
                    c.is_some_and(|c| if by_id { self.card(c).cardid == ids[rank] } else { self.card(c).data.code == codes[rank] })
                });
                let Some(i) = hit else {
                    ok = false;
                    break;
                };
                result.push(pool[i].take().unwrap());
            }
            if ok {
                ms.copy_from_slice(&result);
                return;
            }
        }
        self.ext.state.errors += 1;
    }

    /// `ocgext::is_forced_activatable`: whether the cost/target checks of `peffect` are skipped for
    /// `playerid`.
    pub fn ext_is_forced_activatable(&self, peffect: EffectId, playerid: u8) -> bool {
        let st = &self.ext.state;
        if st.forced_activatable.is_empty() {
            return false;
        }
        let Some(h) = peffect.get_handler_opt(self) else { return false };
        st.forced_activatable.contains(&(playerid, self.card(h).data.code, self.effect(peffect).description))
    }

    /// `ocgext::note_forced_used`: a mandatory trigger only the forcing made activatable.
    pub fn ext_note_forced_used(&mut self, peffect: EffectId) {
        let Some(h) = peffect.get_handler_opt(self) else { return };
        if self.ext.state.forced_used.len() < 1024 {
            let key = (self.card(h).data.code, self.effect(peffect).description);
            self.ext.state.forced_used.insert(key);
        }
    }

    /// The forced result of a card filter on `c` (`forced_matching` of the patch): `Some(true)`
    /// matches, `Some(false)` does not, `None` evaluates the filter.
    pub fn ext_forced_matching(&self, c: CardId) -> Option<bool> {
        let st = &self.ext.state;
        if st.forced_match.is_empty() && st.forced_nomatch.is_empty() {
            return None;
        }
        let id = self.card(c).cardid;
        if st.forced_match.contains(&id) {
            Some(true)
        } else if st.forced_nomatch.contains(&id) {
            Some(false)
        } else {
            None
        }
    }

    /// Whether `viewer` cannot see which card `c` is (piles, face-down cards, the other player's
    /// private hand cards; the deck order is unknown to both players).
    fn ext_hidden_from(&self, c: CardId, viewer: u8) -> bool {
        let cs = &self.card(c).current;
        let loc = cs.location as u32;
        if loc & LOCATION_OVERLAY != 0 {
            return false;
        }
        let own = cs.controler == viewer;
        let facedown = cs.position as u32 & POS_FACEDOWN != 0;
        match loc {
            LOCATION_DECK => true,
            LOCATION_HAND => !own && cs.position as u32 & POS_FACEUP == 0,
            LOCATION_EXTRA | LOCATION_MZONE | LOCATION_SZONE | LOCATION_REMOVED => !own && facedown,
            _ => false,
        }
    }

    /// `ocgext::log_filter_read`: a card filter was evaluated on `c` (the read log, if on).
    pub fn ext_log_filter_read(&mut self, c: CardId, result: bool) {
        let st = &self.ext.state;
        if st.read_viewer < 0 || st.read_log.len() >= 4096 {
            return;
        }
        if self.ext_hidden_from(c, st.read_viewer as u8) {
            let id = self.card(c).cardid;
            self.ext.state.read_log.push((id, result as u8));
        }
    }

    /// `duel::note_read`: the game read property `what` of the card (the property read log).
    #[inline]
    pub fn note_read(&self, c: CardId, what: u32) {
        if self.ext.prop_read_on && !self.querying {
            let id = self.card(c).cardid;
            *self.ext.prop_read_log.0.lock().unwrap().entry(id).or_insert(0) |= what;
        }
    }

    // --- API (`OCG_Ext*`) ---------------------------------------------------------------------

    /// `OCG_ExtPushForcedInts` (kind: `FORCED_COIN` or `FORCED_DICE`)
    pub fn ext_push_forced_ints(&mut self, kind: usize, values: &[i32]) {
        if kind < FORCED_KIND_COUNT {
            self.ext.state.forced_ints[kind].extend(values.iter().copied());
        }
    }
    /// `OCG_ExtPushForcedShuffle`
    pub fn ext_push_forced_shuffle(&mut self, playerid: u8, location: u32, codes: &[u32]) {
        self.ext_push_forced_shuffle_by_id(playerid, location, &[], codes);
    }
    /// `OCG_ExtPushForcedShuffleById`: the result as unique card ids (which physical copy goes
    /// where), bottom first; the codes are used if the pile does not hold exactly these cards.
    pub fn ext_push_forced_shuffle_by_id(&mut self, playerid: u8, location: u32, ids: &[u32], codes: &[u32]) {
        let Some(idx) = shuffle_index(location) else { return };
        if playerid > 1 {
            return;
        }
        self.ext.state.forced_shuffles[playerid as usize][idx].push_back((ids.to_vec(), codes.to_vec()));
    }
    /// `OCG_ExtPushForcedRandomSelect`
    pub fn ext_push_forced_random_select(&mut self, locs: &[ForcedLoc]) {
        self.ext.state.forced_random_selects.push_back(locs.to_vec());
    }
    /// `OCG_ExtPushForcedSetCardShuffle`
    pub fn ext_push_forced_set_card_shuffle(&mut self, codes: &[u32]) {
        self.ext.state.forced_set_card_shuffles.push_back((Vec::new(), codes.to_vec()));
    }
    /// `OCG_ExtPushForcedSetCardShuffleById`
    pub fn ext_push_forced_set_card_shuffle_by_id(&mut self, ids: &[u32], codes: &[u32]) {
        self.ext.state.forced_set_card_shuffles.push_back((ids.to_vec(), codes.to_vec()));
    }
    /// `OCG_ExtForceActivatable`: effects of cards with `code` and description `desc` count as
    /// activatable for `playerid` even if their cost or target check fails.
    pub fn ext_force_activatable(&mut self, playerid: u8, code: u32, desc: u64) {
        self.ext.state.forced_activatable.insert((playerid, code, desc));
    }
    /// `OCG_ExtClearForceActivatable`
    pub fn ext_clear_force_activatable(&mut self) {
        self.ext.state.forced_activatable.clear();
    }
    /// `OCG_ExtSetForcedMatch`: until changed again, these cards match every card filter.
    pub fn ext_set_forced_match(&mut self, ids: &[u32]) {
        self.ext.state.forced_match = ids.iter().copied().collect();
    }
    /// `OCG_ExtSetForcedNoMatch`: until changed again, these cards match no card filter.
    pub fn ext_set_forced_nomatch(&mut self, ids: &[u32]) {
        self.ext.state.forced_nomatch = ids.iter().copied().collect();
    }
    /// `OCG_ExtSetReadLog`: the filter read log on for `viewer` (off with -1).
    pub fn ext_set_read_log(&mut self, viewer: i32) {
        self.ext.state.read_viewer = viewer;
        self.ext.state.read_log.clear();
    }
    /// `OCG_ExtTakeForcedUsed`: (card code, effect description) of the mandatory triggers only the
    /// forcing made activatable since the last call.
    pub fn ext_take_forced_used(&mut self) -> Vec<(u32, u64)> {
        std::mem::take(&mut self.ext.state.forced_used).into_iter().collect()
    }
    /// `OCG_ExtTakeReadLog`: card ids and results (1 = matched) of the filter read log.
    pub fn ext_take_read_log(&mut self) -> Vec<(u32, u8)> {
        std::mem::take(&mut self.ext.state.read_log)
    }
    /// `OCG_ExtClearForced`
    pub fn ext_clear_forced(&mut self) {
        self.ext.state = ExtState::default();
    }
    /// `OCG_ExtPendingForced`: queued forced entries not consumed yet.
    pub fn ext_pending_forced(&self) -> u32 {
        let st = &self.ext.state;
        let mut n = st.forced_random_selects.len() + st.forced_set_card_shuffles.len();
        n += st.forced_ints.iter().map(|q| q.len()).sum::<usize>();
        n += st.forced_shuffles.iter().flatten().map(|q| q.len()).sum::<usize>();
        n as u32
    }
    /// `OCG_ExtGetErrors`: forced entries that could not be applied.
    pub fn ext_get_errors(&self) -> u32 {
        self.ext.state.errors
    }

    /// `OCG_ExtSwapCards`: two cards (by unique id) of the same pile of the same player trade
    /// places, keeping their own history; two face-down cards in their zones trade zones.
    pub fn ext_swap_cards(&mut self, id_a: u32, id_b: u32) -> bool {
        let (Some(a), Some(b)) = (self.card_by_id(id_a), self.card_by_id(id_b)) else { return false };
        if a == b {
            return false;
        }
        let (ca, cb) = (self.card(a), self.card(b));
        if ca.overlay_target().is_some() || cb.overlay_target().is_some() {
            return false;
        }
        if ca.owner != cb.owner || ca.current.controler != cb.current.controler || ca.current.location != cb.current.location {
            return false;
        }
        let loc = ca.current.location as u32;
        let p = ca.current.controler as usize;
        let (sa, sb) = (ca.current.sequence as usize, cb.current.sequence as usize);
        if loc & (LOCATION_MZONE | LOCATION_SZONE) != 0 {
            if self.card(a).is_position(POS_FACEDOWN) == 0
                || self.card(b).is_position(POS_FACEDOWN) == 0
                || ca.equiping_target().is_some()
                || cb.equiping_target().is_some()
                || !ca.equiping_cards.is_empty()
                || !cb.equiping_cards.is_empty()
            {
                return false;
            }
            let zones = if loc & LOCATION_MZONE != 0 { &mut self.player[p].list_mzone } else { &mut self.player[p].list_szone };
            if sa >= zones.len() || sb >= zones.len() || zones[sa] != Some(a) || zones[sb] != Some(b) {
                return false;
            }
            zones.swap(sa, sb);
        } else {
            if !is_sequenced_list(loc) {
                return false;
            }
            let list = self.pile_mut(p as u8, loc as u8);
            if sa >= list.len() || sb >= list.len() || list[sa] != a || list[sb] != b {
                return false;
            }
            list.swap(sa, sb);
        }
        self.card_mut(a).current.sequence = sb as u32;
        self.card_mut(b).current.sequence = sa as u32;
        true
    }

    /// `OCG_ExtSetLocationOrder`: a sequenced location (deck, hand, face-down Extra Deck, GY,
    /// banished) reordered so the card at sequence i has code `codes[i]` (deck: 0 is the bottom).
    pub fn ext_set_location_order(&mut self, playerid: u8, location: u32, codes: &[u32]) -> bool {
        if playerid > 1 || !is_sequenced_list(location) || location.count_ones() != 1 {
            return false;
        }
        let len = self.pile_mut(playerid, location as u8).len();
        let count = self.ext_reorderable_count(playerid, location, len);
        if !self.ext_reorder_by_code(playerid, location as u8, count, codes) {
            return false;
        }
        self.reset_sequence(playerid, location as u8);
        true
    }
    /// `OCG_ExtSetLocationOrderById`
    pub fn ext_set_location_order_by_id(&mut self, playerid: u8, location: u32, ids: &[u32]) -> bool {
        if playerid > 1 || !is_sequenced_list(location) || location.count_ones() != 1 {
            return false;
        }
        let len = self.pile_mut(playerid, location as u8).len();
        let count = self.ext_reorderable_count(playerid, location, len);
        if !self.ext_reorder_by_id(playerid, location as u8, count, ids) {
            return false;
        }
        self.reset_sequence(playerid, location as u8);
        true
    }

    /// `OCG_ExtGetLocationCards`: (code, unique id) of every slot ((0, 0) for an empty zone).
    pub fn ext_get_location_cards(&self, playerid: u8, location: u32) -> Vec<(u32, u32)> {
        let Some(list) = self.ext_location_list(playerid, location) else { return Vec::new() };
        list.iter().map(|c| c.map_or((0, 0), |c| (self.card(c).data.code, self.card(c).cardid))).collect()
    }
    /// `OCG_ExtGetOverlayCards`: the same for the Xyz materials of the card in a zone.
    pub fn ext_get_overlay_cards(&self, playerid: u8, location: u32, sequence: u32) -> Vec<(u32, u32)> {
        if location & (LOCATION_MZONE | LOCATION_SZONE) == 0 {
            return Vec::new();
        }
        let Some(list) = self.ext_location_list(playerid, location) else { return Vec::new() };
        let Some(Some(c)) = list.get(sequence as usize) else { return Vec::new() };
        self.card(*c).xyz_materials.iter().map(|&m| (self.card(m).data.code, self.card(m).cardid)).collect()
    }
    /// `OCG_ExtGetAllCards`: every slot of both players: (player, location, sequence, material
    /// index + 1 (0 for the slot itself; its materials follow it), code, unique id).
    pub fn ext_get_all_cards(&self) -> Vec<[u32; 6]> {
        let mut out = Vec::new();
        for p in 0..2u8 {
            for loc in ALL_LOCATIONS {
                let list = self.ext_location_list(p, loc).unwrap();
                for (i, slot) in list.iter().enumerate() {
                    let entry = |c: Option<CardId>, m: u32| [p as u32, loc, i as u32, m, c.map_or(0, |c| self.card(c).data.code), c.map_or(0, |c| self.card(c).cardid)];
                    out.push(entry(*slot, 0));
                    if let Some(c) = slot {
                        if loc & (LOCATION_MZONE | LOCATION_SZONE) != 0 {
                            for (m, &mat) in self.card(*c).xyz_materials.iter().enumerate() {
                                out.push(entry(Some(mat), m as u32 + 1));
                            }
                        }
                    }
                }
            }
        }
        out
    }
    /// `OCG_ExtChainRelated`: per link of the current chain, whether its card is still the object
    /// that was activated.
    pub fn ext_chain_related(&self) -> Vec<bool> {
        self.core
            .current_chain
            .iter()
            .map(|ch| {
                let Some(te) = ch.triggering_effect else { return false };
                te.get_handler_opt(self).is_some_and(|h| h.is_has_relation_chain(self, te, ch.chain_id))
            })
            .collect()
    }
    /// `OCG_ExtDebugPendingTriggers`: (forced, handler code, description, handler controller,
    /// location, sequence) of the pending trigger chains (`new_fchain`, then `new_ochain`).
    pub fn ext_pending_triggers(&self) -> Vec<[u64; 6]> {
        let dump = |ch: &crate::duel::Chain, forced: u64| {
            let te = ch.triggering_effect;
            let h = te.and_then(|e| e.get_handler_opt(self));
            [
                forced,
                h.map_or(0, |h| self.card(h).data.code as u64),
                te.map_or(0, |e| self.effect(e).description),
                h.map_or(0, |h| self.card(h).current.controler as u64),
                h.map_or(0, |h| self.card(h).current.location as u64),
                h.map_or(0, |h| self.card(h).current.sequence as u64),
            ]
        };
        self.core.new_fchain.iter().map(|ch| dump(ch, 1)).chain(self.core.new_ochain.iter().map(|ch| dump(ch, 0))).collect()
    }
    /// `OCG_ExtSelectChains`: (code, description) of the effects the current selection is about.
    pub fn ext_select_chains(&self) -> Vec<(u32, u64)> {
        self.core
            .select_chains
            .iter()
            .map(|ch| {
                let te = ch.triggering_effect;
                let h = te.and_then(|e| e.get_handler_opt(self));
                (h.map_or(0, |h| self.card(h).data.code), te.map_or(0, |e| self.effect(e).description))
            })
            .collect()
    }

    /// `OCG_ExtPlaceCard`: moves the card with this id to (con, location, sequence) in `position`,
    /// as if it had always been there (silently: no messages or events). Sequence for the deck: 0
    /// top, 1 bottom; location 0 takes it out of play.
    pub fn ext_place_card(&mut self, id: u32, con: u8, location: u32, sequence: u32, position: u32) -> bool {
        let Some(pcard) = self.card_by_id(id) else { return false };
        if con > 1 || (location == 0 && self.card(pcard).current.location == 0 && self.card(pcard).overlay_target().is_none()) {
            return false;
        }
        if location == LOCATION_MZONE || location == LOCATION_SZONE {
            let zones = if location == LOCATION_MZONE { &self.player[con as usize].list_mzone } else { &self.player[con as usize].list_szone };
            if sequence as usize >= zones.len() || zones[sequence as usize].is_some_and(|c| c != pcard) {
                return false;
            }
        }
        let cur = &self.card(pcard).current;
        if cur.location as u32 == location && cur.controler == con && location & LOCATION_ONFIELD != 0 && cur.sequence == sequence {
            self.card_mut(pcard).current.position = position;  // (only its position)
            pcard.refresh_disable_status(self);
            return true;
        }
        // (no shuffle of its own: a forced shuffle order is meant for the real game's next one)
        let deck_check = self.core.shuffle_deck_check;
        let hand_check = self.core.shuffle_hand_check;
        let ok = self.ext_place_card_inner(pcard, con, location, sequence, position);
        self.core.shuffle_deck_check = deck_check;
        self.core.shuffle_hand_check = hand_check;
        ok
    }
    fn ext_place_card_inner(&mut self, pcard: CardId, con: u8, location: u32, sequence: u32, position: u32) -> bool {
        if let Some(t) = self.card(pcard).overlay_target() {
            t.xyz_remove(self, pcard);
        } else if self.card(pcard).current.location != 0 {
            pcard.enable_field_effect(self, false);
            if self.card(pcard).equiping_target().is_some() {
                pcard.unequip(self);
            }
            for eq in self.card(pcard).equiping_cards.clone().iter() {
                eq.unequip(self);
            }
            for mat in self.card(pcard).xyz_materials.clone() {
                pcard.xyz_remove(self, mat);
                let owner = self.card(mat).owner;
                self.add_card(owner, mat, LOCATION_GRAVE as u8, 0, false);
                self.card_mut(mat).current.position = POS_FACEUP;
            }
            if self.card(pcard).current.location as u32 & LOCATION_ONFIELD != 0 {
                pcard.reset(self, RESET_LEAVE, RESET_EVENT);
            }
            pcard.clear_card_target(self);
            self.remove_card(pcard);
        }
        if location == 0 {
            return true;  // (out of play: a token the viewer did not see)
        }
        self.card_mut(pcard).sendto_param.position = position as u8;
        let pzone = location == LOCATION_SZONE
            && self.card(pcard).data.type_ & TYPE_PENDULUM != 0
            && position & POS_FACEUP != 0
            && (sequence == 0 || sequence == 4);
        self.add_card(con, pcard, location as u8, sequence as u8, pzone);
        if self.card(pcard).current.location == 0 {
            return false;
        }
        // (a hand card has the position the core gives one: POS_FACEDOWN, as a drawn card)
        let pos = if location == LOCATION_HAND {
            POS_FACEDOWN
        } else if location == LOCATION_DECK || (location == LOCATION_EXTRA && position & POS_FACEUP == 0) {
            POS_FACEDOWN_DEFENSE
        } else {
            position
        };
        self.card_mut(pcard).current.position = pos;
        let reset = match location {
            LOCATION_HAND => RESET_TOHAND,
            LOCATION_DECK | LOCATION_EXTRA => RESET_TODECK,
            LOCATION_GRAVE => RESET_TOGRAVE,
            LOCATION_REMOVED => RESET_REMOVE,
            _ => RESET_TOFIELD,
        };
        pcard.reset(self, reset, RESET_EVENT);
        pcard.enable_field_effect(self, true);
        pcard.refresh_disable_status(self);
        true
    }

    /// `OCG_ExtCreateCard`: a new card out of play (e.g. a token to place); returns its id.
    pub fn ext_create_card(&mut self, owner: u8, code: u32) -> u32 {
        let c = self.new_card(code);
        let card = self.card_mut(c);
        card.owner = owner;
        card.current.location = 0;
        card.current.controler = owner;
        card.cardid
    }
    /// `OCG_ExtSetCardInfo`: the card's reason (unless `u32::MAX`) and its status bits in `mask`.
    pub fn ext_set_card_info(&mut self, id: u32, reason: u32, status: u32, mask: u32) -> bool {
        let Some(c) = self.card_by_id(id) else { return false };
        let card = self.card_mut(c);
        if reason != u32::MAX {
            card.current.reason = reason;
        }
        card.status = (card.status & !mask) | (status & mask);
        true
    }
    /// `OCG_ExtSetTurn`
    pub fn ext_set_turn(&mut self, turn: u32) {
        self.infos.turn_id = turn as i16;
    }
    /// `OCG_ExtSetLP`
    pub fn ext_set_lp(&mut self, playerid: u8, lp: i32) {
        if playerid < 2 {
            self.player[playerid as usize].lp = lp.max(0);
        }
    }
    /// `OCG_ExtRestartCommand`: at an Idle or Battle command prompt, drops the prompt and lets the
    /// phase compute it again (the next process sends the new prompt). False at another prompt.
    pub fn ext_restart_command(&mut self) -> bool {
        if self.core.units.len() < 2 || !self.core.subunits.is_empty() {
            return false;
        }
        let ok = match (&self.core.units[0], &self.core.units[1]) {
            (Unit::SelectIdleCmd(_), Unit::IdleCommand(p)) => p.step == 1,
            (Unit::SelectBattleCmd(_), Unit::BattleCommand(p)) => p.step == 1,
            _ => false,
        };
        if !ok {
            return false;
        }
        self.core.units.pop_front();
        *self.core.units[0].step_mut() = 0;
        true
    }
    /// `OCG_ExtGetInfo`: turn, turn player, phase, LP of both players.
    pub fn ext_get_info(&self) -> [u32; 5] {
        [self.infos.turn_id as u32, self.infos.turn_player as u32, self.infos.phase as u32, self.player[0].lp as u32, self.player[1].lp as u32]
    }
    /// `OCG_ExtReadLog`: the property read log on or off; returns (card id, mask of the properties
    /// the game read: code 1, type 2, Level 4, Rank 8, attribute 16, race 32, ATK 64, DEF 128,
    /// archetypes 256, Link 512, affected-by checks 2048, count limits 4096) since the last call,
    /// clearing it. Reads while answering queries are not logged.
    pub fn ext_read_log(&mut self, enable: bool) -> Vec<(u32, u32)> {
        self.ext.prop_read_on = enable;
        std::mem::take(&mut *self.ext.prop_read_log.0.lock().unwrap()).into_iter().collect()
    }
}
