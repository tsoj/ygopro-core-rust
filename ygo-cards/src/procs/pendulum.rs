//! `Pendulum.*` (`proc_pendulum.lua`): Pendulum Summons. (The Lua tests an undefined `og`, which is
//! always nil.)

use crate::aux;
use crate::constants::*;
use crate::lua::{card, duel, effect, group, NO_EX};
use crate::prelude::*;
use std::sync::Arc;

/// `Pendulum.AddProcedure(c, reg, desc)`: the Pendulum Summon procedure, and (unless `reg` is
/// `false`) the activation from the hand
pub fn add_procedure(d: &mut Duel, c: CardId, reg: Option<bool>, desc: Option<u64>) {
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_description(d, e1, desc.unwrap_or(1163));
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC_G);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_range(d, e1, LOCATION_PZONE);
    effect::set_condition(d, e1, Some(proc_condition()));
    effect::set_operation(d, e1, Some(proc_operation()));
    effect::set_value(d, e1, SUMMON_TYPE_PENDULUM as i64);
    card::register_effect(d, c, e1, false);
    // register by default
    if reg != Some(false) {
        let e2 = effect::create_effect(d, c);
        effect::set_description(d, e2, 1160);
        effect::set_type(d, e2, EFFECT_TYPE_ACTIVATE);
        effect::set_code(d, e2, EVENT_FREE_CHAIN);
        effect::set_range(d, e2, LOCATION_HAND);
        card::register_effect(d, c, e2, false);
    }
}
/// `Pendulum.Filter(c, e, tp, lscale, rscale, lvchk)`
pub fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lscale: u32, rscale: u32, lvchk: bool) -> bool {
    let (lscale, rscale) = if lscale > rscale { (rscale, lscale) } else { (lscale, rscale) };
    let lv = match crate::card_info(d, c).pendulum_level {
        Some(l) => l,
        None => card::get_level(d, c),
    };
    (card::is_location(d, c, LOCATION_HAND) || (card::is_faceup(d, c) && card::is_type(d, c, TYPE_PENDULUM)))
        && (lvchk || (lv > lscale && lv < rscale) || !card::is_has_effect(d, c, 511004423, None).is_empty())
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_PENDULUM, tp, false, false, None, None, None)
        && !card::is_forbidden(d, c)
}
fn has(d: &mut Duel, c: CardId, code: u32) -> bool {
    !card::is_has_effect(d, c, code, None).is_empty()
}
/// `Pendulum.Condition()`: `function(e, c, inchain, re, rp)`
fn proc_condition() -> LuaFn {
    lua_fn(|d, a| {
        let e = a.e(0);
        let Some(c) = a.card(1) else { return true };
        let inchain = a.v(2).truthy();
        let tp = card::get_controler(d, c);
        let Some(rpz) = duel::get_field_card(d, tp, LOCATION_PZONE, 1) else { return false };
        if c == rpz || (!inchain && duel::get_flag_effect(d, tp, 10000000) > 0) {
            return false;
        }
        let lscale = card::get_left_scale(d, c);
        let rscale = card::get_right_scale(d, rpz);
        let mut loc = 0;
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
            loc += LOCATION_HAND;
        }
        if duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Type(TYPE_PENDULUM), None).0 > 0 {
            loc += LOCATION_EXTRA;
        }
        if loc == 0 {
            return false;
        }
        let g = duel::get_field_group(d, tp, loc, 0);
        let lvchk = has(d, c, 511007000) && has(d, rpz, 511007000);
        group::is_exists(d, &g, &|d, x| filter(d, x, e, tp, lscale, rscale, lvchk), 1, NO_EX)
    })
}
fn shared(v: &Value) -> GroupRef {
    match v {
        Value::SharedGroup(r) => *r,
        _ => crate::lua_error("attempt to index a nil value (local 'sg')"),
    }
}
/// `Pendulum.Operation()`: `function(e,tp,eg,ep,ev,re,r,rp,c,sg,inchain)` (fills the core's `sg`)
fn proc_operation() -> LuaFn {
    lua_fn(|d, a| {
        let (e, tp) = (a.e(0), a.p(1));
        let c = a.c(8);
        let sgr = shared(a.v(9));
        let inchain = a.v(10).truthy();
        let mut sg = d.group(sgr).clone();
        let rpz = duel::get_field_card(d, tp, LOCATION_PZONE, 1).nil();
        let lscale = card::get_left_scale(d, c);
        let rscale = card::get_right_scale(d, rpz);
        let mut ft1 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 as i64;
        let mut ft2 = duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Type(TYPE_PENDULUM), None).0 as i64;
        let mut ft = duel::get_usable_m_zone_count(d, tp, None) as i64;
        if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
            if ft1 > 0 {
                ft1 = 1;
            }
            if ft2 > 0 {
                ft2 = 1;
            }
            ft = 1;
        }
        let mut loc = 0;
        if ft1 > 0 {
            loc += LOCATION_HAND;
        }
        if ft2 > 0 {
            loc += LOCATION_EXTRA;
        }
        let lvchk = has(d, c, 511007000) && has(d, rpz, 511007000);
        let tg = duel::get_matching_group(d, Some(&|d, x| filter(d, x, e, tp, lscale, rscale, lvchk)), tp, loc, 0, NO_EX);
        let count_in = |d: &mut Duel, g: &CardSet, l: u32| group::filter_count(d, g, &|d, x| card::is_location(d, x, l), NO_EX) as i64;
        ft1 = ft1.min(count_in(d, &tg, LOCATION_HAND));
        ft2 = ft2.min(count_in(d, &tg, LOCATION_EXTRA));
        ft2 = ft2.min(aux::check_summon_gate_limit(d, tp).unwrap_or(ft2));
        loop {
            let ct1 = count_in(d, &tg, LOCATION_HAND);
            let ct2 = count_in(d, &tg, LOCATION_EXTRA);
            let mut ct = ft;
            if ct1 > ft1 {
                ct = ct.min(ft1);
            }
            if ct2 > ft2 {
                ct = ct.min(ft2);
            }
            let _ = ct;
            let mut loc = 0;
            if ft1 > 0 {
                loc += LOCATION_HAND;
            }
            if ft2 > 0 {
                loc += LOCATION_EXTRA;
            }
            let g = group::filter(d, &tg, &|d, x| card::is_location(d, x, loc), (&sg).into());
            if g.is_empty() || ft == 0 {
                break;
            }
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            let cancel = duel::is_summon_cancelable(d);
            let Some(tc) = group::select_unselect(d, &g, Some(&sg), tp, !sg.is_empty(), cancel, None, None) else { break };
            if sg.contains(tc) {
                sg.remove(tc);
                if card::is_location(d, tc, LOCATION_HAND) {
                    ft1 += 1;
                } else {
                    ft2 += 1;
                }
                ft += 1;
            } else {
                sg.insert(tc);
                if has(d, c, 511007000) || has(d, rpz, 511007000) {
                    if !filter(d, tc, e, tp, lscale, rscale, false) {
                        let pg = group::difference(&sg, &CardSet::from_card(tc));
                        let (ct0, ct3, ct4) = (pg.len() as i64, count_in(d, &pg, LOCATION_HAND), count_in(d, &pg, LOCATION_EXTRA));
                        group::remove_card(&mut sg, &pg);
                        ft1 += ct3;
                        ft2 += ct4;
                        ft += ct0;
                    } else {
                        let pg = group::filter(d, &sg, &|d, x| !filter(d, x, e, tp, lscale, rscale, false), NO_EX);
                        group::remove_card(&mut sg, &pg);
                        if !pg.is_empty() {
                            let first = group::get_first(d, &pg).nil();
                            if card::is_location(d, first, LOCATION_HAND) {
                                ft1 += 1;
                            } else {
                                ft2 += 1;
                            }
                            ft += 1;
                        }
                    }
                }
                if card::is_location(d, tc, LOCATION_HAND) {
                    ft1 -= 1;
                } else {
                    ft2 -= 1;
                }
                ft -= 1;
            }
            *d.group_mut(sgr) = sg.clone();
        }
        *d.group_mut(sgr) = sg.clone();
        if !sg.is_empty() {
            if !inchain {
                duel::register_flag_effect(d, tp, 10000000, RESET_PHASE + PHASE_END + RESET_SELF_TURN, 0, 1, 0);
            }
            duel::hint_selection(d, c, true);
            duel::hint_selection(d, rpz, true);
        }
    })
}

/// `Pendulum.PlayerCanGainAdditionalPendulumSummon(player, effect_flag)`
pub fn player_can_gain_additional_pendulum_summon(d: &mut Duel, player: u8, effect_flag: u32) -> bool {
    !(duel::has_flag_effect(d, player, CARD_ZEFRAATH, None) || duel::has_flag_effect(d, player, effect_flag, None)) && duel::is_turn_player(d, player) && !duel::is_phase(d, PHASE_END)
}
/// `pendulum_filter` of the additional Pendulum Summons: `function(c, e, tp, lscale, rscale)`
pub type PendFilter = Arc<dyn Fn(&mut Duel, CardId, EffectId, u8, u32, u32) -> bool + Send + Sync>;
/// `Pendulum.GrantAdditionalPendulumSummon(handler, pendulum_filter, player, pendulum_location,
/// hint_description, effect_description, owner_card_id, reset, reset_count)`
#[allow(clippy::too_many_arguments)]
pub fn grant_additional_pendulum_summon(d: &mut Duel, handler: CardId, pendulum_filter: Option<PendFilter>, player: u8, pendulum_location: u32, hint_description: Option<u64>, effect_description: Option<u64>, owner_card_id: u32, reset: Option<u32>, reset_count: Option<u16>) {
    if !player_can_gain_additional_pendulum_summon(d, player, owner_card_id) {
        return;
    }
    let reset = reset.unwrap_or(RESET_PHASE | PHASE_END);
    let reset_count = reset_count.unwrap_or(1);
    aux::register_client_hint(d, handler, 0, player, 1, 0, hint_description, reset, Some(reset_count));
    let flag_id = if owner_card_id == CARD_ZEFRAATH { owner_card_id + 1 } else { owner_card_id };
    duel::register_flag_effect(d, player, flag_id, reset, 0, reset_count, 0);
    // Create an extra Pendulum Summon effect
    let extra = create_additional_pendulum_summon_effect(d, handler, pendulum_filter.clone(), Some(pendulum_location), effect_description, owner_card_id, Some(reset), Some(reset_count));
    // Grant the above effect to cards in your Pendulum Zones
    let e1 = effect::create_effect(d, handler);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_GRANT);
    effect::set_target_range(d, e1, LOCATION_SZONE, 0);
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_PZONE))));
    effect::set_label_object(d, e1, Value::Effect(extra));
    effect::set_reset(d, e1, reset, reset_count);
    duel::register_effect(d, e1, player);
    if pendulum_location & LOCATION_EXTRA == 0 {
        return;
    }
    // Create an equivalent effect for "Harmonic Oscillation"
    let (harmonic, _) = create_harmonic_oscillation_effect(d, handler, pendulum_filter, effect_description, owner_card_id, None);
    // Grant the above effect to cards in your opponent's Pendulum Zones
    let e2 = effect::clone(d, e1);
    effect::set_target_range(d, e2, 0, LOCATION_SZONE);
    effect::set_label_object(d, e2, Value::Effect(harmonic));
    duel::register_effect(d, e2, player);
}
fn pend_and(d: &mut Duel, c: CardId, e: EffectId, tp: u8, l: u32, r: u32, f: &Option<PendFilter>) -> bool {
    filter(d, c, e, tp, l, r, false) && f.as_ref().is_none_or(|f| f(d, c, e, tp, l, r))
}
/// `Pendulum.CreateAdditionalPendulumSummonEffect(handler, pendulum_filter, pendulum_location,
/// effect_description, owner_card_id, reset, reset_count)`
#[allow(clippy::too_many_arguments)]
pub fn create_additional_pendulum_summon_effect(d: &mut Duel, handler: CardId, pendulum_filter: Option<PendFilter>, pendulum_location: Option<u32>, effect_description: Option<u64>, owner_card_id: u32, reset: Option<u32>, reset_count: Option<u16>) -> EffectId {
    let loc = pendulum_location.unwrap_or(LOCATION_HAND | LOCATION_EXTRA);
    let desc = effect_description.unwrap_or(1163);
    let reset = reset.unwrap_or(RESET_PHASE | PHASE_END);
    let reset_count = reset_count.unwrap_or(1);
    let (pf1, pf2) = (pendulum_filter.clone(), pendulum_filter);
    let cond = lua_fn(move |d, a| {
        let e = a.e(0);
        let Some(c) = a.card(1) else { return true };
        if a.v(2).truthy() {
            return false;
        }
        let tp = card::get_controler(d, c);
        if duel::has_flag_effect(d, tp, CARD_ZEFRAATH, None) {
            return false;
        }
        let mut zones = 0;
        if loc & LOCATION_HAND > 0 {
            zones = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
        }
        if loc & LOCATION_EXTRA > 0 {
            zones += duel::get_location_count_from_ex(d, tp, None, NO_EX, duel::ExtraCard::None, None).0;
        }
        if zones == 0 {
            return false;
        }
        let Some(rsc) = duel::get_field_card(d, tp, LOCATION_PZONE, 1) else { return false };
        if c == rsc {
            return false;
        }
        let (mut l, mut r) = (card::get_left_scale(d, c), card::get_right_scale(d, rsc));
        if l > r {
            std::mem::swap(&mut l, &mut r);
        }
        duel::is_existing_matching_card(d, Some(&|d, x| pend_and(d, x, e, tp, l, r, &pf1)), tp, loc, 0, 1, NO_EX)
    });
    let op = lua_fn(move |d, a| {
        let (e, tp) = (a.e(0), a.p(1));
        let c = a.c(8);
        let sgr = shared(a.v(9));
        let mut sg = d.group(sgr).clone();
        let rsc = duel::get_field_card(d, tp, LOCATION_PZONE, 1).nil();
        let (mut l, mut r) = (card::get_left_scale(d, c), card::get_right_scale(d, rsc));
        if l > r {
            std::mem::swap(&mut l, &mut r);
        }
        let mut mmz = 0i64;
        let mut emz = 0i64;
        if loc & LOCATION_HAND > 0 {
            mmz = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 as i64;
        }
        if loc & LOCATION_EXTRA > 0 {
            emz = duel::get_location_count_from_ex(d, tp, None, NO_EX, duel::ExtraCard::None, None).0 as i64;
        }
        let mut usable = duel::get_usable_m_zone_count(d, tp, None) as i64;
        if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
            if mmz > 0 {
                mmz = 1;
            }
            if emz > 0 {
                emz = 1;
            }
            usable = 1;
        }
        let avail = duel::get_matching_group(d, Some(&|d, x| pend_and(d, x, e, tp, l, r, &pf2)), tp, loc, 0, NO_EX);
        let count_in = |d: &mut Duel, g: &CardSet, lc: u32| group::filter_count(d, g, &|d, x| card::is_location(d, x, lc), NO_EX) as i64;
        mmz = mmz.min(count_in(d, &avail, LOCATION_HAND));
        emz = emz.min(count_in(d, &avail, LOCATION_EXTRA));
        emz = emz.min(aux::check_summon_gate_limit(d, tp).unwrap_or(emz));
        loop {
            if usable == 0 {
                break;
            }
            let mut tl = 0;
            if mmz > 0 {
                tl |= LOCATION_HAND;
            }
            if emz > 0 {
                tl |= LOCATION_EXTRA;
            }
            let rem = group::filter(d, &avail, &|d, x| card::is_location(d, x, tl), (&sg).into());
            if rem.is_empty() {
                break;
            }
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            let cancel = duel::is_summon_cancelable(d);
            let Some(sc) = group::select_unselect(d, &rem, Some(&sg), tp, !sg.is_empty(), cancel, None, None) else { break };
            let in_hand = card::is_location(d, sc, LOCATION_HAND);
            if sg.contains(sc) {
                sg.remove(sc);
                if in_hand { mmz += 1 } else { emz += 1 }
                usable += 1;
            } else {
                sg.insert(sc);
                if in_hand { mmz -= 1 } else { emz -= 1 }
                usable -= 1;
            }
            *d.group_mut(sgr) = sg.clone();
        }
        *d.group_mut(sgr) = sg.clone();
        if !sg.is_empty() {
            duel::register_flag_effect(d, tp, CARD_ZEFRAATH, RESET_PHASE | PHASE_END, 0, 1, 0);
            duel::hint(d, HINT_CARD as u8, 0, owner_card_id as u64);
            duel::hint_selection(d, c, true);
            duel::hint_selection(d, rsc, true);
        }
    });
    // This turn, you can conduct 1 Pendulum Summon of a monster(s) in addition to your Pendulum Summon
    let e1 = effect::create_effect(d, handler);
    effect::set_description(d, e1, desc);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC_G);
    effect::set_range(d, e1, LOCATION_PZONE);
    effect::set_condition(d, e1, Some(cond));
    effect::set_operation(d, e1, Some(op));
    effect::set_value(d, e1, SUMMON_TYPE_PENDULUM as i64);
    effect::set_reset(d, e1, reset, reset_count);
    e1
}
/// `Pendulum.CreateHarmonicOscillationEffect(handler, pendulum_filter, effect_description,
/// owner_card_id, pendulum_flag)`: (the effect, the effect that resets it)
pub fn create_harmonic_oscillation_effect(d: &mut Duel, handler: CardId, pendulum_filter: Option<PendFilter>, effect_description: Option<u64>, owner_card_id: u32, pendulum_flag: Option<u32>) -> (EffectId, EffectId) {
    let flag = pendulum_flag.unwrap_or(CARD_ZEFRAATH);
    let desc = effect_description.unwrap_or_else(|| aux::stringid(CARD_HARMONIC_OSCILLATION, 2));
    let (pf1, pf2) = (pendulum_filter.clone(), pendulum_filter);
    let cond = lua_fn(move |d, a| {
        let e = a.e(0);
        let Some(c) = a.card(1) else { return true };
        let inchain = a.v(2).truthy();
        let rp = a.p(4);
        let tp = effect::get_owner_player(d, e);
        if duel::get_location_count_from_ex(d, tp, None, NO_EX, duel::ExtraCard::None, None).0 <= 0 || (inchain && tp == 1 - rp) || (!inchain && duel::has_flag_effect(d, tp, flag, None)) || (inchain && flag != 10000000) {
            return false;
        }
        let Some(rsc) = duel::get_field_card(d, 1 - tp, LOCATION_PZONE, 1) else { return false };
        if !card::has_flag_effect(d, c, CARD_HARMONIC_OSCILLATION, None) || !card::has_flag_effect(d, rsc, CARD_HARMONIC_OSCILLATION, None) {
            return false;
        }
        if card::get_flag_effect_label(d, c, CARD_HARMONIC_OSCILLATION).first() != card::get_flag_effect_label(d, rsc, CARD_HARMONIC_OSCILLATION).first() {
            return false;
        }
        let (mut l, mut r) = (card::get_left_scale(d, c), card::get_right_scale(d, rsc));
        if l > r {
            std::mem::swap(&mut l, &mut r);
        }
        duel::is_existing_matching_card(d, Some(&|d, x| pend_and(d, x, e, tp, l, r, &pf1)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
    });
    let op = lua_fn(move |d, a| {
        let e = a.e(0);
        let c = a.c(8);
        let sgr = shared(a.v(9));
        let inchain = a.v(10).truthy();
        let tp = effect::get_owner_player(d, e);
        let rsc = duel::get_field_card(d, 1 - tp, LOCATION_PZONE, 1).nil();
        let (mut l, mut r) = (card::get_left_scale(d, c), card::get_right_scale(d, rsc));
        if l > r {
            std::mem::swap(&mut l, &mut r);
        }
        let mut ft = duel::get_location_count_from_ex(d, tp, None, NO_EX, duel::ExtraCard::None, None).0 as i64;
        if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
            ft = 1;
        }
        ft = ft.min(aux::check_summon_gate_limit(d, tp).unwrap_or(ft));
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let min = if duel::is_summon_cancelable(d) { 0 } else { 1 };
        let g = duel::select_matching_card(d, tp, Some(&|d, x| pend_and(d, x, e, tp, l, r, &pf2)), tp, LOCATION_EXTRA, 0, min, ft as u16, false, NO_EX);
        let mut sg = d.group(sgr).clone();
        if let Some(g) = g {
            group::add_card(&mut sg, &g);
        }
        *d.group_mut(sgr) = sg.clone();
        if !sg.is_empty() {
            if !inchain {
                duel::register_flag_effect(d, tp, flag, RESET_PHASE | PHASE_END, 0, 1, 0);
            }
            if flag != 10000000 {
                duel::hint(d, HINT_CARD as u8, 0, owner_card_id as u64);
            }
            duel::hint(d, HINT_CARD as u8, 0, CARD_HARMONIC_OSCILLATION as u64);
            duel::hint_selection(d, c, true);
            duel::hint_selection(d, rsc, true);
        }
    });
    let e1 = effect::create_effect(d, handler);
    effect::set_description(d, e1, desc);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_BOTH_SIDE, 0);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC_G);
    effect::set_range(d, e1, LOCATION_PZONE);
    effect::set_condition(d, e1, Some(cond));
    effect::set_operation(d, e1, Some(op));
    effect::set_value(d, e1, SUMMON_TYPE_PENDULUM as i64);
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // Reset the above effect if the right Pendulum Scale leaves the field
    let e2 = effect::create_effect(d, handler);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e2, EVENT_LEAVE_FIELD_P);
    effect::set_range(d, e2, LOCATION_PZONE);
    effect::set_operation(d, e2, Some(operation(move |d, _a| effect::reset(d, e1))));
    effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
    (e1, e2)
}
