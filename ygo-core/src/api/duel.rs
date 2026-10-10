//! `Duel.*` (`libduel.cpp`). Functions that hand work to the core (Lua: yield) run the queued
//! units right away (`run_pushed_units`) and then read the result, as the Lua continuation does.
//! A card or a group argument is `impl Into<CardSet>`.

use crate::constants::*;
use crate::duel::Duel;
use crate::lua::check_action_permission;
use crate::types::*;
use crate::units;

fn flag_code(code: u32) -> u32 {
    (code & 0xfffffff) | 0x10000000
}
fn turn_reset(reset: u32) -> u32 {
    if reset & RESET_PHASE != 0 && reset & (RESET_SELF_TURN | RESET_OPPO_TURN) == 0 { reset | RESET_SELF_TURN | RESET_OPPO_TURN } else { reset }
}
fn result_i32(d: &mut Duel) -> i32 {
    d.run_pushed_units();
    d.returns.at_i32(0)
}

pub fn enable_global_flag(_d: &mut Duel, _flag: u32) {}
pub fn get_lp(d: &Duel, p: u8) -> i32 {
    if p > 1 { 0 } else { d.player[p as usize].lp }
}
pub fn set_lp(d: &mut Duel, p: u8, lp: i32) {
    let lp = lp.max(0);
    if p > 1 {
        return;
    }
    d.player[p as usize].lp = lp;
    let m = d.new_message(MSG_LPUPDATE);
    d.msg(m).u8(p).u32(lp as u32);
}
pub fn get_turn_player(d: &Duel) -> u8 {
    d.infos.turn_player
}
pub fn is_turn_player(d: &Duel, p: u8) -> bool {
    d.infos.turn_player == p
}
/// `Duel.GetTurnCount(player)`: the turns of a player, or (no player) all turns.
pub fn get_turn_count(d: &Duel, p: Option<u8>) -> i32 {
    match p {
        Some(p) if p > 1 => 0,
        Some(p) => d.infos.turn_id_by_player[p as usize] as i32,
        None => d.infos.turn_id as i32,
    }
}
pub fn get_draw_count(d: &mut Duel, p: u8) -> i32 {
    if p > 1 { 0 } else { d.get_draw_count(p) }
}
/// `Duel.RegisterEffect(e, player)`
pub fn register_effect(d: &mut Duel, e: EffectId, p: u8) {
    if p > 1 {
        return;
    }
    d.add_effect(e, p);
}
/// `Duel.RegisterFlagEffect(player, code, reset, flag, count, label)`
pub fn register_flag_effect(d: &mut Duel, p: u8, code: u32, reset: u32, flag: u32, count: u16, label: u32) -> Option<EffectId> {
    if p > 1 {
        return None;
    }
    let code = flag_code(code);
    let count = if count == 0 { 1 } else { count };
    let tc = d.temp_card;
    let pe = d.new_effect();
    let e = d.effect_mut(pe);
    e.effect_owner = p;
    e.owner = Some(tc);
    e.handler = None;
    e.type_ = EFFECT_TYPE_FIELD as u16;
    e.code = code;
    e.reset_flag = turn_reset(reset);
    e.flag[0] = flag | EFFECT_FLAG_CANNOT_DISABLE | EFFECT_FLAG_PLAYER_TARGET | EFFECT_FLAG_FIELD_ONLY;
    e.s_range = 1;
    e.o_range = 0;
    e.reset_count = count;
    e.label.push(label as i64);
    d.add_effect(pe, p);
    Some(pe)
}
pub fn get_flag_effect(d: &mut Duel, p: u8, code: u32) -> usize {
    if p > 1 {
        return 0;
    }
    let mut eset = Vec::new();
    d.filter_player_effect(p, flag_code(code), &mut eset, true);
    eset.len()
}
pub fn reset_flag_effect(d: &mut Duel, p: u8, code: u32) {
    if p > 1 {
        return;
    }
    let code = flag_code(code);
    for e in d.field_effects.aura_effect.range(code) {
        if d.effect_exists(e) && d.effect(e).code == code && e.is_target_player(d, p) {
            d.remove_effect(e);
        }
    }
}
pub fn set_flag_effect_label(d: &mut Duel, p: u8, code: u32, lab: i32) -> bool {
    if p > 1 {
        return false;
    }
    let mut eset = Vec::new();
    d.filter_player_effect(p, flag_code(code), &mut eset, true);
    match eset.first() {
        None => false,
        Some(&e) => {
            d.effect_mut(e).label = vec![lab as i64];
            true
        }
    }
}
/// `Duel.GetFlagEffectLabel(player, code)` (empty: Lua nil)
pub fn get_flag_effect_label(d: &mut Duel, p: u8, code: u32) -> Vec<i64> {
    if p > 1 {
        return Vec::new();
    }
    let mut eset = Vec::new();
    d.filter_player_effect(p, flag_code(code), &mut eset, true);
    eset.iter().map(|&e| d.effect(e).label.first().copied().unwrap_or(0)).collect()
}
/// `Duel.Destroy(targets, reason, dest = LOCATION_GRAVE, reason_player)`
pub fn destroy(d: &mut Duel, targets: impl Into<CardSet>, reason: u32, dest: Option<u32>, reason_player: Option<u8>) -> i32 {
    check_action_permission(d);
    let dest = dest.unwrap_or(LOCATION_GRAVE) as u16;
    let rp = reason_player.unwrap_or(d.core.reason_player);
    let re = d.core.reason_effect;
    d.destroy(targets.into(), re, reason, rp, PLAYER_NONE as u8, dest, 0);
    result_i32(d)
}
/// `Duel.Remove(targets, pos, reason, player = PLAYER_NONE, reason_player)`
pub fn remove(d: &mut Duel, targets: impl Into<CardSet>, pos: u8, reason: u32, playerid: Option<u8>, reason_player: Option<u8>) -> i32 {
    check_action_permission(d);
    let p = playerid.unwrap_or(PLAYER_NONE as u8);
    let rp = reason_player.unwrap_or(d.core.reason_player);
    let re = d.core.reason_effect;
    d.send_to(targets.into(), re, reason, rp, p, LOCATION_REMOVED as u16, 0, pos, false);
    result_i32(d)
}
/// `Duel.SendtoGrave(targets, reason, player = PLAYER_NONE, reason_player)`
pub fn send_to_grave(d: &mut Duel, targets: impl Into<CardSet>, reason: u32, playerid: Option<u8>, reason_player: Option<u8>) -> i32 {
    check_action_permission(d);
    let p = playerid.unwrap_or(PLAYER_NONE as u8);
    let rp = reason_player.unwrap_or(d.core.reason_player);
    let re = d.core.reason_effect;
    d.send_to(targets.into(), re, reason, rp, p, LOCATION_GRAVE as u16, 0, POS_FACEUP as u8, false);
    result_i32(d)
}
/// A summon started during a chain waits until the chain is resolved (`core.reserved`).
fn reserve_summon(d: &mut Duel, pcard: Option<CardId>, proc_group_type: Option<u32>) {
    if !d.core.current_chain.is_empty() {
        let u = d.core.subunits.pop_back().unwrap();
        d.core.reserved = Some(u);
        match proc_group_type {
            Some(t) => d.core.summoning_proc_group_type = t,
            None => d.core.summoning_card = pcard,
        }
    }
    d.run_pushed_units();
}
/// `Duel.Summon(player, c, ignore_count, e, min_tribute = 0, zone = 0x1f)`
pub fn summon(d: &mut Duel, p: u8, c: CardId, ignore_count: bool, e: Option<EffectId>, min_tribute: u8, zone: Option<u32>) {
    check_action_permission(d);
    if d.core.effect_damage_step != 0 || p > 1 {
        return;
    }
    d.core.summon_cancelable = 0;
    d.summon(p, c, e, ignore_count, min_tribute, zone.unwrap_or(0x1f));
    reserve_summon(d, Some(c), None);
}
/// `Duel.SpecialSummonRule(player, c, sumtype = 0)`
pub fn special_summon_rule(d: &mut Duel, p: u8, c: CardId, sumtype: u32) {
    check_action_permission(d);
    if d.core.effect_damage_step != 0 || p > 1 {
        return;
    }
    d.core.summon_cancelable = 0;
    d.special_summon_rule(p, c, sumtype);
    reserve_summon(d, Some(c), None);
}
/// `spsummon_rule`: Synchro/Xyz/Link/procedure Summon with forced materials.
#[allow(clippy::too_many_arguments)]
fn spsummon_rule(d: &mut Duel, p: u8, c: CardId, summon_type: u32, must: Option<CardSet>, materials: Option<CardSet>, minc: u16, maxc: u16) {
    check_action_permission(d);
    if d.core.effect_damage_step != 0 || p > 1 {
        return;
    }
    d.core.must_use_mats = must;
    d.core.only_use_mats = materials;
    d.core.forced_summon_minc = minc as i32;
    d.core.forced_summon_maxc = maxc as i32;
    d.core.summon_cancelable = 0;
    d.special_summon_rule(p, c, summon_type);
    reserve_summon(d, Some(c), None);
}
pub fn synchro_summon(d: &mut Duel, p: u8, c: CardId, must: Option<CardSet>, materials: Option<CardSet>, minc: u16, maxc: u16) {
    spsummon_rule(d, p, c, SUMMON_TYPE_SYNCHRO, must, materials, minc, maxc)
}
pub fn xyz_summon(d: &mut Duel, p: u8, c: CardId, must: Option<CardSet>, materials: Option<CardSet>, minc: u16, maxc: u16) {
    spsummon_rule(d, p, c, SUMMON_TYPE_XYZ, must, materials, minc, maxc)
}
pub fn link_summon(d: &mut Duel, p: u8, c: CardId, must: Option<CardSet>, materials: Option<CardSet>, minc: u16, maxc: u16) {
    spsummon_rule(d, p, c, SUMMON_TYPE_LINK, must, materials, minc, maxc)
}
#[allow(clippy::too_many_arguments)]
pub fn procedure_summon(d: &mut Duel, p: u8, c: CardId, sumtype: u32, must: Option<CardSet>, materials: Option<CardSet>, minc: u16, maxc: u16) {
    spsummon_rule(d, p, c, sumtype, must, materials, minc, maxc)
}
fn spsummon_rule_group(d: &mut Duel, p: u8, summon_type: u32) {
    d.core.summon_cancelable = 0;
    d.special_summon_rule_group(p, summon_type);
    reserve_summon(d, None, Some(summon_type));
}
pub fn pendulum_summon(d: &mut Duel, p: u8) {
    spsummon_rule_group(d, p, SUMMON_TYPE_PENDULUM)
}
pub fn procedure_summon_group(d: &mut Duel, p: u8, sumtype: u32) {
    spsummon_rule_group(d, p, sumtype)
}
/// `Duel.MSet(player, c, ignore_count, e, min_tribute = 0, zone = 0x1f)`
pub fn m_set(d: &mut Duel, p: u8, c: CardId, ignore_count: bool, e: Option<EffectId>, min_tribute: u8, zone: Option<u32>) {
    check_action_permission(d);
    if d.core.effect_damage_step != 0 || p > 1 {
        return;
    }
    d.core.summon_cancelable = 0;
    d.mset(p, c, e, ignore_count, min_tribute, zone.unwrap_or(0x1f));
    reserve_summon(d, Some(c), None);
}
/// `Duel.SSet(player, targets, toplayer = player, confirm = true)`
pub fn s_set(d: &mut Duel, p: u8, targets: impl Into<CardSet>, toplayer: Option<u8>, confirm: bool) -> i32 {
    check_action_permission(d);
    if p > 1 {
        return 0;
    }
    let mut tp = toplayer.unwrap_or(p);
    if tp > 1 {
        tp = p;
    }
    let g = targets.into();
    if g.is_empty() {
        return d.returns.at_i32(0);
    }
    let gr = d.new_group(g);
    let re = d.core.reason_effect;
    d.emplace_process(units::SpellSetGroup { step: 0, setplayer: p, toplayer: tp, confirm, ptarget: Some(gr), reason_effect: re, set_cards: CardSet::new() });
    result_i32(d)
}
/// `Duel.CreateToken(player, code)`
pub fn create_token(d: &mut Duel, p: u8, code: u32) -> Option<CardId> {
    check_action_permission(d);
    if p as u32 >= PLAYER_NONE {
        return None;
    }
    let c = d.new_card(code);
    let cc = d.card_mut(c);
    cc.owner = p;
    cc.current.location = 0;
    cc.current.controler = p;
    Some(c)
}
/// `Duel.SpecialSummon(targets, sumtype, sumplayer, target_player, nocheck, nolimit, pos, zone = 0xff)`
#[allow(clippy::too_many_arguments)]
pub fn special_summon(d: &mut Duel, targets: impl Into<CardSet>, sumtype: u32, sumplayer: u8, playerid: u8, nocheck: bool, nolimit: bool, positions: u8, zone: Option<u32>) -> i32 {
    check_action_permission(d);
    if sumplayer as u32 >= PLAYER_NONE {
        return 0;
    }
    d.special_summon(targets.into(), sumtype, sumplayer, playerid, nocheck, nolimit, positions, zone.unwrap_or(0xff));
    result_i32(d)
}
#[allow(clippy::too_many_arguments)]
pub fn special_summon_step(d: &mut Duel, c: CardId, sumtype: u32, sumplayer: u8, playerid: u8, nocheck: bool, nolimit: bool, positions: u8, zone: Option<u32>) -> bool {
    check_action_permission(d);
    if sumplayer as u32 >= PLAYER_NONE {
        return false;
    }
    d.special_summon_step(c, sumtype, sumplayer, playerid, nocheck, nolimit, positions, zone.unwrap_or(0xff));
    result_i32(d) != 0
}
pub fn special_summon_complete(d: &mut Duel) -> i32 {
    check_action_permission(d);
    if d.core.special_summoning.is_empty() && d.core.ss_tograve_set.is_empty() {
        d.core.operated_set.clear();
        return 0;
    }
    let (re, rp) = (d.core.reason_effect, d.core.reason_player);
    d.special_summon_complete(re, rp);
    result_i32(d)
}
/// `Duel.SendtoHand(targets, player = PLAYER_NONE, reason, reason_player)`
pub fn send_to_hand(d: &mut Duel, targets: impl Into<CardSet>, playerid: Option<u8>, reason: u32, reason_player: Option<u8>) -> i32 {
    check_action_permission(d);
    let p = playerid.unwrap_or(PLAYER_NONE as u8);
    if p as u32 > PLAYER_NONE {
        return 0;
    }
    let rp = reason_player.unwrap_or(d.core.reason_player);
    let re = d.core.reason_effect;
    d.send_to(targets.into(), re, reason, rp, p, LOCATION_HAND as u16, 0, POS_FACEUP as u8, false);
    result_i32(d)
}
/// `Duel.SendtoDeck(targets, player = PLAYER_NONE, seq, reason, reason_player)` (seq -2: back to the
/// location it was sent from... the Extra Deck for Extra Deck monsters: location 0)
pub fn send_to_deck(d: &mut Duel, targets: impl Into<CardSet>, playerid: Option<u8>, sequence: i32, reason: u32, reason_player: Option<u8>) -> i32 {
    check_action_permission(d);
    let p = playerid.unwrap_or(PLAYER_NONE as u8);
    if p as u32 > PLAYER_NONE {
        return 0;
    }
    let rp = reason_player.unwrap_or(d.core.reason_player);
    let location = if sequence == -2 { 0 } else { LOCATION_DECK as u16 };
    let re = d.core.reason_effect;
    d.send_to(targets.into(), re, reason, rp, p, location, sequence as u32, POS_FACEUP as u8, false);
    result_i32(d)
}
/// `Duel.SendtoExtraP(targets, player = PLAYER_NONE, reason, reason_player)`
pub fn send_to_extra_p(d: &mut Duel, targets: impl Into<CardSet>, playerid: Option<u8>, reason: u32, reason_player: Option<u8>) -> i32 {
    check_action_permission(d);
    let p = playerid.unwrap_or(PLAYER_NONE as u8);
    if p as u32 > PLAYER_NONE {
        return 0;
    }
    let rp = reason_player.unwrap_or(d.core.reason_player);
    let re = d.core.reason_effect;
    d.send_to(targets.into(), re, reason, rp, p, LOCATION_EXTRA as u16, 0, POS_FACEUP as u8, false);
    result_i32(d)
}
/// `Duel.Sendto(targets, location, reason, pos = POS_FACEUP, player = PLAYER_NONE, seq = 0, reason_player)`
#[allow(clippy::too_many_arguments)]
pub fn send_to(d: &mut Duel, targets: impl Into<CardSet>, location: u16, reason: u32, pos: Option<u8>, playerid: Option<u8>, sequence: u32, reason_player: Option<u8>) -> i32 {
    check_action_permission(d);
    let p = playerid.unwrap_or(PLAYER_NONE as u8);
    if p as u32 > PLAYER_NONE {
        return 0;
    }
    let rp = reason_player.unwrap_or(d.core.reason_player);
    let re = d.core.reason_effect;
    d.send_to(targets.into(), re, reason, rp, p, location, sequence, pos.unwrap_or(POS_FACEUP as u8), true);
    result_i32(d)
}
/// `Duel.RemoveCards(targets)`: takes cards out of the duel.
pub fn remove_cards(d: &mut Duel, targets: impl Into<CardSet>) {
    check_action_permission(d);
    let g: CardSet = targets.into();
    let order = crate::card::public_order(d, &g);
    let mut tot = order.len();
    let mut cur = tot.min(255);
    let mut m = d.new_message(MSG_REMOVE_CARDS);
    d.msg(m).u32(cur as u32);
    for &c in &order {
        let loc = c.get_info_location(d);
        d.msg(m).loc(loc);
        tot -= 1;
        cur -= 1;
        if cur == 0 && tot > 0 {
            m = d.new_message(MSG_REMOVE_CARDS);
            cur = tot.min(255);
            d.msg(m).u32(cur as u32);
        }
        c.enable_field_effect(d, false);
        c.cancel_field_effect(d);
    }
    for c in order {
        d.remove_card(c);
    }
}
pub fn get_operated_group(d: &Duel) -> Group {
    d.core.operated_set.clone()
}
/// `Duel.IsCanAddCounter(player)` / `Duel.IsCanAddCounter(player, countertype, count, c)`
pub fn is_can_add_counter(d: &mut Duel, p: u8, counter: Option<(u16, u32, CardId)>) -> bool {
    if p > 1 {
        return false;
    }
    match counter {
        None => d.is_player_can_action(p, EFFECT_CANNOT_PLACE_COUNTER),
        Some((ct, count, c)) => d.is_player_can_place_counter(p, c, ct, count as u16),
    }
}
/// `Duel.RemoveCounter(player, s, o, countertype, count, reason)`
pub fn remove_counter(d: &mut Duel, rplayer: u8, s: u8, o: u8, countertype: u16, count: u32, reason: u32) -> bool {
    check_action_permission(d);
    if rplayer > 1 {
        return false;
    }
    d.remove_counter_op(reason, None, rplayer as u32, s, o, countertype as u32, count);
    result_i32(d) != 0
}
pub fn is_can_remove_counter(d: &mut Duel, rplayer: u8, s: u8, o: u8, countertype: u16, count: u32, reason: u32) -> bool {
    if rplayer > 1 {
        return false;
    }
    d.is_player_can_remove_counter(rplayer, None, s, o, countertype, count as u16, reason)
}
pub fn get_counter(d: &Duel, p: u8, s: u8, o: u8, countertype: u16) -> u32 {
    if p > 1 { 0 } else { d.get_field_counter(p, s, o, countertype) }
}
/// `Duel.ChangePosition(targets, au, ad = au, du = au, dd = au, noflip = false)`
#[allow(clippy::too_many_arguments)]
pub fn change_position(d: &mut Duel, targets: impl Into<CardSet>, au: u8, ad: Option<u8>, du: Option<u8>, dd: Option<u8>, noflip: bool) -> i32 {
    check_action_permission(d);
    let flag = if noflip { NO_FLIP_EFFECT } else { 0 };
    let (re, rp) = (d.core.reason_effect, d.core.reason_player);
    d.change_position(targets.into(), re, rp, au, ad.unwrap_or(au), du.unwrap_or(au), dd.unwrap_or(au), flag, true);
    result_i32(d)
}
/// `Duel.Release(targets, reason, reason_player)`
pub fn release(d: &mut Duel, targets: impl Into<CardSet>, reason: u32, reason_player: Option<u8>) -> i32 {
    check_action_permission(d);
    let rp = reason_player.unwrap_or(d.core.reason_player);
    let re = d.core.reason_effect;
    d.release(targets.into(), re, reason, rp);
    result_i32(d)
}
/// `Duel.MoveToField(c, move_player, target_player, dest, pos, enable, zone = 0xff)`
#[allow(clippy::too_many_arguments)]
pub fn move_to_field(d: &mut Duel, c: CardId, move_player: u8, playerid: u8, destination: u16, positions: u8, enable: bool, zone: Option<u32>) -> bool {
    check_action_permission(d);
    if playerid > 1 {
        return false;
    }
    c.enable_field_effect(d, false);
    d.adjust_instant();
    d.move_to_field(c, move_player, playerid, destination, positions, enable, 0, zone.unwrap_or(0xff) as u8, false, units::LocationReason::None, true);
    result_i32(d) != 0
}
/// `Duel.ReturnToField(c, pos = previous position, zone = 0xff)`
pub fn return_to_field(d: &mut Duel, c: CardId, pos: Option<u8>, zone: Option<u32>) -> bool {
    check_action_permission(d);
    if d.card(c).current.reason & REASON_TEMPORARY == 0 {
        return false;
    }
    let prev = d.card(c).previous;
    let pos = pos.unwrap_or(prev.position as u8);
    c.enable_field_effect(d, false);
    d.adjust_instant();
    d.refresh_location_info_instant();
    d.move_to_field(c, prev.controler, prev.controler, prev.location as u16, pos, true, 1, zone.unwrap_or(0xff) as u8, false, units::LocationReason::ReturnToField, true);
    result_i32(d) != 0
}
/// `Duel.MoveSequence(c, seq, location = current location)`
pub fn move_sequence(d: &mut Duel, c: CardId, seq: u8, location: Option<u32>) -> bool {
    check_action_permission(d);
    let cur = d.card(c).current;
    let cur_loc = cur.location as u32;
    let mut location = location.unwrap_or(cur_loc) & 0xffff;
    let mut seq = seq;
    let mut pzone = false;
    let mut fzone = false;
    if location == LOCATION_PZONE {
        if !d.is_flag(DUEL_PZONE) {
            crate::lua::lua_error("LOCATION_PZONE passed with pendulum zones disabled");
        }
        pzone = true;
        location = LOCATION_SZONE;
    } else if location == LOCATION_FZONE {
        fzone = true;
        location = LOCATION_SZONE;
    }
    let playerid = cur.controler;
    if location != cur_loc {
        crate::lua::lua_error("The new location doesn't match the current one");
    }
    if pzone {
        if seq > 1 {
            crate::lua::lua_error("Invalid sequence");
        }
        seq = d.get_pzone_index(seq, playerid);
    } else if fzone {
        if seq != 0 {
            crate::lua::lua_error("Invalid sequence");
        }
        seq = 5;
    } else if location == LOCATION_MZONE && d.is_flag(DUEL_EMZONE) {
        if seq > 6 {
            crate::lua::lua_error("Invalid sequence");
        }
    } else if (location == LOCATION_MZONE || location == LOCATION_SZONE) && seq > 4 {
        crate::lua::lua_error("Invalid sequence");
    }
    let re = d.core.reason_effect;
    let mut res = false;
    if c.is_affect_by_effect(d, re) {
        let prev_loc = c.get_info_location(d);
        let prev_code = d.card(c).data.code;
        res = d.move_card(playerid, c, cur.location, seq, pzone);
        if res {
            if cur.pzone != pzone {
                let to = c.get_info_location(d);
                let r = d.card(c).current.reason;
                let m = d.new_message(MSG_MOVE);
                d.msg(m).u32(prev_code).loc(prev_loc).loc(to).u32(r);
            }
            let rp = d.core.reason_player;
            d.raise_single_event(c, None, EVENT_MOVE, re, 0, rp, playerid, 0);
            d.raise_event_card(Some(c), EVENT_MOVE, re, 0, rp, playerid, 0);
            d.process_single_event();
            d.process_instant_event();
        }
    }
    res
}
pub fn swap_sequence(d: &mut Duel, c1: CardId, c2: CardId) {
    check_action_permission(d);
    let a = d.card(c1).current;
    let b = d.card(c2).current;
    let (player, location) = (a.controler, a.location as u32);
    let re = d.core.reason_effect;
    if b.controler == player && (location == LOCATION_MZONE || location == LOCATION_SZONE) && b.location as u32 == location && c1.is_affect_by_effect(d, re) && c2.is_affect_by_effect(d, re) {
        d.swap_card(c1, c2);
        let rp = d.core.reason_player;
        d.raise_single_event(c1, None, EVENT_MOVE, re, 0, rp, player, 0);
        d.raise_single_event(c2, None, EVENT_MOVE, re, 0, rp, player, 0);
        d.raise_event(CardSet::from_iter([c1, c2]), EVENT_MOVE, re, 0, rp, player, 0);
        d.process_single_event();
        d.process_instant_event();
    }
}
/// `Duel.Activate(e)` (the activation runs after the current function)
pub fn activate(d: &mut Duel, e: EffectId) {
    check_action_permission(d);
    d.emplace_process(units::ActivateEffect { step: 0, peffect: e });
}
pub fn set_chain_limit(d: &mut Duel, f: LuaFn) {
    let p = d.core.reason_player as i32;
    d.core.chain_limit.push(crate::duel::ChainLimit { function: f, player: p });
}
pub fn set_chain_limit_till_chain_end(d: &mut Duel, f: LuaFn) {
    let p = d.core.reason_player as i32;
    d.core.chain_limit_p.push(crate::duel::ChainLimit { function: f, player: p });
}
pub fn get_chain_material(d: &mut Duel, p: u8) -> Option<EffectId> {
    if p > 1 {
        return None;
    }
    let mut eset = Vec::new();
    d.filter_player_effect(p, EFFECT_CHAIN_MATERIAL, &mut eset, true);
    eset.first().copied()
}
/// `Duel.ConfirmDecktop(player, count)`: the confirmed cards.
pub fn confirm_decktop(d: &mut Duel, p: u8, count: u32) -> Group {
    if p > 1 {
        return CardSet::new();
    }
    let n = d.player[p as usize].list_main.len();
    let mut count = count as usize;
    if count >= n {
        count = n;
    } else if d.core.deck_reversed {
        let pc = d.player[p as usize].list_main[n - 1 - count];
        let (code, pos) = (d.card(pc).data.code, d.card(pc).current.position);
        let m = d.new_message(MSG_DECK_TOP);
        d.msg(m).u8(p).u32(count as u32).u32(code).u32(pos);
    }
    let m = d.new_message(MSG_CONFIRM_DECKTOP);
    d.msg(m).u8(p).u32(count as u32);
    for i in 0..count {
        let pc = d.player[p as usize].list_main[n - 1 - i];
        let cc = d.card(pc);
        let (code, ctrl, loc, seq) = (cc.data.code, cc.current.controler, cc.current.location, cc.current.sequence);
        d.msg(m).u32(code).u8(ctrl).u8(loc).u32(seq);
    }
    d.run_pushed_units();
    let main = &d.player[p as usize].list_main;
    let count = count.min(main.len());
    CardSet::from_iter(main[main.len() - count..].iter().copied())
}
/// `Duel.ConfirmExtratop(player, count)`: the confirmed cards (face-down Extra Deck, from the top).
pub fn confirm_extratop(d: &mut Duel, p: u8, count: u32) -> Group {
    if p > 1 {
        return CardSet::new();
    }
    let pl = &d.player[p as usize];
    let n = pl.list_extra.len();
    let epc = pl.extra_p_count as usize;
    let max_count = (n - epc) as u32;
    let count = count.min(max_count) as usize;
    let cards: Vec<CardId> = (0..count).map(|i| pl.list_extra[n - 1 - epc - i]).collect();
    let m = d.new_message(MSG_CONFIRM_EXTRATOP);
    d.msg(m).u8(p).u32(count as u32);
    for &pc in &cards {
        let cc = d.card(pc);
        let (code, ctrl, loc, seq) = (cc.data.code, cc.current.controler, cc.current.location, cc.current.sequence);
        d.msg(m).u32(code).u8(ctrl).u8(loc).u32(seq);
    }
    d.run_pushed_units();
    CardSet::from_iter(cards)
}
/// `Duel.ConfirmCards(player, targets)`
pub fn confirm_cards(d: &mut Duel, p: u8, targets: impl Into<CardSet>) {
    if p > 1 {
        return;
    }
    let g: CardSet = targets.into();
    if g.is_empty() {
        return;
    }
    let m = d.new_message(MSG_CONFIRM_CARDS);
    d.msg(m).u8(p).u32(g.len() as u32);
    // EVENT_CONFIRM when a card is revealed, EVENT_TOHAND_CONFIRM when a card in the hand is
    let reason = if d.core.chain_solving { REASON_EFFECT } else { REASON_COST };
    let trigeff = d.core.reason_effect;
    let revealing = 1 - p;
    let was_during_to_hand = d.check_event(EVENT_TO_HAND, None);
    let mut handgroup = CardSet::new();
    let rp = d.core.reason_player;
    for c in crate::card::public_order(d, &g) {
        let cc = d.card(c);
        let (code, ctrl, loc, seq) = (cc.data.code, cc.current.controler, cc.current.location, cc.current.sequence);
        d.msg(m).u32(code).u8(ctrl).u8(loc).u32(seq);
        d.raise_single_event(c, None, EVENT_CONFIRM, trigeff, reason, rp, revealing, 0);
        if was_during_to_hand && d.card(c).current.location as u32 == LOCATION_HAND {
            d.raise_single_event(c, None, EVENT_TOHAND_CONFIRM, trigeff, reason, rp, revealing, 0);
            handgroup.insert(c);
        }
    }
    d.raise_event(g, EVENT_CONFIRM, trigeff, reason, rp, revealing, 0);
    if !handgroup.is_empty() {
        d.raise_event(handgroup, EVENT_TOHAND_CONFIRM, trigeff, reason, rp, revealing, 0);
    }
    d.process_single_event();
    d.process_instant_event();
    d.run_pushed_units();
}
fn sort_deck(d: &mut Duel, sort_player: u8, target_player: u8, count: u16, bottom: bool) {
    check_action_permission(d);
    if sort_player > 1 || target_player > 1 || !(1..=64).contains(&count) {
        return;
    }
    d.emplace_process(units::SortDeck { step: 0, count, sort_player, target_player, bottom });
    d.run_pushed_units();
}
pub fn sort_decktop(d: &mut Duel, sort_player: u8, target_player: u8, count: u16) {
    sort_deck(d, sort_player, target_player, count, false)
}
pub fn sort_deckbottom(d: &mut Duel, sort_player: u8, target_player: u8, count: u16) {
    sort_deck(d, sort_player, target_player, count, true)
}
/// `Duel.CheckEvent(code, get_info)`: the event if it happened (`Some`).
pub fn check_event(d: &Duel, code: u32) -> Option<crate::duel::Event> {
    let mut pe = crate::duel::Event::default();
    if d.check_event(code, Some(&mut pe)) { Some(pe) } else { None }
}
/// `Duel.RaiseEvent(targets, code, re, r, rp, ep, ev)`
#[allow(clippy::too_many_arguments)]
pub fn raise_event(d: &mut Duel, targets: impl Into<CardSet>, code: u32, re: Option<EffectId>, r: u32, rp: u8, ep: u8, ev: u32) {
    check_action_permission(d);
    d.raise_event(targets.into(), code, re, r, rp, ep, ev);
    d.process_instant_event();
    d.run_pushed_units();
}
/// `Duel.RaiseSingleEvent(c, code, re, r, rp, ep, ev)`
#[allow(clippy::too_many_arguments)]
pub fn raise_single_event(d: &mut Duel, c: CardId, code: u32, re: Option<EffectId>, r: u32, rp: u8, ep: u8, ev: u32) {
    check_action_permission(d);
    d.raise_single_event(c, None, code, re, r, rp, ep, ev);
    d.process_single_event();
    d.run_pushed_units();
}
pub fn check_timing(d: &Duel, tm: u32) -> bool {
    d.core.hint_timing[0] & tm != 0 || d.core.hint_timing[1] & tm != 0
}
fn env_enabled(d: &Duel, c: Option<CardId>) -> bool {
    c.is_some_and(|c| {
        let cc = d.card(c);
        cc.is_position(POS_FACEUP) != 0 && cc.get_status(STATUS_EFFECT_ENABLED) != 0
    })
}
/// `Duel.GetEnvironment()`: (code, player)
pub fn get_environment(d: &mut Duel) -> (u32, u8) {
    for p in 0..2 {
        let f = d.player[p].list_szone[5];
        if env_enabled(d, f) {
            let c = f.unwrap();
            let code = c.get_code(d);
            return (code, d.card(c).current.controler);
        }
    }
    let mut eset = Vec::new();
    d.filter_field_effect(EFFECT_CHANGE_ENVIRONMENT, &mut eset, true);
    match eset.last() {
        Some(&pe) => (pe.get_value(d, 0) as u32, pe.get_handler_player(d)),
        None => (0, 2),
    }
}
/// `Duel.IsEnvironment(code, player = PLAYER_ALL, location = LOCATION_FZONE + LOCATION_ONFIELD)`
pub fn is_environment(d: &mut Duel, code: u32, playerid: Option<u8>, loc: Option<u32>) -> bool {
    let playerid = playerid.unwrap_or(PLAYER_ALL as u8);
    let loc = loc.unwrap_or(LOCATION_FZONE + LOCATION_ONFIELD) & 0xffff;
    if playerid > 1 && playerid as u32 != PLAYER_ALL {
        return false;
    }
    let applies = |p: u8| playerid == p || playerid as u32 == PLAYER_ALL;
    let check_zone = |d: &mut Duel, p: u8, cards: Vec<Option<CardId>>| -> bool {
        if !applies(p) {
            return false;
        }
        for c in cards {
            if env_enabled(d, c) && code == c.unwrap().get_code(d) {
                return true;
            }
        }
        false
    };
    if loc & LOCATION_FZONE != 0 {
        for p in 0..2u8 {
            let f = vec![d.player[p as usize].list_szone[5]];
            if check_zone(d, p, f) {
                return true;
            }
        }
    }
    if loc & LOCATION_SZONE != 0 {
        for p in 0..2u8 {
            let z = d.player[p as usize].list_szone.clone();
            if check_zone(d, p, z) {
                return true;
            }
        }
    }
    if loc & LOCATION_MZONE != 0 {
        for p in 0..2u8 {
            let z = d.player[p as usize].list_mzone.clone();
            if check_zone(d, p, z) {
                return true;
            }
        }
    }
    let apply_change_env = loc & (LOCATION_FZONE | LOCATION_SZONE) != 0 && !(env_enabled(d, d.player[0].list_szone[5]) || env_enabled(d, d.player[1].list_szone[5]));
    if apply_change_env {
        let mut eset = Vec::new();
        d.filter_field_effect(EFFECT_CHANGE_ENVIRONMENT, &mut eset, true);
        if let Some(&pe) = eset.last() {
            let hp = pe.get_handler_player(d);
            if (playerid as u32 == PLAYER_ALL || playerid == hp) && code == pe.get_value(d, 0) as u32 {
                return true;
            }
        }
    }
    false
}
/// `Duel.Win(player, reason)`
pub fn win(d: &mut Duel, playerid: u8, reason: u32) {
    let mut playerid = playerid;
    if playerid > 2 {
        return;
    }
    let cannot_lose = |d: &mut Duel, p: u8| d.is_player_affected_by_effect(p, EFFECT_CANNOT_LOSE_EFFECT).is_some();
    if playerid == 0 {
        if cannot_lose(d, 1) {
            return;
        }
    } else if playerid == 1 {
        if cannot_lose(d, 0) {
            return;
        }
    } else {
        let (c0, c1) = (cannot_lose(d, 0), cannot_lose(d, 1));
        if c0 && c1 {
            return;
        } else if c0 {
            playerid = 0;
        } else if c1 {
            playerid = 1;
        }
    }
    if d.core.win_player == 5 {
        d.core.win_player = playerid;
        d.core.win_reason = reason as u8;
    } else if (d.core.win_player == 0 && playerid == 1) || (d.core.win_player == 1 && playerid == 0) {
        d.core.win_player = 2;
        d.core.win_reason = reason as u8;
    }
}
/// `Duel.Draw(player, count, reason)`
pub fn draw(d: &mut Duel, p: u8, count: u32, reason: u32) -> i32 {
    check_action_permission(d);
    if p > 1 {
        return 0;
    }
    let (re, rp) = (d.core.reason_effect, d.core.reason_player);
    d.draw(re, reason, rp, p, count as u16);
    result_i32(d)
}
/// `Duel.Damage(player, amount, reason, is_step = false, reason_player)`
pub fn damage(d: &mut Duel, p: u8, amount: i64, reason: u32, is_step: bool, reason_player: Option<u8>) -> u32 {
    check_action_permission(d);
    if p > 1 {
        return 0;
    }
    let amount = if amount > 0 { amount as u32 } else { 0 };
    let mut rp = reason_player.unwrap_or(d.core.reason_player);
    if rp as u32 > PLAYER_NONE {
        rp = d.core.reason_player;
    }
    let re = d.core.reason_effect;
    d.damage(re, reason, rp, None, p, amount, is_step);
    d.run_pushed_units();
    d.returns.at_u32(0)
}
/// `Duel.Recover(player, amount, reason, is_step = false, reason_player)`
pub fn recover(d: &mut Duel, p: u8, amount: i64, reason: u32, is_step: bool, reason_player: Option<u8>) -> u32 {
    check_action_permission(d);
    if p > 1 {
        return 0;
    }
    let amount = if amount > 0 { amount as u32 } else { 0 };
    let mut rp = reason_player.unwrap_or(d.core.reason_player);
    if rp as u32 > PLAYER_NONE {
        rp = d.core.reason_player;
    }
    let re = d.core.reason_effect;
    d.recover(re, reason, rp as u32, p as u32, amount, is_step);
    d.run_pushed_units();
    d.returns.at_u32(0)
}
/// `Duel.RDComplete()`: the damage/recovery steps queued with `is_step` happen now.
pub fn rd_complete(d: &mut Duel) {
    let r = std::mem::take(&mut d.core.recover_damage_reserve);
    d.core.subunits.extend(r);
    d.run_pushed_units();
}
/// `Duel.Equip(player, equip_card, target, faceup = true, is_step = false)`
pub fn equip(d: &mut Duel, p: u8, equip_card: CardId, target: CardId, up: bool, step: bool) -> bool {
    check_action_permission(d);
    if p > 1 {
        return false;
    }
    d.equip_op(p, equip_card, target, up, step);
    result_i32(d) != 0
}
pub fn equip_complete(d: &mut Duel) {
    let eqs = d.core.equiping_cards.clone();
    let mut etargets = CardSet::new();
    for c in crate::card::public_order(d, &eqs) {
        if d.card(c).is_position(POS_FACEUP) != 0 {
            c.enable_field_effect(d, true);
        }
        if let Some(t) = d.card(c).equiping_target {
            etargets.insert(t);
        }
    }
    if etargets.is_empty() {
        return;
    }
    d.adjust_instant();
    let (re, rp) = (d.core.reason_effect, d.core.reason_player);
    for t in crate::card::public_order(d, &etargets) {
        d.raise_single_event(t, Some(&eqs), EVENT_EQUIP, re, 0, rp, PLAYER_NONE as u8, 0);
    }
    d.raise_event(eqs, EVENT_EQUIP, re, 0, rp, PLAYER_NONE as u8, 0);
    d.core.hint_timing[0] |= TIMING_EQUIP;
    d.core.hint_timing[1] |= TIMING_EQUIP;
    d.process_single_event();
    d.process_instant_event();
    d.run_pushed_units();
}
/// `Duel.GetControl(targets, player, reset_phase = 0, reset_count = 0, zone = 0xff, chose_player)`
#[allow(clippy::too_many_arguments)]
pub fn get_control(d: &mut Duel, targets: impl Into<CardSet>, p: u8, reset_phase: u16, reset_count: u8, zone: Option<u32>, chose_player: Option<u8>) -> i32 {
    check_action_permission(d);
    if p > 1 {
        return 0;
    }
    let chose = chose_player.unwrap_or(d.core.reason_player);
    let re = d.core.reason_effect;
    d.get_control(targets.into(), re, chose, p, reset_phase & 0x3ff, reset_count, zone.unwrap_or(0xff));
    result_i32(d)
}
/// `Duel.SwapControl(targets1, targets2, reset_phase = 0, reset_count = 0)`
pub fn swap_control(d: &mut Duel, targets1: impl Into<CardSet>, targets2: impl Into<CardSet>, reset_phase: u16, reset_count: u8) -> bool {
    check_action_permission(d);
    let (re, rp) = (d.core.reason_effect, d.core.reason_player);
    d.swap_control(re, rp as u32, targets1.into(), targets2.into(), (reset_phase & 0x3ff) as u32, reset_count as u32);
    result_i32(d) != 0
}
pub fn check_lp_cost(d: &mut Duel, p: u8, cost: u32) -> bool {
    p <= 1 && d.check_lp_cost(p, cost)
}
pub fn pay_lp_cost(d: &mut Duel, p: u8, cost: u32) {
    check_action_permission(d);
    if p > 1 {
        return;
    }
    d.emplace_process(units::PayLPCost { step: 0, playerid: p, cost });
    d.run_pushed_units();
}
/// `Duel.DiscardDeck(player, count, reason)`
pub fn discard_deck(d: &mut Duel, p: u8, count: u16, reason: u32) -> i32 {
    check_action_permission(d);
    d.emplace_process(units::DiscardDeck { step: 0, count, playerid: p, reason });
    result_i32(d)
}
/// `Duel.DiscardHand(player, filter, min, max, reason, ex)`
#[allow(clippy::too_many_arguments)]
pub fn discard_hand(d: &mut Duel, p: u8, f: Option<super::Filter>, min: u16, max: u16, reason: u32, ex: super::Exclude) -> i32 {
    check_action_permission(d);
    let mut g = CardSet::new();
    let (exc, exg) = match ex {
        super::Exclude::None => (None, None),
        super::Exclude::Card(c) => (Some(c), None),
        super::Exclude::Group(g) => (None, Some(g)),
    };
    let pf = f.map(|f| move |d: &mut Duel, c: CardId| super::check_matching(d, f, c));
    let filter: crate::field2::CardFilter = pf.as_ref().map(|x| x as &dyn Fn(&mut Duel, CardId) -> bool);
    d.filter_matching_card(filter, p, LOCATION_HAND, 0, Some(&mut g), exc, exg, None, 0, false);
    d.core.select_cards = g.to_vec();
    if d.core.select_cards.is_empty() {
        return 0;
    }
    d.emplace_process(units::DiscardHand { step: 0, playerid: p, min: min as u8, max: max as u8, reason });
    result_i32(d)
}
pub fn disable_shuffle_check(d: &mut Duel, disable: bool) {
    d.core.shuffle_check_disabled = disable;
}
pub fn disable_self_destroy_check(d: &mut Duel, disable: bool) {
    d.core.selfdes_disabled = disable;
}
fn shuffle_loc(d: &mut Duel, p: u8, loc: u32) {
    if p > 1 {
        return;
    }
    d.shuffle(p, loc as u8);
    d.run_pushed_units();
}
pub fn shuffle_deck(d: &mut Duel, p: u8) {
    shuffle_loc(d, p, LOCATION_DECK)
}
pub fn shuffle_extra(d: &mut Duel, p: u8) {
    shuffle_loc(d, p, LOCATION_EXTRA)
}
pub fn shuffle_hand(d: &mut Duel, p: u8) {
    shuffle_loc(d, p, LOCATION_HAND)
}
/// `Duel.ShuffleSetCard(g)`: shuffles face-down cards among their zones.
pub fn shuffle_set_card(d: &mut Duel, g: &Group) {
    if g.is_empty() || g.len() > 5 {
        return;
    }
    let order = crate::card::public_order(d, g); // (not creation order: visible)
    let first = d.card(order[0]).current;
    let loc = first.location;
    if loc as u32 != LOCATION_MZONE && loc as u32 != LOCATION_SZONE {
        return;
    }
    let tp = first.controler;
    if tp > 1 {
        return;
    }
    let mut ms: Vec<CardId> = Vec::with_capacity(5);
    let mut seq: Vec<u8> = Vec::with_capacity(5);
    for &c in &order {
        let cur = d.card(c).current;
        if cur.location != loc || cur.position & POS_FACEUP != 0 || cur.sequence > 4 || cur.controler != tp {
            return;
        }
        ms.push(c);
        seq.push(cur.sequence as u8);
    }
    let ct = ms.len();
    for i in (1..ct).rev() {
        let s = d.get_next_integer(0, i as i32) as usize;
        ms.swap(i, s);
    }
    let m = d.new_message(MSG_SHUFFLE_SET_CARD);
    d.msg(m).u8(loc).u8(ct as u8);
    for &c in &order {
        let l = c.get_info_location(d);
        d.msg(m).loc(l);
    }
    for i in 0..ct {
        let c = ms[i];
        let list = if loc as u32 == LOCATION_MZONE { &mut d.player[tp as usize].list_mzone } else { &mut d.player[tp as usize].list_szone };
        list[seq[i] as usize] = Some(c);
        d.card_mut(c).current.sequence = seq[i] as u32;
        let cc = d.card(c);
        let (re, r, rp) = (cc.current.reason_effect, cc.current.reason, cc.current.reason_player);
        d.raise_single_event(c, None, EVENT_MOVE, re, r, rp, tp, 0);
    }
    let (re, rp) = (d.core.reason_effect, d.core.reason_player);
    d.raise_event(g.clone(), EVENT_MOVE, re, 0, rp, tp, 0);
    d.process_single_event();
    d.process_instant_event();
    for &c in &order {
        let l = if d.card(c).xyz_materials.is_empty() { LocInfo::default() } else { c.get_info_location(d) };
        d.msg(m).loc(l);
    }
    // (ocgext: the zone each card went to, in the order above; only its owner sees it)
    for &c in &order {
        let s = d.card(c).current.sequence as u8;
        d.msg(m).u8(s);
    }
    d.run_pushed_units();
}
/// `Duel.ChangeAttacker(c, ignore_count = false)`
pub fn change_attacker(d: &mut Duel, new_attacker: CardId, ignore_count: bool) {
    let Some(attacker) = d.core.attacker else { return };
    if attacker == new_attacker {
        return;
    }
    let at = d.core.attack_target;
    let at_info = at.map(|t| (t, d.card(t).fieldid_r));
    {
        let a = d.card_mut(attacker);
        a.announce_count = a.announce_count.wrapping_add(1);
        a.announced_cards.addcard(at_info);
    }
    d.attack_all_target_check();
    d.core.attacker = Some(new_attacker);
    let fid = {
        let a = d.card_mut(new_attacker);
        a.attack_controler = a.current.controler;
        a.fieldid_r
    };
    d.core.pre_field[0] = fid;
    if !ignore_count {
        let in_damage = d.infos.phase as u32 == PHASE_DAMAGE;
        let a = d.card_mut(new_attacker);
        a.attack_announce_count = a.attack_announce_count.wrapping_add(1);
        if in_damage {
            a.attacked_count = a.attacked_count.wrapping_add(1);
            a.attacked_cards.addcard(at_info);
        }
    }
}
/// `Duel.ChangeAttackTarget(target, ignore = false)`
pub fn change_attack_target(d: &mut Duel, target: Option<CardId>, ignore: bool) -> bool {
    let Some(attacker) = d.core.attacker else { return false };
    if !attacker.is_capable_attack(d) || d.card(attacker).is_status(STATUS_ATTACK_CANCELED) {
        return false;
    }
    let mut cv = Vec::new();
    let ca = d.core.chain_attack;
    d.get_attack_target(attacker, &mut cv, ca, true, None);
    let ok = (target.is_some_and(|t| cv.contains(&t)) || ignore) || (target.is_none() && attacker.is_affected_by_effect(d, EFFECT_CANNOT_DIRECT_ATTACK).is_none());
    if !ok {
        return false;
    }
    d.core.attack_target = target;
    d.core.attack_rollback = false;
    d.core.opp_mzone.clear();
    let turnp = d.infos.turn_player;
    let mut fids: Vec<u32> = d.player[1 - turnp as usize].list_mzone.iter().flatten().map(|&c| d.card(c).fieldid_r).collect();
    fids.sort_unstable();
    fids.dedup();
    d.core.opp_mzone = fids;
    let aloc = attacker.get_info_location(d);
    let m = d.new_message(MSG_ATTACK);
    d.msg(m).loc(aloc);
    match target {
        Some(t) => {
            let ep = if d.card(t).current.controler != turnp { turnp } else { 1 - turnp };
            d.raise_single_event(t, None, EVENT_BE_BATTLE_TARGET, None, REASON_REPLACE, 0, ep, 0);
            d.raise_event_card(Some(t), EVENT_BE_BATTLE_TARGET, None, REASON_REPLACE, 0, ep, 0);
            d.process_single_event();
            d.process_instant_event();
            let tl = t.get_info_location(d);
            d.msg(m).loc(tl);
        }
        None => {
            d.core.attack_player = 1;
            d.msg(m).loc(LocInfo::default());
        }
    }
    true
}
pub fn attack_cost_paid(d: &mut Duel, paid: Option<u8>) {
    d.core.attack_cost_paid = paid.unwrap_or(1);
}
pub fn force_attack(d: &mut Duel, attacker: CardId, target: CardId) {
    check_action_permission(d);
    d.core.set_forced_attack = true;
    d.core.forced_attacker = Some(attacker);
    d.core.forced_attack_target = Some(target);
    d.run_pushed_units();
}
/// `Duel.CalculateDamage(attacker, target, new_attack = false)`
pub fn calculate_damage(d: &mut Duel, attacker: CardId, target: Option<CardId>, new_attack: bool) {
    check_action_permission(d);
    if Some(attacker) == target {
        return;
    }
    d.emplace_process(units::DamageStep { step: 0, backup_phase: 0, new_attack, attacker: Some(attacker), attack_target: target, cards_destroyed_by_battle: None });
    d.run_pushed_units();
}
pub fn get_battle_damage(d: &Duel, p: u8) -> i32 {
    if p > 1 { 0 } else { d.core.battle_damage[p as usize] }
}
/// `Duel.ChangeBattleDamage(player, value, check = true)`
pub fn change_battle_damage(d: &mut Duel, p: u8, dam: u32, check: bool) {
    if p > 1 || (check && d.core.battle_damage[p as usize] == 0) {
        return;
    }
    d.core.battle_damage[p as usize] = dam as i32;
}
pub fn change_target_card(d: &mut Duel, count: u8, g: &Group) {
    d.change_target(count, g);
}
pub fn change_target_player(d: &mut Duel, count: u8, p: u8) {
    if p <= 1 {
        d.change_target_player(count, p);
    }
}
pub fn change_target_param(d: &mut Duel, count: u8, param: i32) {
    d.change_target_param(count, param);
}
pub fn break_effect(d: &mut Duel) {
    check_action_permission(d);
    d.break_effect(true);
    d.raise_event_card(None, EVENT_BREAK_EFFECT, None, 0, PLAYER_NONE as u8, PLAYER_NONE as u8, 0);
    d.process_instant_event();
    d.run_pushed_units();
}
pub fn change_chain_operation(d: &mut Duel, count: u8, f: LuaFn) {
    d.change_chain_effect(count, Some(f));
}
pub fn negate_activation(d: &mut Duel, count: u8) -> bool {
    d.negate_chain(count)
}
pub fn negate_effect(d: &mut Duel, count: u8) -> bool {
    d.disable_chain(count)
}
/// `Duel.NegateRelatedChain(c, reset)`
pub fn negate_related_chain(d: &mut Duel, c: CardId, reset_flag: u32) {
    if d.core.current_chain.len() < 2 {
        return;
    }
    let re = d.core.reason_effect;
    if !c.is_affect_by_effect(d, re) {
        return;
    }
    for i in (0..d.core.current_chain.len()).rev() {
        let ch = &d.core.current_chain[i];
        let (te, cid) = (ch.triggering_effect.unwrap(), ch.chain_id);
        if te.get_handler(d) == c && c.is_has_relation_chain(d, te, cid) {
            let owner = re.unwrap().get_handler(d);
            let ne = d.new_effect();
            let e = d.effect_mut(ne);
            e.owner = Some(owner);
            e.type_ = EFFECT_TYPE_SINGLE as u16;
            e.code = EFFECT_DISABLE_CHAIN;
            e.value = cid as i32;
            e.reset_flag = RESET_CHAIN | RESET_EVENT | reset_flag;
            c.add_effect(d, ne);
        }
    }
}
/// `Duel.NegateSummon(targets)`
pub fn negate_summon(d: &mut Duel, targets: impl Into<CardSet>) {
    let g: CardSet = targets.into();
    let mut sumplayer = PLAYER_NONE as u8;
    for c in crate::card::public_order(d, &g) {
        let cc = d.card_mut(c);
        sumplayer = cc.summon.player;
        cc.set_status(STATUS_SUMMONING, false);
        cc.set_status(STATUS_SUMMON_DISABLED, true);
        if cc.summon.type_ & SUMMON_TYPE_PENDULUM != SUMMON_TYPE_PENDULUM {
            cc.set_status(STATUS_PROC_COMPLETE, false);
        }
    }
    let event_code = if d.check_event(EVENT_SUMMON, None) {
        EVENT_SUMMON_NEGATED
    } else if d.check_event(EVENT_FLIP_SUMMON, None) {
        EVENT_FLIP_SUMMON_NEGATED
    } else if d.check_event(EVENT_SPSUMMON, None) {
        EVENT_SPSUMMON_NEGATED
    } else {
        0
    };
    let (re, rp) = (d.core.reason_effect, d.core.reason_player);
    d.raise_event(g, event_code, re, REASON_EFFECT, rp, sumplayer, 0);
    d.process_instant_event();
}
/// `Duel.IncreaseSummonedCount(c)`
pub fn increase_summoned_count(d: &mut Duel, c: Option<CardId>) {
    let p = d.core.reason_player as usize;
    if let Some(c) = c {
        if let Some(pextra) = c.is_affected_by_effect(d, EFFECT_EXTRA_SUMMON_COUNT) {
            pextra.get_value_card(d, Some(c), 0);
            return;
        }
    }
    d.core.summon_count[p] = d.core.summon_count[p].wrapping_add(1);
}
pub fn check_summoned_count(d: &mut Duel, c: Option<CardId>) -> bool {
    let p = d.core.reason_player;
    if c.is_some_and(|c| c.is_affected_by_effect(d, EFFECT_EXTRA_SUMMON_COUNT).is_some()) {
        return true;
    }
    (d.core.summon_count[p as usize] as i32) < d.get_summon_count_limit(p)
}
fn location_reason(r: u32) -> units::LocationReason {
    use units::LocationReason::*;
    match r {
        0x0 => None,
        0x1 => ToField,
        0x2 => Control,
        0x4 => Count,
        0x8 => Return,
        0x9 => ReturnToField,
        0x10 => Summon,
        0x11 => SpSummon,
        _ => Set,
    }
}
/// `Duel.GetLocationCount(player, location, use_player, reason = LOCATION_REASON_TOFIELD, zone = 0xff)`: (count, zones in use)
pub fn get_location_count(d: &mut Duel, p: u8, location: u8, uplayer: Option<u8>, reason: Option<u32>, zone: Option<u32>) -> (i32, u32) {
    if p > 1 {
        return (0, 0);
    }
    let up = uplayer.unwrap_or(d.core.reason_player);
    let mut list = 0u32;
    let n = d.get_useable_count(None, p, location, up, location_reason(reason.unwrap_or(0x1)), zone.unwrap_or(0xff), Some(&mut list));
    (n, list)
}
/// Runs `f` with the monsters in `ex` taken out of the Monster Zones (as `GetMZoneCount` does).
fn without_mzone_cards<T>(d: &mut Duel, ex: super::Exclude, f: impl FnOnce(&mut Duel) -> T) -> T {
    if matches!(ex, super::Exclude::None) {
        return f(d);
    }
    let default_loc = if d.is_flag(DUEL_3_COLUMNS_FIELD) { 0x1111 } else { 0 };
    let mut saved: [(u32, Vec<Option<CardId>>); 2] = Default::default();
    for p in 0..2 {
        let mut used = default_loc;
        let mut digit = 1u32;
        let mut list = Vec::new();
        for &c in &d.player[p].list_mzone {
            match c {
                Some(c) if !ex.contains(c) => {
                    used |= digit;
                    list.push(Some(c));
                }
                _ => list.push(None),
            }
            digit <<= 1;
        }
        used |= d.player[p].used_location & 0xff00;
        let old_used = std::mem::replace(&mut d.player[p].used_location, used);
        let old_list = std::mem::replace(&mut d.player[p].list_mzone, list);
        saved[p] = (old_used, old_list);
    }
    let r = f(d);
    for (p, (u, l)) in saved.into_iter().enumerate() {
        d.player[p].used_location = u;
        d.player[p].list_mzone = l;
    }
    r
}
/// `Duel.GetMZoneCount(player, ex, use_player, reason, zone)`: (count, zones in use)
pub fn get_m_zone_count(d: &mut Duel, p: u8, ex: super::Exclude, uplayer: Option<u8>, reason: Option<u32>, zone: Option<u32>) -> (i32, u32) {
    if p > 1 {
        return (0, 0);
    }
    let up = uplayer.unwrap_or(d.core.reason_player);
    without_mzone_cards(d, ex, |d| {
        let mut list = 0u32;
        let n = d.get_useable_count(None, p, LOCATION_MZONE as u8, up, location_reason(reason.unwrap_or(0x1)), zone.unwrap_or(0xff), Some(&mut list));
        (n, list)
    })
}
/// The card a count from the Extra Deck is for: a card, or a type (a temporary card of that type).
pub enum ExtraCard {
    None,
    Card(CardId),
    Type(u32),
}
/// `Duel.GetLocationCountFromEx(player, use_player, ex, card_or_type, zone = 0xff)`: (count, zones in use)
pub fn get_location_count_from_ex(d: &mut Duel, p: u8, uplayer: Option<u8>, ex: super::Exclude, sc: ExtraCard, zone: Option<u32>) -> (i32, u32) {
    if p > 1 {
        return (0, 0);
    }
    let up = uplayer.unwrap_or(d.core.reason_player);
    without_mzone_cards(d, ex, |d| {
        let (scard, temp) = match sc {
            ExtraCard::None => (None, false),
            ExtraCard::Card(c) => (Some(c), false),
            ExtraCard::Type(t) => {
                let tc = d.temp_card;
                let c = d.card_mut(tc);
                c.current.location = LOCATION_EXTRA as u8;
                c.data.type_ = TYPE_MONSTER | t;
                c.current.position = if t & TYPE_PENDULUM != 0 { POS_FACEUP_DEFENSE } else { POS_FACEDOWN_DEFENSE };
                (Some(tc), true)
            }
        };
        let mut list = 0u32;
        let n = d.get_useable_count_fromex(scard, p, up, zone.unwrap_or(0xff), Some(&mut list));
        if temp {
            let tc = d.temp_card;
            let c = d.card_mut(tc);
            c.current.location = 0;
            c.data.type_ = 0;
            c.current.position = 0;
        }
        (n, list)
    })
}
pub fn get_usable_m_zone_count(d: &mut Duel, p: u8, uplayer: Option<u8>) -> i32 {
    if p > 1 {
        return 0;
    }
    let up = uplayer.unwrap_or(d.core.reason_player);
    let (mut f1, mut f2) = (0u32, 0u32);
    let ct1 = d.get_tofield_count(None, p, LOCATION_MZONE as u8, up as u32, units::LocationReason::ToField, 0xff, Some(&mut f1));
    let ct2 = d.get_spsummonable_count_fromex(None, p, up, 0xff, Some(&mut f2));
    let ct3 = crate::field1::field_used_count(!(f1 | f2) & 0x1f);
    let count = ct1 + ct2 - ct3;
    let limit = d.get_mzone_limit(p, up, units::LocationReason::ToField);
    count.min(limit)
}
fn linked_loc(l: u32) -> u32 {
    let l = l & 0xffff;
    if l == 1 { LOCATION_MZONE } else { l }
}
pub fn get_linked_group(d: &mut Duel, rplayer: u8, location1: u32, location2: u32) -> Group {
    if rplayer > 1 {
        return CardSet::new();
    }
    let mut cset = CardSet::new();
    d.get_linked_cards_field(rplayer, linked_loc(location1) as u8, linked_loc(location2) as u8, &mut cset);
    cset
}
pub fn get_linked_group_count(d: &mut Duel, rplayer: u8, location1: u32, location2: u32) -> usize {
    get_linked_group(d, rplayer, location1, location2).len()
}
pub fn get_linked_zone(d: &mut Duel, p: u8) -> u32 {
    if p > 1 { 0 } else { d.get_linked_zone(p as i32, false, false) }
}
pub fn get_free_linked_zone(d: &mut Duel, p: u8) -> u32 {
    if p > 1 { 0 } else { d.get_linked_zone(p as i32, true, false) }
}
pub fn get_field_card(d: &Duel, p: u8, location: u32, sequence: u32) -> Option<CardId> {
    if p > 1 {
        return None;
    }
    let c = d.get_field_card(p as u32, location & 0xffff, sequence)?;
    if d.card(c).get_status(STATUS_SUMMONING | STATUS_SPSUMMON_STEP) != 0 { None } else { Some(c) }
}
pub fn check_location(d: &Duel, p: u8, location: u32, sequence: u32) -> bool {
    p <= 1 && d.is_location_useable(p as u32, location & 0xffff, sequence)
}
/// `Duel.GetCurrentChain(real = false)`
pub fn get_current_chain(d: &Duel, real: bool) -> i32 {
    if real { d.core.real_chain_count } else { d.core.current_chain.len() as i32 }
}
pub const CHAININFO_TRIGGERING_EFFECT: u32 = 1;
pub const CHAININFO_TRIGGERING_PLAYER: u32 = 2;
pub const CHAININFO_TRIGGERING_CONTROLER: u32 = 3;
pub const CHAININFO_TRIGGERING_LOCATION: u32 = 4;
pub const CHAININFO_TRIGGERING_LOCATION_SYMBOLIC: u32 = 5;
pub const CHAININFO_TRIGGERING_SEQUENCE: u32 = 6;
pub const CHAININFO_TRIGGERING_SEQUENCE_SYMBOLIC: u32 = 7;
pub const CHAININFO_TARGET_CARDS: u32 = 8;
pub const CHAININFO_TARGET_PLAYER: u32 = 9;
pub const CHAININFO_TARGET_PARAM: u32 = 10;
pub const CHAININFO_DISABLE_REASON: u32 = 11;
pub const CHAININFO_DISABLE_PLAYER: u32 = 12;
pub const CHAININFO_CHAIN_ID: u32 = 13;
pub const CHAININFO_TYPE: u32 = 14;
pub const CHAININFO_EXTTYPE: u32 = 15;
pub const CHAININFO_TRIGGERING_POSITION: u32 = 16;
pub const CHAININFO_TRIGGERING_CODE: u32 = 17;
pub const CHAININFO_TRIGGERING_CODE2: u32 = 18;
pub const CHAININFO_TRIGGERING_TYPE: u32 = 19;
pub const CHAININFO_TRIGGERING_LEVEL: u32 = 20;
pub const CHAININFO_TRIGGERING_RANK: u32 = 21;
pub const CHAININFO_TRIGGERING_ATTRIBUTE: u32 = 22;
pub const CHAININFO_TRIGGERING_RACE: u32 = 23;
pub const CHAININFO_TRIGGERING_ATTACK: u32 = 24;
pub const CHAININFO_TRIGGERING_DEFENSE: u32 = 25;
pub const CHAININFO_TRIGGERING_STATUS: u32 = 26;
pub const CHAININFO_TRIGGERING_SUMMON_LOCATION: u32 = 27;
pub const CHAININFO_TRIGGERING_SUMMON_TYPE: u32 = 28;
pub const CHAININFO_TRIGGERING_SUMMON_PROC_COMPLETE: u32 = 29;
pub const CHAININFO_TRIGGERING_SETCODES: u32 = 30;
pub const CHAININFO_TRIGGERING_LINK: u32 = 31;
pub const CHAININFO_TRIGGERING_LSCALE: u32 = 32;
pub const CHAININFO_TRIGGERING_RSCALE: u32 = 33;
/// `Duel.GetChainInfo(ch, flags...)`: one value per flag. Without such a chain link the C++
/// returns no values, which Lua reads as `nil`s: every value is then `Value::Nil`.
pub fn get_chain_info(d: &Duel, chaincount: u8, flags: &[u32]) -> Vec<Value> {
    let Some(r) = d.get_chain(chaincount) else { return vec![Value::Nil; flags.len()] };
    let ch = d.chain_at(r);
    let st = &ch.triggering_state;
    let tl = ch.triggering_location as u32;
    flags
        .iter()
        .map(|&flag| match flag {
            CHAININFO_TRIGGERING_EFFECT => Value::effect(ch.triggering_effect),
            CHAININFO_TRIGGERING_PLAYER => Value::Int(ch.triggering_player as i64),
            CHAININFO_TRIGGERING_CONTROLER => Value::Int(ch.triggering_controler as i64),
            CHAININFO_TRIGGERING_LOCATION => Value::Int((tl & !(LOCATION_STZONE | LOCATION_MMZONE | LOCATION_EMZONE)) as i64),
            CHAININFO_TRIGGERING_LOCATION_SYMBOLIC => Value::Int((tl & !(LOCATION_MZONE | LOCATION_SZONE)) as i64),
            CHAININFO_TRIGGERING_SEQUENCE => Value::Int(ch.triggering_sequence as i64),
            CHAININFO_TRIGGERING_SEQUENCE_SYMBOLIC => {
                let s = ch.triggering_sequence as i64;
                let v = if d.is_flag(DUEL_3_COLUMNS_FIELD) && tl & !(LOCATION_STZONE | LOCATION_MMZONE) != 0 {
                    s + 1
                } else if tl & LOCATION_PZONE != 0 {
                    if ch.triggering_sequence == d.get_pzone_index(0, ch.triggering_controler) as u32 { 0 } else { 1 }
                } else if tl & LOCATION_FZONE != 0 {
                    0
                } else if tl & LOCATION_EMZONE != 0 {
                    s - 5
                } else {
                    s
                };
                Value::Int(v)
            }
            CHAININFO_TRIGGERING_POSITION => Value::Int(ch.triggering_position as i64),
            CHAININFO_TRIGGERING_CODE => Value::Int(st.code as i64),
            CHAININFO_TRIGGERING_CODE2 => Value::Int(st.code2 as i64),
            CHAININFO_TRIGGERING_TYPE => Value::Int(st.type_ as i64),
            CHAININFO_TRIGGERING_LEVEL => Value::Int(st.level as i64),
            CHAININFO_TRIGGERING_RANK => Value::Int(st.rank as i64),
            CHAININFO_TRIGGERING_LSCALE => Value::Int(st.lscale as i64),
            CHAININFO_TRIGGERING_RSCALE => Value::Int(st.rscale as i64),
            CHAININFO_TRIGGERING_LINK => Value::Int(st.link as i64),
            CHAININFO_TRIGGERING_ATTRIBUTE => Value::Int(st.attribute as i64),
            CHAININFO_TRIGGERING_RACE => Value::Int(st.race as i64),
            CHAININFO_TRIGGERING_ATTACK => Value::Int(st.attack as i64),
            CHAININFO_TRIGGERING_DEFENSE => Value::Int(st.defense as i64),
            CHAININFO_TARGET_CARDS => ch.target_cards.clone().map_or(Value::Nil, Value::Group),
            CHAININFO_TARGET_PLAYER => Value::Int(ch.target_player as i64),
            CHAININFO_TARGET_PARAM => Value::Int(ch.target_param as i64),
            CHAININFO_DISABLE_REASON => Value::effect(ch.disable_reason),
            CHAININFO_DISABLE_PLAYER => Value::Int(ch.disable_player as i64),
            CHAININFO_CHAIN_ID => Value::Int(ch.chain_id as i64),
            CHAININFO_TYPE => {
                let ct = d.effect(ch.triggering_effect.unwrap()).card_type & (TYPE_MONSTER | TYPE_SPELL | TYPE_TRAP);
                Value::Int(if ct == TYPE_TRAP | TYPE_MONSTER { TYPE_MONSTER } else { ct } as i64)
            }
            CHAININFO_EXTTYPE => Value::Int(d.effect(ch.triggering_effect.unwrap()).card_type as i64),
            CHAININFO_TRIGGERING_STATUS => Value::Int(ch.triggering_status as i64),
            CHAININFO_TRIGGERING_SUMMON_LOCATION => Value::Int(ch.triggering_summon_location as i64),
            CHAININFO_TRIGGERING_SUMMON_TYPE => Value::Int(ch.triggering_summon_type as i64),
            CHAININFO_TRIGGERING_SUMMON_PROC_COMPLETE => Value::Bool(ch.triggering_summon_proc_complete),
            CHAININFO_TRIGGERING_SETCODES => Value::Multi(st.setcodes.iter().map(|s| Value::Int(s as i64)).collect()),
            _ => crate::lua::lua_error("Passed invalid CHAININFO flag."),
        })
        .collect()
}
/// `Duel.GetChainEvent(ch)`
pub fn get_chain_event(d: &Duel, chaincount: u8) -> Option<crate::duel::Event> {
    d.get_chain(chaincount).map(|r| d.chain_at(r).evt.clone())
}
/// `Duel.GetFirstTarget()`: the targets of the current chain link, in public order.
pub fn get_first_target(d: &Duel) -> Vec<CardId> {
    let Some(r) = d.get_chain(0) else { return Vec::new() };
    match &d.chain_at(r).target_cards {
        Some(t) if !t.is_empty() => crate::card::public_order(d, t),
        _ => Vec::new(),
    }
}
pub fn get_current_phase(d: &Duel) -> u32 {
    d.infos.phase as u32
}
pub fn is_main_phase(d: &Duel) -> bool {
    let p = d.infos.phase as u32;
    p == PHASE_MAIN1 || p == PHASE_MAIN2
}
pub fn is_battle_phase(d: &Duel) -> bool {
    let p = d.infos.phase as u32;
    (PHASE_BATTLE_START..=PHASE_BATTLE).contains(&p)
}
pub fn is_phase(d: &Duel, phase: u32) -> bool {
    d.infos.phase as u32 == phase & 0xffff
}
/// `Duel.SkipPhase(player, phase, reset, count, value = 0)`
pub fn skip_phase(d: &mut Duel, p: u8, phase: u32, reset: u32, count: u16, value: u32) {
    if p > 1 {
        return;
    }
    let count = if count == 0 { 1 } else { count };
    let code = match phase & 0xffff {
        PHASE_DRAW => EFFECT_SKIP_DP,
        PHASE_STANDBY => EFFECT_SKIP_SP,
        PHASE_MAIN1 => EFFECT_SKIP_M1,
        PHASE_BATTLE => EFFECT_SKIP_BP,
        PHASE_MAIN2 => EFFECT_SKIP_M2,
        PHASE_END => EFFECT_SKIP_EP,
        _ => return,
    };
    let tc = d.temp_card;
    let pe = d.new_effect();
    let e = d.effect_mut(pe);
    e.owner = Some(tc);
    e.effect_owner = p;
    e.type_ = EFFECT_TYPE_FIELD as u16;
    e.code = code;
    e.reset_flag = (reset & 0x3ff) | RESET_PHASE | RESET_SELF_TURN;
    e.flag[0] = EFFECT_FLAG_CANNOT_DISABLE | EFFECT_FLAG_PLAYER_TARGET;
    e.s_range = 1;
    e.o_range = 0;
    e.reset_count = count;
    e.value = value as i32;
    d.add_effect(pe, p);
}
pub fn is_attack_cost_paid(d: &Duel) -> u8 {
    d.core.attack_cost_paid
}
pub fn is_damage_calculated(d: &Duel) -> bool {
    d.core.damage_calculated
}
pub fn get_attacker(d: &Duel) -> Option<CardId> {
    d.core.attacker
}
pub fn get_attack_target(d: &Duel) -> Option<CardId> {
    d.core.attack_target
}
/// `Duel.GetBattleMonster(player)`: (the player's monster, the opponent's monster)
pub fn get_battle_monster(d: &Duel, p: u8) -> (Option<CardId>, Option<CardId>) {
    let (a, t) = (d.core.attacker, d.core.attack_target);
    let pick = |p: u8| {
        if a.is_some_and(|a| d.card(a).current.controler == p) {
            a
        } else if t.is_some_and(|t| d.card(t).current.controler == p) {
            t
        } else {
            None
        }
    };
    (pick(p), pick(1u8.wrapping_sub(p)))
}
pub fn negate_attack(d: &mut Duel) -> bool {
    d.emplace_process(units::AttackDisable { step: 0 });
    result_i32(d) != 0
}
/// `Duel.ChainAttack(target)`
pub fn chain_attack(d: &mut Duel, target: Option<CardId>) {
    let Some(attacker) = d.core.attacker else { return };
    let re = d.core.reason_effect;
    if !attacker.is_affect_by_effect(d, re) {
        return;
    }
    d.core.chain_attack = true;
    d.core.chain_attacker_id = d.card(attacker).fieldid;
    if target.is_some() {
        d.core.chain_attack_target = target;
    }
}
pub fn readjust(d: &mut Duel) {
    let adjcard = d.core.reason_effect.unwrap().get_handler(d);
    let n = match d.core.readjust_map.iter_mut().find(|x| x.0 == adjcard) {
        Some(x) => {
            x.1 += 1;
            x.1
        }
        None => {
            d.core.readjust_map.push((adjcard, 1));
            1
        }
    };
    if n > 3 {
        let rp = d.core.reason_player;
        d.send_to(CardSet::from_card(adjcard), None, REASON_RULE, rp, PLAYER_NONE as u8, LOCATION_GRAVE as u16, 0, POS_FACEUP as u8, false);
        d.run_pushed_units();
        return;
    }
    d.core.re_adjust = true;
}
pub fn adjust_instantly(d: &mut Duel, c: Option<CardId>) {
    if let Some(c) = c {
        c.filter_disable_related_cards(d);
    }
    d.adjust_instant();
}
pub fn get_field_group(d: &mut Duel, p: u8, location1: u32, location2: u32) -> Group {
    let mut g = CardSet::new();
    d.filter_field_card(p, location1 & 0xffff, location2 & 0xffff, Some(&mut g));
    g
}
pub fn get_field_group_count(d: &mut Duel, p: u8, location1: u32, location2: u32) -> i32 {
    d.filter_field_card(p, location1 & 0xffff, location2 & 0xffff, None)
}
pub fn get_decktop_group(d: &Duel, p: u8, count: u32) -> Group {
    let main = &d.player[p as usize].list_main;
    let count = (count as usize).min(main.len());
    CardSet::from_iter(main[main.len() - count..].iter().copied())
}
pub fn get_deckbottom_group(d: &Duel, p: u8, count: u32) -> Group {
    let main = &d.player[p as usize].list_main;
    let count = (count as usize).min(main.len());
    CardSet::from_iter(main[..count].iter().copied())
}
pub fn get_extra_top_group(d: &Duel, p: u8, count: u32) -> Group {
    let pl = &d.player[p as usize];
    let n = pl.list_extra.len();
    let epc = pl.extra_p_count as usize;
    let count = (count as usize).min(n - epc);
    CardSet::from_iter((0..count).map(|i| pl.list_extra[n - 1 - epc - i]))
}
fn matching(d: &mut Duel, f: Option<super::Filter>, s: u8, loc1: u32, loc2: u32, ex: super::Exclude, out: Option<&mut CardSet>, pret: Option<&mut Option<CardId>>, fcount: i32) -> bool {
    let (exc, exg) = match ex {
        super::Exclude::None => (None, None),
        super::Exclude::Card(c) => (Some(c), None),
        super::Exclude::Group(g) => (None, Some(g)),
    };
    let pf = f.map(|f| move |d: &mut Duel, c: CardId| super::check_matching(d, f, c));
    let filter: crate::field2::CardFilter = pf.as_ref().map(|x| x as &dyn Fn(&mut Duel, CardId) -> bool);
    d.filter_matching_card(filter, s, loc1 & 0xffff, loc2 & 0xffff, out, exc, exg, pret, fcount, false)
}
/// `Duel.GetMatchingGroup(f, player, s, o, ex)`
pub fn get_matching_group(d: &mut Duel, f: Option<super::Filter>, p: u8, s: u32, o: u32, ex: super::Exclude) -> Group {
    let mut g = CardSet::new();
    matching(d, f, p, s, o, ex, Some(&mut g), None, 0);
    g
}
pub fn get_matching_group_count(d: &mut Duel, f: Option<super::Filter>, p: u8, s: u32, o: u32, ex: super::Exclude) -> usize {
    get_matching_group(d, f, p, s, o, ex).len()
}
pub fn get_first_matching_card(d: &mut Duel, f: Option<super::Filter>, p: u8, s: u32, o: u32, ex: super::Exclude) -> Option<CardId> {
    let mut r = None;
    matching(d, f, p, s, o, ex, None, Some(&mut r), 0);
    r
}
/// `Duel.IsExistingMatchingCard(f, player, s, o, count, ex)`
pub fn is_existing_matching_card(d: &mut Duel, f: Option<super::Filter>, p: u8, s: u32, o: u32, count: u32, ex: super::Exclude) -> bool {
    matching(d, f, p, s, o, ex, None, None, count as i32)
}
/// `Duel.SelectMatchingCard(player, f, s_player, s, o, min, max, cancelable, ex)`: `None` if canceled.
#[allow(clippy::too_many_arguments)]
pub fn select_matching_card(d: &mut Duel, sel_player: u8, f: Option<super::Filter>, p: u8, s: u32, o: u32, min: u16, max: u16, cancelable: bool, ex: super::Exclude) -> Option<Group> {
    check_action_permission(d);
    if sel_player > 1 {
        return None;
    }
    let mut g = CardSet::new();
    matching(d, f, p, s, o, ex, Some(&mut g), None, 0);
    d.core.select_cards = g.to_vec();
    d.emplace_process(units::SelectCard { step: 0, playerid: sel_player, cancelable, min: min as u8, max: max as u8 });
    d.run_pushed_units();
    super::return_cards(d, cancelable)
}
/// `Duel.SelectCardsFromCodes(player, min, max, cancelable, ret_index, codes...)`: the chosen
/// (code, 1-based index) pairs, `None` if canceled.
pub fn select_cards_from_codes(d: &mut Duel, p: u8, min: u16, max: u16, cancelable: bool, codes: &[u32]) -> Option<Vec<(u32, u32)>> {
    check_action_permission(d);
    d.core.select_cards_codes.clear();
    if p > 1 {
        return None;
    }
    d.core.select_cards_codes = codes.iter().enumerate().map(|(i, &c)| (c, i as u32 + 1)).collect();
    d.emplace_process(units::SelectCardCodes { step: 0, playerid: p, cancelable, min: min as u8, max: max as u8 });
    d.run_pushed_units();
    if d.return_card_codes.canceled { None } else { Some(d.return_card_codes.list.clone()) }
}
fn protected_filter<'a>(f: Option<super::Filter<'a>>) -> Option<impl Fn(&mut Duel, CardId) -> bool + 'a> {
    f.map(|f| move |d: &mut Duel, c: CardId| super::check_matching(d, f, c))
}
fn ex_parts<'a>(ex: super::Exclude<'a>) -> (Option<CardId>, Option<&'a CardSet>) {
    match ex {
        super::Exclude::None => (None, None),
        super::Exclude::Card(c) => (Some(c), None),
        super::Exclude::Group(g) => (None, Some(g)),
    }
}
/// `Duel.GetReleaseGroup(player, use_hand = false, use_oppo = false, reason = REASON_COST)`
pub fn get_release_group(d: &mut Duel, p: u8, hand: bool, oppo: bool, reason: Option<u32>) -> Group {
    if p > 1 {
        return CardSet::new();
    }
    let (mut a, mut b, mut c) = (CardSet::new(), CardSet::new(), CardSet::new());
    d.get_release_list(p, Some(&mut a), Some(&mut b), Some(&mut c), hand, None, None, None, oppo, reason.unwrap_or(REASON_COST));
    a.extend(b.iter());
    a.extend(c.iter());
    a
}
pub fn get_release_group_count(d: &mut Duel, p: u8, hand: bool, oppo: bool, reason: Option<u32>) -> i32 {
    if p > 1 {
        return 0;
    }
    d.get_release_list(p, None, None, None, hand, None, None, None, oppo, reason.unwrap_or(REASON_COST))
}
/// The optional arguments of `Duel.CheckReleaseGroup(Ex)` / `SelectReleaseGroup(Ex)` given after
/// the boolean (`use_hand, max/cancelable, check_field, to_check, toplayer, zone, use_oppo`).
#[derive(Clone, Copy, Default)]
pub struct ReleaseArgs {
    pub use_hand: Option<bool>,
    pub max: Option<u16>,
    pub cancelable: bool,
    pub check_field: bool,
    pub to_check: Option<CardId>,
    pub toplayer: Option<u8>,
    pub zone: Option<u32>,
    pub use_oppo: bool,
}
/// `Duel.CheckReleaseGroup(player, f, count, [use_hand, max, check_field, to_check, toplayer, zone, use_oppo,] ex)`
pub fn check_release_group(d: &mut Duel, p: u8, f: Option<super::Filter>, min: u16, ra: ReleaseArgs, ex: super::Exclude, default_use_hand: bool) -> bool {
    if p > 1 {
        return false;
    }
    let use_hand = ra.use_hand.unwrap_or(default_use_hand);
    let max = ra.max.unwrap_or(min);
    let toplayer = ra.toplayer.unwrap_or(p);
    let zone = ra.zone.unwrap_or(0xff) as u8;
    let (exc, exg) = ex_parts(ex);
    let pf = protected_filter(f);
    let filter: crate::field2::CardFilter = pf.as_ref().map(|x| x as &dyn Fn(&mut Duel, CardId) -> bool);
    let r = d.check_release_list(p, min as i32, max as i32, use_hand, filter, exc, exg, ra.check_field, toplayer, zone, ra.to_check, ra.use_oppo, REASON_EFFECT);
    d.core.must_select_cards.clear();
    r
}
/// `Duel.SelectReleaseGroup(player, f, min, max, [use_hand, cancelable, check_field, to_check, toplayer, zone, use_oppo,] ex)`: `None` if canceled.
#[allow(clippy::too_many_arguments)]
pub fn select_release_group(d: &mut Duel, p: u8, f: Option<super::Filter>, min: u16, max: u16, ra: ReleaseArgs, ex: super::Exclude, default_use_hand: bool) -> Option<Group> {
    check_action_permission(d);
    if p > 1 {
        return None;
    }
    let use_hand = ra.use_hand.unwrap_or(default_use_hand);
    let toplayer = ra.toplayer.unwrap_or(p);
    let zone = ra.zone.unwrap_or(0xff) as u8;
    let (exc, exg) = ex_parts(ex);
    let pf = protected_filter(f);
    let filter: crate::field2::CardFilter = pf.as_ref().map(|x| x as &dyn Fn(&mut Duel, CardId) -> bool);
    let (mut a, mut b, mut c) = (CardSet::new(), CardSet::new(), CardSet::new());
    d.get_release_list(p, Some(&mut a), Some(&mut b), Some(&mut c), use_hand, filter, exc, exg, ra.use_oppo, REASON_EFFECT);
    d.core.release_cards = a;
    d.core.release_cards_ex = b;
    d.core.release_cards_ex_oneof = c;
    d.emplace_process(units::SelectRelease {
        step: 0,
        min,
        max,
        playerid: p,
        cancelable: ra.cancelable,
        check_field: ra.check_field,
        toplayer,
        zone,
        to_check: ra.to_check,
        extra_release_nonsum_effect: None,
        must_choose_one: None,
    });
    d.run_pushed_units();
    super::return_cards(d, ra.cancelable)
}
/// `Duel.GetTributeGroup(c)`
pub fn get_tribute_group(d: &mut Duel, target: CardId) -> Group {
    let (mut a, mut b, mut c) = (CardSet::new(), CardSet::new(), CardSet::new());
    d.get_summon_release_list(target, Some(&mut a), Some(&mut b), Some(&mut c), None, false, 0xff00ff, 0x1);
    a.extend(b.iter());
    a.extend(c.iter());
    a
}
/// `Duel.GetTributeCount(c, mg, ex = false)`
pub fn get_tribute_count(d: &mut Duel, target: CardId, mg: Option<&Group>, ex: bool) -> i32 {
    d.get_summon_release_list(target, None, None, None, mg, ex, 0xff00ff, 0x1)
}
/// `Duel.CheckTribute(c, min, max = min, mg, toplayer = controller, zone = 0x1f)`
pub fn check_tribute(d: &mut Duel, target: CardId, min: u16, max: Option<u16>, mg: Option<&Group>, toplayer: Option<u8>, zone: Option<u32>) -> bool {
    let tp = toplayer.unwrap_or(d.card(target).current.controler);
    d.check_tribute(target, min as i32, max.unwrap_or(min) as i32, mg, tp, zone.unwrap_or(0x1f), 0xff00ff, 0x1)
}
/// `Duel.SelectTribute(player, c, min, max, mg, toplayer = player, zone = 0x1f, cancelable = false)`
#[allow(clippy::too_many_arguments)]
pub fn select_tribute(d: &mut Duel, p: u8, target: CardId, min: u16, max: u16, mg: Option<&Group>, toplayer: Option<u8>, zone: Option<u32>, cancelable: bool) -> Option<Group> {
    check_action_permission(d);
    if p > 1 {
        return None;
    }
    let tp = toplayer.unwrap_or(p);
    if tp > 1 {
        return None;
    }
    let ex = tp != p;
    let (mut a, mut b, mut c) = (CardSet::new(), CardSet::new(), CardSet::new());
    d.get_summon_release_list(target, Some(&mut a), Some(&mut b), Some(&mut c), mg, ex, 0xff00ff, 0x1);
    d.core.release_cards = a;
    d.core.release_cards_ex = b;
    d.core.release_cards_ex_oneof = c;
    d.select_tribute_cards(None, p, cancelable, min, max, tp, zone.unwrap_or(0x1f));
    d.run_pushed_units();
    super::return_cards(d, cancelable)
}
fn target_matching(d: &mut Duel, f: Option<super::Filter>, s: u8, loc1: u32, loc2: u32, ex: super::Exclude, out: Option<&mut CardSet>, fcount: i32) -> bool {
    let (exc, exg) = ex_parts(ex);
    let pf = protected_filter(f);
    let filter: crate::field2::CardFilter = pf.as_ref().map(|x| x as &dyn Fn(&mut Duel, CardId) -> bool);
    d.filter_matching_card(filter, s, loc1 & 0xffff, loc2 & 0xffff, out, exc, exg, None, fcount, true)
}
/// `Duel.GetTargetCount(f, player, s, o, ex)`
pub fn get_target_count(d: &mut Duel, f: Option<super::Filter>, p: u8, s: u32, o: u32, ex: super::Exclude) -> usize {
    let mut g = CardSet::new();
    target_matching(d, f, p, s, o, ex, Some(&mut g), 0);
    g.len()
}
/// `Duel.IsExistingTarget(f, player, s, o, count, ex)`
pub fn is_existing_target(d: &mut Duel, f: Option<super::Filter>, p: u8, s: u32, o: u32, count: u16, ex: super::Exclude) -> bool {
    target_matching(d, f, p, s, o, ex, None, count as i32)
}
/// `Duel.SelectTarget(player, f, s_player, s, o, min, max, cancelable, ex)`: the targets (`None`
/// if canceled or there is no chain link).
#[allow(clippy::too_many_arguments)]
pub fn select_target(d: &mut Duel, sel_player: u8, f: Option<super::Filter>, p: u8, s: u32, o: u32, min: u16, max: u16, cancelable: bool, ex: super::Exclude) -> Option<Group> {
    check_action_permission(d);
    if sel_player > 1 || d.core.current_chain.is_empty() {
        return None;
    }
    let mut g = CardSet::new();
    target_matching(d, f, p, s, o, ex, Some(&mut g), 0);
    d.core.select_cards = g.to_vec();
    d.emplace_process(units::SelectCard { step: 0, playerid: sel_player, cancelable, min: min as u8, max: max as u8 });
    d.run_pushed_units();
    if d.return_cards.canceled {
        return None;
    }
    let ret = CardSet::from_iter(d.return_cards.list.iter().copied());
    Some(add_targets(d, ret))
}
/// The cards just selected become targets of the current chain link (the continuation of
/// `Duel.SelectTarget`): the group the script gets.
pub(crate) fn add_targets(d: &mut Duel, ret: CardSet) -> Group {
    let Some(r) = d.get_chain(0) else { return ret };
    let ch = d.chain_at_mut(r);
    let tc = ch.target_cards.get_or_insert_with(CardSet::new);
    tc.extend(ret.iter());
    let all = tc.clone();
    let (te, cid) = (ch.triggering_effect.unwrap(), ch.chain_id);
    if d.effect(te).type_ as u32 & EFFECT_TYPE_CONTINUOUS != 0 {
        return all;
    }
    for c in crate::card::public_order(d, &ret) {
        // (announced one by one)
        c.create_relation_chain(d, te, cid);
        if d.effect(te).is_flag(EFFECT_FLAG_CARD_TARGET) {
            let loc = c.get_info_location(d);
            let m = d.new_message(MSG_BECOME_TARGET);
            d.msg(m).u32(1u32).loc(loc);
        }
    }
    ret
}
/// `Duel.SelectFusionMaterial(player, c, g, gc, chkf = PLAYER_NONE)`
pub fn select_fusion_material(d: &mut Duel, p: u8, c: CardId, g: &Group, gc: Option<CardSet>, chkf: Option<u64>) -> Group {
    check_action_permission(d);
    if p > 1 {
        return CardSet::new();
    }
    let fm = d.new_group(g.clone());
    let forced = gc.map(|g| d.new_group(g));
    d.emplace_process(units::SelectFusion { step: 0, playerid: p, chkf: chkf.unwrap_or(PLAYER_NONE as u64), fusion_materials: Some(fm), forced_materials: forced, pcard: Some(c) });
    d.run_pushed_units();
    d.core.fusion_materials.clone()
}
pub fn set_fusion_material(d: &mut Duel, g: &Group) {
    d.core.fusion_materials = g.clone();
}
/// `Duel.GetRitualMaterial(player, check_level = true)`
pub fn get_ritual_material(d: &mut Duel, p: u8, check_level: bool) -> Group {
    if p > 1 {
        return CardSet::new();
    }
    let mut g = CardSet::new();
    let re = d.core.reason_effect;
    d.get_ritual_material(p, re, &mut g, check_level);
    g
}
pub fn release_ritual_material(d: &mut Duel, g: &Group, release_deck: bool) {
    check_action_permission(d);
    d.ritual_release(g, release_deck);
    d.run_pushed_units();
}
pub fn get_fusion_material(d: &mut Duel, p: u8) -> Group {
    if p > 1 {
        return CardSet::new();
    }
    let mut g = CardSet::new();
    d.get_fusion_material(p, &mut g);
    g
}
pub fn is_summon_cancelable(d: &Duel) -> bool {
    d.core.summon_cancelable != 0
}
/// `Duel.SetSelectedCard(targets)`
pub fn set_selected_card(d: &mut Duel, targets: impl Into<CardSet>) {
    d.core.must_select_cards = targets.into().to_vec();
}
pub fn grab_selected_card(d: &mut Duel) -> Group {
    CardSet::from_iter(std::mem::take(&mut d.core.must_select_cards))
}
/// `Duel.SetTargetCard(targets)`
pub fn set_target_card(d: &mut Duel, targets: impl Into<super::CardOrGroup>) {
    check_action_permission(d);
    let targets: super::CardOrGroup = targets.into();
    let is_card = matches!(targets, super::CardOrGroup::Card(_));
    let g = targets.cards();
    let Some(r) = d.get_chain(0) else { return };
    let ch = d.chain_at_mut(r);
    let (te, cid) = (ch.triggering_effect.unwrap(), ch.chain_id);
    let continuous = d.effect(te).type_ as u32 & EFFECT_TYPE_CONTINUOUS != 0;
    let ch = d.chain_at_mut(r);
    let tc = ch.target_cards.get_or_insert_with(CardSet::new);
    if continuous {
        // (a card is added; a group replaces the targets)
        if is_card {
            tc.extend(g.iter());
        } else {
            *tc = g;
        }
        return;
    }
    tc.extend(g.iter());
    let order = crate::card::public_order(d, &g);
    for &c in &order {
        c.create_relation_chain(d, te, cid);
    }
    if d.effect(te).is_flag(EFFECT_FLAG_CARD_TARGET) {
        let m = d.new_message(MSG_BECOME_TARGET);
        d.msg(m).u32(order.len() as u32);
        for c in order {
            let loc = c.get_info_location(d);
            d.msg(m).loc(loc);
        }
    }
}
pub fn clear_target_card(d: &mut Duel) {
    if let Some(r) = d.get_chain(0) {
        if let Some(tc) = d.chain_at_mut(r).target_cards.as_mut() {
            tc.clear();
        }
    }
}
pub fn set_target_player(d: &mut Duel, p: u8) {
    check_action_permission(d);
    if let Some(r) = d.get_chain(0) {
        d.chain_at_mut(r).target_player = p;
    }
}
pub fn set_target_param(d: &mut Duel, param: u32) {
    check_action_permission(d);
    if let Some(r) = d.get_chain(0) {
        d.chain_at_mut(r).target_param = param as i32;
    }
}
fn set_opinfo(list: &mut Vec<(u64, crate::duel::OpTarget)>, cate: u64, opt: crate::duel::OpTarget) {
    match list.binary_search_by_key(&cate, |x| x.0) {
        Ok(i) => list[i].1 = opt,
        Err(i) => list.insert(i, (cate, opt)),
    }
}
/// `Duel.SetOperationInfo(ch, category, targets, count, player, param)`
#[allow(clippy::too_many_arguments)]
pub fn set_operation_info(d: &mut Duel, ct: u32, cate: u64, targets: Option<CardSet>, count: u8, p: u8, param: i32) {
    check_action_permission(d);
    let Some(r) = d.get_chain(ct as u8) else { return };
    if let Some(t) = &targets {
        if cate == 0x200 && p as u32 == PLAYER_ALL && t.len() != 2 {
            crate::lua::lua_error("Called Duel.SetOperationInfo with CATEGORY_SPECIAL_SUMMON and PLAYER_ALL but the group size wasn't exactly 2.");
        }
    }
    let opt = crate::duel::OpTarget { op_cards: targets, op_count: count, op_player: p, op_param: param };
    set_opinfo(&mut d.chain_at_mut(r).opinfos, cate, opt);
}
/// `Duel.GetOperationInfo(ch, category)`: `None` (Lua false) or the info.
pub fn get_operation_info(d: &Duel, ct: u32, cate: u64) -> Option<crate::duel::OpTarget> {
    let r = d.get_chain(ct as u8)?;
    d.chain_at(r).opinfos.iter().find(|x| x.0 == cate).map(|x| x.1.clone())
}
#[allow(clippy::too_many_arguments)]
pub fn set_possible_operation_info(d: &mut Duel, ct: u32, cate: u64, targets: Option<CardSet>, count: u8, p: u8, param: i32) {
    check_action_permission(d);
    let Some(r) = d.get_chain(ct as u8) else { return };
    let opt = crate::duel::OpTarget { op_cards: targets, op_count: count, op_player: p, op_param: param };
    set_opinfo(&mut d.chain_at_mut(r).possibleopinfos, cate, opt);
}
pub fn get_possible_operation_info(d: &Duel, ct: u32, cate: u64) -> Option<crate::duel::OpTarget> {
    let r = d.get_chain(ct as u8)?;
    d.chain_at(r).possibleopinfos.iter().find(|x| x.0 == cate).map(|x| x.1.clone())
}
pub fn get_operation_count(d: &Duel, ct: u32) -> usize {
    d.get_chain(ct as u8).map_or(0, |r| d.chain_at(r).opinfos.len())
}
pub fn clear_operation_info(d: &mut Duel, ct: u32) {
    check_action_permission(d);
    if let Some(r) = d.get_chain(ct as u8) {
        let ch = d.chain_at_mut(r);
        ch.opinfos.clear();
        ch.possibleopinfos.clear();
    }
}
/// `Duel.Overlay(target, materials, send_materials_to_grave = false)`
pub fn overlay(d: &mut Duel, target: CardId, materials: impl Into<CardSet>, send_materials_to_grave: bool) {
    check_action_permission(d);
    if d.card(target).overlay_target.is_some() {
        crate::lua::lua_error("Attempt to overlay materials to a card that is an overlay material.");
    }
    let mats: CardSet = materials.into();
    if mats.contains(target) {
        crate::lua::lua_error("Attempt to overlay a card with itself.");
    }
    d.xyz_overlay(target, mats, send_materials_to_grave);
    d.run_pushed_units();
}
pub fn get_overlay_group(d: &mut Duel, rplayer: u8, s: u8, o: u8, targets: Option<&Group>) -> Group {
    if rplayer > 1 {
        return CardSet::new();
    }
    let mut g = CardSet::new();
    d.get_overlay_group(rplayer, s, o, &mut g, targets);
    g
}
pub fn get_overlay_count(d: &mut Duel, rplayer: u8, s: u8, o: u8, targets: Option<&Group>) -> i32 {
    if rplayer > 1 { 0 } else { d.get_overlay_count(rplayer, s, o, targets) }
}
#[allow(clippy::too_many_arguments)]
pub fn check_remove_overlay_card(d: &mut Duel, p: u8, s: u8, o: u8, count: u16, reason: u32, targets: Option<&Group>) -> bool {
    p <= 1 && d.is_player_can_remove_overlay_card(p, targets, s, o, count, reason)
}
/// `Duel.RemoveOverlayCard(player, s, o, min, max, reason, targets)`
#[allow(clippy::too_many_arguments)]
pub fn remove_overlay_card(d: &mut Duel, p: u8, s: u8, o: u8, min: u16, max: u16, reason: u32, targets: Option<&Group>) -> i32 {
    check_action_permission(d);
    if p > 1 {
        return 0;
    }
    d.remove_overlay_card(reason, targets, p as u32, s, o, min, max);
    result_i32(d)
}
/// `Duel.Hint(type, player, desc)`
pub fn hint(d: &mut Duel, htype: u8, p: u8, desc: u64) {
    if p > 1 {
        return;
    }
    let p = if htype as u32 == HINT_OPSELECTED { 1 - p } else { p };
    let m = d.new_message(MSG_HINT);
    d.msg(m).u8(htype).u8(p).u64(desc);
}
/// `Duel.HintSelection(targets, selection = true)`
pub fn hint_selection(d: &mut Duel, targets: impl Into<CardSet>, selection: bool) {
    let g: CardSet = targets.into();
    let m = d.new_message(if selection { MSG_CARD_SELECTED } else { MSG_BECOME_TARGET });
    d.msg(m).u32(g.len() as u32);
    for c in crate::card::public_order(d, &g) {
        let loc = c.get_info_location(d);
        d.msg(m).loc(loc);
    }
}
/// `Duel.SelectEffectYesNo(player, c, desc = 95)`
pub fn select_effect_yes_no(d: &mut Duel, p: u8, c: CardId, desc: Option<u64>) -> bool {
    check_action_permission(d);
    if p > 1 {
        return false;
    }
    d.emplace_process(units::SelectEffectYesNo { step: 0, playerid: p, pcard: Some(c), description: desc.unwrap_or(95) });
    result_i32(d) != 0
}
pub fn select_yes_no(d: &mut Duel, p: u8, desc: u64) -> bool {
    check_action_permission(d);
    if p > 1 {
        return false;
    }
    d.emplace_process(units::SelectYesNo { step: 0, playerid: p, description: desc });
    result_i32(d) != 0
}
/// `Duel.SelectOption(player, [sel_hint = true,] options...)`: the index chosen.
pub fn select_option(d: &mut Duel, p: u8, sel_hint: bool, options: &[u64]) -> i32 {
    check_action_permission(d);
    if p > 1 {
        return 0;
    }
    d.core.select_options = options.to_vec();
    d.emplace_process(units::SelectOption { step: 0, playerid: p });
    d.run_pushed_units();
    let r = d.returns.at_i32(0);
    if sel_hint && !d.core.select_options.is_empty() {
        let v = d.core.select_options[r as usize];
        let m = d.new_message(MSG_HINT);
        d.msg(m).u8(HINT_OPSELECTED as u8).u8(p).u64(v);
    }
    r
}
pub fn select_position(d: &mut Duel, p: u8, c: CardId, positions: u8) -> u32 {
    check_action_permission(d);
    let code = d.card(c).data.code;
    d.emplace_process(units::SelectPosition { step: 0, playerid: p, positions, code });
    result_i32(d) as u32
}
/// The zones a `SelectDisField` response names, as a flag.
fn selected_zones_flag(d: &Duel, p: u8, count: u8) -> u32 {
    let mut dfflag = 0u32;
    let mut pa = 0usize;
    for _ in 0..count {
        let pp = d.returns.at_i8(pa) as u8;
        let l = d.returns.at_i8(pa + 1) as u8;
        let s = d.returns.at_i8(pa + 2) as u8 as u32;
        dfflag |= 1u32.wrapping_shl(s + if pp == p { 0 } else { 16 } + if l as u32 == LOCATION_MZONE { 0 } else { 8 });
        pa += 3;
    }
    dfflag
}
fn select_dis_field(d: &mut Duel, p: u8, flag: u32, count: u8) -> u32 {
    d.emplace_process(units::SelectPlace { step: 0, playerid: p, count, flag, disable_field: true });
    d.run_pushed_units();
    selected_zones_flag(d, p, count)
}
/// `Duel.SelectDisableField(player, count, s, o, filter, all_field = false)`
pub fn select_disable_field(d: &mut Duel, p: u8, count: u8, location1: u32, location2: u32, filter: Option<u32>, all_field: bool) -> u32 {
    check_action_permission(d);
    if p > 1 {
        return 0;
    }
    let mut dfilter: u32 = 0xe0e0e0e0;
    if all_field {
        dfilter = if d.is_flag(DUEL_EMZONE) { 0x800080 } else { 0xE000E0 };
        dfilter |= d.get_pzone_zones_flag();
    }
    let filt = dfilter | filter.unwrap_or(dfilter);
    let (mut ct1, mut ct2, mut ct3, mut ct4) = (0u32, 0u32, 0u32, 0u32);
    let mut flag: u32 = 0xffffffff;
    let zone_count = |d: &mut Duel, pl: u8, loc: u32, extra: &[(u32, u32)], ct: &mut u32| -> u32 {
        let mut plist = 0u32;
        *ct = d.get_useable_count(None, pl, loc as u8, PLAYER_NONE as u8, units::LocationReason::None, 0xff, Some(&mut plist)) as u32;
        if all_field {
            let mask: u32 = extra.iter().map(|x| x.1).sum();
            plist &= !mask;
            for &(seq, bit) in extra {
                if !d.is_location_useable(pl as u32, loc, seq) {
                    plist |= bit;
                } else {
                    *ct += 1;
                }
            }
        }
        plist
    };
    let mz: &[(u32, u32)] = &[(5, 0x20), (6, 0x40)];
    let sz: &[(u32, u32)] = &[(5, 0x20), (6, 0x40), (7, 0x80)];
    if location1 & LOCATION_MZONE != 0 {
        let pl = zone_count(d, p, LOCATION_MZONE, mz, &mut ct1);
        flag = (flag & 0xffffff00) | pl;
    }
    if location1 & LOCATION_SZONE != 0 {
        let pl = zone_count(d, p, LOCATION_SZONE, sz, &mut ct2);
        flag = (flag & 0xffff00ff) | (pl << 8);
    }
    if location2 & LOCATION_MZONE != 0 {
        let pl = zone_count(d, 1 - p, LOCATION_MZONE, mz, &mut ct3);
        flag = (flag & 0xff00ffff) | (pl << 16);
    }
    if location2 & LOCATION_SZONE != 0 {
        let pl = zone_count(d, 1 - p, LOCATION_SZONE, sz, &mut ct4);
        flag = (flag & 0xffffff) | (pl << 24);
    }
    flag |= filt | if all_field { 0x800080 } else { 0xe0e0e0e0 };
    let total = ct1 + ct2 + ct3 + ct4;
    let count = if count as u32 > total { total as u8 } else { count };
    if count == 0 {
        return 0;
    }
    select_dis_field(d, p, flag, count)
}
/// `Duel.SelectFieldZone(player, count, s, o, filter = 0xe0e0e0e0)`
pub fn select_field_zone(d: &mut Duel, p: u8, count: u8, location1: u32, location2: u32, filter: Option<u32>) -> u32 {
    check_action_permission(d);
    if p > 1 {
        return 0;
    }
    let mut filter = filter.unwrap_or(0xe0e0e0e0);
    filter |= if d.is_flag(DUEL_EMZONE) { 0x800080 } else { 0xE000E0 };
    filter |= d.get_pzone_zones_flag();
    let mut flag: u32 = 0xffffffff;
    if location1 & LOCATION_MZONE != 0 {
        flag &= 0xffffff00;
    }
    if location1 & LOCATION_SZONE != 0 {
        flag &= 0xffff00ff;
    }
    if location2 & LOCATION_MZONE != 0 {
        flag &= 0xff00ffff;
    }
    if location2 & LOCATION_SZONE != 0 {
        flag &= 0xffffff;
    }
    flag |= filter;
    if flag == 0xffffffff {
        return 0;
    }
    select_dis_field(d, p, flag, count)
}
pub fn announce_race(d: &mut Duel, p: u8, count: u8, available: u64) -> u64 {
    check_action_permission(d);
    if available & !RACE_ALL != 0 {
        crate::lua::lua_error("Passed an invalid race.");
    }
    let count = count.min(available.count_ones() as u8);
    d.emplace_process(units::AnnounceRace { step: 0, playerid: p, count, available });
    d.run_pushed_units();
    d.returns.at_u64(0)
}
pub fn announce_attribute(d: &mut Duel, p: u8, count: u8, available: u32) -> u32 {
    check_action_permission(d);
    if available & !ATTRIBUTE_ALL != 0 {
        crate::lua::lua_error("Passed an invalid attribute.");
    }
    let count = count.min(available.count_ones() as u8);
    d.emplace_process(units::AnnounceAttribute { step: 0, playerid: p, count, available });
    result_i32(d) as u32
}
fn announce_number_options(d: &mut Duel, p: u8) -> Option<(u64, i32)> {
    if d.core.select_options.is_empty() {
        return None;
    }
    d.emplace_process(units::AnnounceNumber { step: 0, playerid: p });
    d.run_pushed_units();
    let r = d.returns.at_i32(0);
    Some((d.core.select_options[r as usize], r))
}
/// `Duel.AnnounceNumberRange(player, min = 1, max = 12, excluded...)`: (number, index)
pub fn announce_number_range(d: &mut Duel, p: u8, min: Option<u32>, max: Option<u32>, excluded: &[u32]) -> Option<(u64, i32)> {
    check_action_permission(d);
    let (mut min, mut max) = (min.unwrap_or(1), max.unwrap_or(12));
    if min > max {
        std::mem::swap(&mut min, &mut max);
    }
    let ex: std::collections::BTreeSet<u32> = excluded.iter().copied().filter(|&v| v >= min && v <= max).collect();
    d.core.select_options = (min..=max).filter(|v| !ex.contains(v)).map(|v| v as u64).collect();
    announce_number_options(d, p)
}
/// `Duel.AnnounceNumber(player, numbers...)`: (number, index)
pub fn announce_number(d: &mut Duel, p: u8, numbers: &[u64]) -> Option<(u64, i32)> {
    check_action_permission(d);
    d.core.select_options = numbers.to_vec();
    announce_number_options(d, p)
}
/// `Duel.AnnounceCard(player, type_or_opcodes...)`: `filter` is the announce filter (opcodes), or a
/// single card type when it has one entry.
pub fn announce_card(d: &mut Duel, p: u8, filter: &[u64]) -> i32 {
    check_action_permission(d);
    let mut options: Vec<u64> = if filter.len() <= 1 {
        let ttype = filter.first().copied().unwrap_or((TYPE_MONSTER | TYPE_SPELL | TYPE_TRAP) as u64);
        vec![ttype, OPCODE_ISTYPE]
    } else {
        filter.to_vec()
    };
    let mut stack_size: i32 = 0;
    let mut has_opcodes = false;
    for &opcode in &options {
        if opcode != OPCODE_ALLOW_ALIASES && opcode != OPCODE_ALLOW_TOKENS {
            has_opcodes = true;
        }
        match opcode {
            OPCODE_ADD | OPCODE_SUB | OPCODE_MUL | OPCODE_DIV | OPCODE_AND | OPCODE_OR | OPCODE_BAND | OPCODE_BOR | OPCODE_BXOR | OPCODE_LSHIFT | OPCODE_RSHIFT => stack_size -= 1,
            OPCODE_NEG | OPCODE_NOT | OPCODE_BNOT | OPCODE_ISCODE | OPCODE_ISSETCARD | OPCODE_ISTYPE | OPCODE_ISRACE | OPCODE_ISATTRIBUTE | OPCODE_ALLOW_ALIASES | OPCODE_ALLOW_TOKENS => {}
            _ => stack_size += 1,
        }
        if stack_size <= 0 {
            break;
        }
    }
    if stack_size != 1 && has_opcodes {
        crate::lua::lua_error("Parameters are invalid.");
    }
    if !has_opcodes {
        options.push((TYPE_MONSTER | TYPE_SPELL | TYPE_TRAP) as u64);
        options.push(OPCODE_ISTYPE);
    }
    d.core.select_options = options;
    d.emplace_process(units::AnnounceCard { step: 0, playerid: p });
    result_i32(d)
}
fn select_option_hinted(d: &mut Duel, p: u8, options: &[u64], hint: bool) -> i32 {
    d.core.select_options = options.to_vec();
    d.emplace_process(units::SelectOption { step: 0, playerid: p });
    d.run_pushed_units();
    let r = d.returns.at_i32(0);
    if hint {
        let v = d.core.select_options[r as usize];
        let m = d.new_message(MSG_HINT);
        d.msg(m).u8(HINT_OPSELECTED as u8).u8(p).u64(v);
    }
    r
}
pub fn announce_type(d: &mut Duel, p: u8) -> i32 {
    check_action_permission(d);
    select_option_hinted(d, p, &[70, 71, 72], true)
}
/// `Duel.AnnounceCoin(player, desc = 552)`: 1 heads, 0 tails.
pub fn announce_coin(d: &mut Duel, p: u8, desc: Option<u64>) -> i32 {
    check_action_permission(d);
    let m = d.new_message(MSG_HINT);
    d.msg(m).u8(HINT_SELECTMSG as u8).u8(p).u64(desc.unwrap_or(552));
    // (the continuation reads argument 2, the description, as the "hint" flag: any value is true)
    1 - select_option_hinted(d, p, &[60, 61], true)
}
/// `Duel.TossCoin(player, count)`: the results.
pub fn toss_coin(d: &mut Duel, p: u8, count: u8) -> Vec<i32> {
    check_action_permission(d);
    if p > 1 || count == 0 {
        return Vec::new();
    }
    let count = count.min(5);
    let (re, rp) = (d.core.reason_effect, d.core.reason_player);
    d.emplace_process(units::TossCoin { step: 0, playerid: p, reason_player: rp, count, reason_effect: re });
    d.run_pushed_units();
    d.core.coin_results.iter().map(|&r| r as i32).collect()
}
/// `Duel.TossDice(player, count1, count2 = 0)`
pub fn toss_dice(d: &mut Duel, p: u8, count1: u8, count2: u8) -> Vec<i32> {
    check_action_permission(d);
    if p > 1 {
        return Vec::new();
    }
    let (re, rp) = (d.core.reason_effect, d.core.reason_player);
    d.emplace_process(units::TossDice { step: 0, playerid: p, reason_player: rp, count1, count2, reason_effect: re });
    d.run_pushed_units();
    d.core.dice_results.iter().map(|&r| r as i32).collect()
}
/// `Duel.RockPaperScissors(repeat = true)`
pub fn rock_paper_scissors(d: &mut Duel, repeat: bool) -> i32 {
    d.emplace_process(units::RockPaperScissors { step: 0, repeat, hand0: 0 });
    result_i32(d)
}
pub fn get_coin_result(d: &Duel) -> Vec<i32> {
    d.core.coin_results.iter().map(|&r| r as i32).collect()
}
pub fn get_dice_result(d: &Duel) -> Vec<i32> {
    d.core.dice_results.iter().map(|&r| r as i32).collect()
}
pub fn set_coin_result(d: &mut Duel, results: &[u8]) {
    if results.len() != d.core.coin_results.len() {
        crate::lua::lua_error(format!("The number of coin results passed didn't match the expected amount of {}. (Passed {})", d.core.coin_results.len(), results.len()));
    }
    for (i, &r) in results.iter().enumerate() {
        d.core.coin_results[i] = r != 0;
    }
}
pub fn set_dice_result(d: &mut Duel, results: &[u8]) {
    if results.len() != d.core.dice_results.len() {
        crate::lua::lua_error(format!("The number of dice results passed didn't match the expected amount of {}. (Passed {})", d.core.dice_results.len(), results.len()));
    }
    d.core.dice_results.copy_from_slice(results);
}
pub fn is_duel_type(d: &Duel, duel_type: u64) -> bool {
    d.is_flag(duel_type)
}
pub fn get_duel_type(d: &Duel) -> u64 {
    d.core.duel_options
}
pub fn is_player_affected_by_effect(d: &mut Duel, p: u8, code: u32) -> Option<EffectId> {
    if p > 1 { None } else { d.is_player_affected_by_effect(p, code) }
}
/// `Duel.GetPlayerEffect(player, code = 0)` (empty: Lua nil)
pub fn get_player_effect(d: &mut Duel, p: u8, code: u32) -> Vec<EffectId> {
    if p > 1 {
        return Vec::new();
    }
    let mut eset = Vec::new();
    d.get_player_effect(p, code, &mut eset);
    eset
}
/// `Duel.IsPlayerCanDraw(player, count = 0)`
pub fn is_player_can_draw(d: &mut Duel, p: u8, count: u32) -> bool {
    p <= 1 && d.is_player_can_draw(p) && d.player[p as usize].list_main.len() >= count as usize
}
pub fn is_player_can_discard_deck(d: &mut Duel, p: u8, count: u32) -> bool {
    p <= 1 && d.is_player_can_discard_deck(p, count)
}
pub fn is_player_can_discard_deck_as_cost(d: &mut Duel, p: u8, count: u32) -> bool {
    p <= 1 && d.is_player_can_discard_deck_as_cost(p, count)
}
/// `Duel.IsPlayerCanSummon(player)` / `(player, sumtype, c)`
pub fn is_player_can_summon(d: &mut Duel, p: u8, args: Option<(u32, CardId)>) -> bool {
    if p > 1 {
        return false;
    }
    match args {
        None => d.is_player_can_action(p, EFFECT_CANNOT_SUMMON),
        Some((sumtype, c)) => d.is_player_can_summon(sumtype, p, c, p),
    }
}
/// `Duel.CanPlayerSetMonster(player)` / `(player, sumtype, c)`
pub fn can_player_set_monster(d: &mut Duel, p: u8, args: Option<(u32, CardId)>) -> bool {
    if p > 1 {
        return false;
    }
    match args {
        None => d.is_player_can_action(p, EFFECT_CANNOT_MSET),
        Some((sumtype, c)) => d.is_player_can_mset(sumtype, p, c, p),
    }
}
pub fn can_player_set_spell_trap(d: &mut Duel, p: u8, c: Option<CardId>) -> bool {
    if p > 1 {
        return false;
    }
    match c {
        None => d.is_player_can_action(p, EFFECT_CANNOT_SSET),
        Some(c) => d.is_player_can_sset(p, c),
    }
}
/// `Duel.IsPlayerCanSpecialSummon(player)` / `(player, sumtype, sumpos, toplayer, c)`
pub fn is_player_can_special_summon(d: &mut Duel, p: u8, args: Option<(u32, u8, u8, CardId)>) -> bool {
    if p > 1 {
        return false;
    }
    match args {
        None => d.is_player_can_spsummon(p),
        Some((sumtype, sumpos, toplayer, c)) => {
            let re = d.core.reason_effect;
            d.is_player_can_spsummon_ex(re, sumtype, sumpos, p, toplayer, c, None)
        }
    }
}
pub fn is_player_can_flip_summon(d: &mut Duel, p: u8, c: Option<CardId>) -> bool {
    if p > 1 {
        return false;
    }
    match c {
        None => d.is_player_can_action(p, EFFECT_CANNOT_FLIP_SUMMON),
        Some(c) => d.is_player_can_flipsummon(p, c),
    }
}
/// The optional overrides of `Duel.IsPlayerCanSpecialSummonMonster` (nil: from the database).
#[derive(Clone, Default)]
pub struct MonsterData {
    pub setcodes: Option<Vec<u16>>,
    pub type_: Option<u32>,
    pub attack: Option<i32>,
    pub defense: Option<i32>,
    pub level: Option<u32>,
    pub race: Option<u64>,
    pub attribute: Option<u32>,
}
/// `Duel.IsPlayerCanSpecialSummonMonster(player, code, setcodes, type, atk, def, level, race, attribute, pos = POS_FACEUP, toplayer = player, sumtype = 0)`
pub fn is_player_can_special_summon_monster(d: &mut Duel, p: u8, code: u32, md: MonsterData, pos: Option<u8>, toplayer: Option<u8>, sumtype: u32) -> bool {
    if p > 1 {
        return false;
    }
    let mut dat = d.read_card(code);
    dat.code = code;
    dat.alias = 0;
    if let Some(sc) = md.setcodes {
        for s in sc {
            dat.setcodes.insert(s);
        }
    }
    if let Some(v) = md.type_ {
        dat.type_ = v;
    }
    if let Some(v) = md.attack {
        dat.attack = v;
    }
    if let Some(v) = md.defense {
        dat.defense = v;
    }
    if let Some(v) = md.level {
        dat.level = v;
    }
    if let Some(v) = md.race {
        dat.race = v;
    }
    if let Some(v) = md.attribute {
        dat.attribute = v;
    }
    let pos = pos.unwrap_or(POS_FACEUP as u8);
    let tp = toplayer.unwrap_or(p);
    d.is_player_can_spsummon_monster(p, tp, pos, sumtype, &dat)
}
pub fn is_player_can_special_summon_count(d: &mut Duel, p: u8, count: u32) -> bool {
    p <= 1 && d.is_player_can_spsummon_count(p, count)
}
/// `Duel.IsPlayerCanRelease(player)` / `(player, c, reason = REASON_COST)`
pub fn is_player_can_release(d: &mut Duel, p: u8, args: Option<(CardId, Option<u32>)>) -> bool {
    if p > 1 {
        return false;
    }
    match args {
        None => d.is_player_can_action(p, EFFECT_CANNOT_RELEASE),
        Some((c, r)) => d.is_player_can_release(p, c, r.unwrap_or(REASON_COST)),
    }
}
/// `Duel.IsPlayerCanRemove(player)` / `(player, c, reason = REASON_EFFECT)`
pub fn is_player_can_remove(d: &mut Duel, p: u8, args: Option<(CardId, Option<u32>)>) -> bool {
    if p > 1 {
        return false;
    }
    match args {
        None => d.is_player_can_action(p, EFFECT_CANNOT_REMOVE),
        Some((c, r)) => d.is_player_can_remove(p, c, r.unwrap_or(REASON_EFFECT)),
    }
}
pub fn is_player_can_send_to_hand(d: &mut Duel, p: u8, c: Option<CardId>) -> bool {
    if p > 1 {
        return false;
    }
    match c {
        None => d.is_player_can_action(p, EFFECT_CANNOT_TO_HAND),
        Some(c) => d.is_player_can_send_to_hand(p, c),
    }
}
/// `Duel.IsPlayerCanSendtoGrave(player)` / `(player, c, reason = REASON_EFFECT)`
pub fn is_player_can_send_to_grave(d: &mut Duel, p: u8, args: Option<(CardId, Option<u32>)>) -> bool {
    if p > 1 {
        return false;
    }
    match args {
        None => d.is_player_can_action(p, EFFECT_CANNOT_TO_GRAVE),
        Some((c, r)) => d.is_player_can_send_to_grave(p, c, r.unwrap_or(REASON_EFFECT)),
    }
}
pub fn is_player_can_send_to_deck(d: &mut Duel, p: u8, c: Option<CardId>) -> bool {
    if p > 1 {
        return false;
    }
    match c {
        None => d.is_player_can_action(p, EFFECT_CANNOT_TO_DECK),
        Some(c) => d.is_player_can_send_to_deck(p, c),
    }
}
pub fn is_player_can_additional_summon(d: &Duel, p: u8) -> bool {
    p <= 1 && d.core.extra_summon[p as usize] == 0
}
fn is_player_can_procedure_summon_group_impl(d: &mut Duel, p: u8, summon_type: u32) -> bool {
    let mut eset = Vec::new();
    d.filter_field_effect(EFFECT_SPSUMMON_PROC_G, &mut eset, true);
    for peff in eset {
        if peff.get_value(d, 0) as u32 != summon_type {
            continue;
        }
        let pcard = peff.get_handler(d);
        let ctrl = d.card(pcard).current.controler;
        if ctrl != p && !d.effect(peff).is_flag(EFFECT_FLAG_BOTH_SIDE) {
            continue;
        }
        let (oreason, op) = (d.core.reason_effect, d.core.reason_player);
        d.core.reason_effect = Some(peff);
        d.core.reason_player = ctrl;
        d.add_param(Value::Effect(peff));
        d.add_param(Value::Card(pcard));
        d.add_param(Value::Bool(true));
        d.add_param(Value::effect(oreason));
        d.add_param(Value::Int(p as i64));
        let cond = d.effect(peff).condition.clone();
        let ok = d.check_condition(cond.as_ref(), 5);
        d.core.reason_effect = oreason;
        d.core.reason_player = op;
        if ok {
            return true;
        }
    }
    false
}
pub fn is_player_can_pendulum_summon(d: &mut Duel, p: u8) -> bool {
    is_player_can_procedure_summon_group_impl(d, p, SUMMON_TYPE_PENDULUM)
}
pub fn is_player_can_procedure_summon_group(d: &mut Duel, p: u8, sumtype: u32) -> bool {
    is_player_can_procedure_summon_group_impl(d, p, sumtype)
}
/// `Duel.IsChainNegatable(ct)`: always true, as in the C++ (libduel.cpp)
pub fn is_chain_negatable(_d: &Duel, _chaincount: u8) -> bool {
    true
}
pub fn is_chain_disablable(d: &mut Duel, chaincount: u8) -> bool {
    if d.core.chain_solving { d.is_chain_disablable(chaincount) } else { true }
}
pub fn is_chain_solving(d: &Duel) -> bool {
    d.core.chain_solving
}
pub fn check_chain_target(d: &mut Duel, chaincount: u8, c: CardId) -> bool {
    d.check_chain_target(chaincount, c)
}
/// `Duel.CheckChainUniqueness()`: no two chain links of cards with the same name.
pub fn check_chain_uniqueness(d: &mut Duel) -> bool {
    if d.core.current_chain.is_empty() {
        return true;
    }
    let mut codes = std::collections::BTreeSet::new();
    for i in 0..d.core.current_chain.len() {
        let h = d.core.current_chain[i].triggering_effect.unwrap().get_handler(d);
        codes.insert(h.get_code(d));
    }
    codes.len() == d.core.current_chain.len()
}
/// `Duel.GetActivityCount(player, activity...)`
pub fn get_activity_count(d: &Duel, p: u8, activities: &[u32]) -> Vec<i64> {
    if p > 1 {
        return Vec::new();
    }
    let c = &d.core;
    let pi = p as usize;
    activities
        .iter()
        .map(|&a| match a & 0xff {
            ACTIVITY_SUMMON => c.summon_state_count[pi] as i64,
            ACTIVITY_NORMALSUMMON => c.normalsummon_state_count[pi] as i64,
            ACTIVITY_SPSUMMON => c.spsummon_state_count[pi] as i64,
            ACTIVITY_FLIPSUMMON => c.flipsummon_state_count[pi] as i64,
            ACTIVITY_ATTACK => c.attack_state_count[pi] as i64,
            ACTIVITY_BATTLE_PHASE => c.battle_phase_count[pi] as i64,
            _ => crate::lua::lua_error("Passed invalid ACTIVITY flag."),
        })
        .collect()
}
pub fn check_phase_activity(d: &Duel) -> bool {
    d.core.phase_action
}
/// `Duel.AddCustomActivityCounter(id, activity, filter)`
pub fn add_custom_activity_counter(d: &mut Duel, counter_id: u32, activity: u32, f: LuaFn) {
    let activity = activity & 0xff;
    if activity == ACTIVITY_BATTLE_PHASE || activity > ACTIVITY_CHAIN {
        crate::lua::lua_error("Passed invalid ACTIVITY counter.");
    }
    let m = d.get_counter_map(activity);
    if m.iter().any(|x| x.0 == counter_id) {
        return;
    }
    m.push((counter_id, crate::duel::ActionValue { check_function: Some(f), player_amount: [0, 0] }));
}
pub fn get_custom_activity_count(d: &mut Duel, counter_id: u32, p: u8, activity: u32) -> i32 {
    let activity = activity & 0xff;
    if activity == ACTIVITY_BATTLE_PHASE || activity > ACTIVITY_CHAIN || activity == 0 {
        crate::lua::lua_error("Passed invalid ACTIVITY counter.");
    }
    d.get_counter_map(activity).iter().find(|x| x.0 == counter_id).map_or(0, |x| x.1.player_amount[p as usize] as i32)
}
pub fn get_battled_count(d: &Duel, p: u8) -> u32 {
    if p > 1 { 0 } else { d.core.battled_count[p as usize] as u32 }
}
pub fn is_able_to_enter_bp(d: &mut Duel) -> bool {
    d.is_able_to_enter_bp()
}
/// `Duel.GetRandomNumber(max)` (0..=max) / `Duel.GetRandomNumber(min, max)`
pub fn get_random_number(d: &mut Duel, min: Option<u32>, max: u32) -> i32 {
    d.get_next_integer(min.unwrap_or(0) as i32, max as i32)
}
pub fn assume_reset(d: &mut Duel) {
    d.restore_assumes();
}
pub fn get_card_from_card_id(d: &Duel, id: u32) -> Option<CardId> {
    if id >= 1 && (id as usize) <= d.cards.len() && d.card(CardId(id - 1)).cardid == id {
        return Some(CardId(id - 1));
    }
    (0..d.cards.len() as u32).map(CardId).find(|&c| d.card(c).cardid == id)
}
pub fn tag_swap(d: &mut Duel, p: u8) {
    check_action_permission(d);
    if p > 1 {
        return;
    }
    d.tag_swap(p);
    d.run_pushed_units();
}
pub fn get_players_count(d: &Duel, p: u8) -> i32 {
    if p > 1 { 0 } else { d.get_player_count(p) }
}
pub fn swap_deck_and_grave(d: &mut Duel, p: u8) {
    check_action_permission(d);
    if p <= 1 {
        d.swap_deck_and_grave(p);
    }
}
/// `Duel.MajesticCopy(c, copied, reset, count)`
pub fn majestic_copy(d: &mut Duel, c: CardId, ccard: CardId, reset: Option<(u32, u16)>) {
    let (mut resv, resc) = reset.unwrap_or((RESET_EVENT + 0x1fe0000 + RESET_PHASE + PHASE_END + RESET_SELF_TURN + RESET_OPPO_TURN, 1));
    if resv & RESET_PHASE != 0 && resv & (RESET_SELF_TURN | RESET_OPPO_TURN) == 0 {
        resv |= RESET_SELF_TURN | RESET_OPPO_TURN;
    }
    let mut effs = d.card(ccard).single_effect.all();
    effs.extend(d.card(ccard).field_effect.all());
    for pe in effs {
        let e = d.effect(pe);
        if e.type_ as u32 & 0x7c == 0 && !e.is_flag2(EFFECT_FLAG2_MAJESTIC_MUST_COPY) {
            continue;
        }
        if !e.is_flag(EFFECT_FLAG_INITIAL) {
            continue;
        }
        let ce = pe.clone_effect(d, true);
        {
            let x = d.effect_mut(ce);
            x.owner = Some(c);
            x.flag[0] &= !EFFECT_FLAG_INITIAL;
            x.effect_owner = PLAYER_NONE as u8;
            x.reset_flag = resv;
            x.reset_count = resc;
        }
        ce.recharge(d);
        let x = d.effect_mut(ce);
        if x.type_ as u32 & EFFECT_TYPE_TRIGGER_F != 0 {
            x.type_ &= !(EFFECT_TYPE_TRIGGER_F as u16);
            x.type_ |= EFFECT_TYPE_TRIGGER_O as u16;
            x.flag[0] |= EFFECT_FLAG_DELAY;
        }
        if x.type_ as u32 & EFFECT_TYPE_QUICK_F != 0 {
            x.type_ &= !(EFFECT_TYPE_QUICK_F as u16);
            x.type_ |= EFFECT_TYPE_QUICK_O as u16;
        }
        c.add_effect(d, ce);
    }
}
pub fn get_starting_hand(d: &Duel, p: u8) -> i32 {
    if p > 1 { 0 } else { d.player[p as usize].start_count }
}
pub fn get_reason_player(d: &Duel) -> u8 {
    d.core.reason_player
}
pub fn get_reason_effect(d: &Duel) -> Option<EffectId> {
    d.core.reason_effect
}
/// `Duel.GetCard*FromCode(code)`: the printed data (`None` for an unknown code).
pub fn get_card_data_from_code(d: &Duel, code: u32) -> Option<CardData> {
    let data = d.read_card(code);
    if data.code == 0 { None } else { Some(data) }
}
