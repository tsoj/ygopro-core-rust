//! Maliss <C> TB-11
use super::prelude::*;

pub const ID: u32 = 57111661;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MALISS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Maliss" monster from your Deck
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Can be activated the turn it was Set by banishing 1 face-up "Maliss" monster you control
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_SET_AVAILABLE, 0);
    effect::set_code(d, e2, EFFECT_TRAP_ACT_IN_SET_TURN);
    effect::set_value_fn(d, e2, lua_fn(|d, a: Args| {
        effect::set_label(d, a.e(0), &[1]);
        Value::Nil
    }));
    effect::set_condition(d, e2, Some(lua_fn(|d, a: Args| e2_cond(d, a.e(0)))));
    card::register_effect(d, c, e2, false);
    effect::set_label_object(d, e1, Value::Effect(e2));
    // Workaround: Unaffected by other EFFECT_TRAP_ACT_IN_SET_TURN effects while the field is full
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e3, EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e3, EFFECT_IMMUNE_EFFECT);
    effect::set_condition(d, e3, Some(lua_fn(|d, a: Args| {
        let e = a.e(0);
        let tp = effect::get_handler_player(d, e);
        let loc = loc_of(d, tp);
        !duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp, None, false)), tp, loc, 0, 1, NO_EX)
    })));
    effect::set_value_fn(d, e3, lua_fn(move |d, a: Args| {
        let te = a.e(1);
        effect::get_code(d, te) == EFFECT_TRAP_ACT_IN_SET_TURN && te != e2
    }));
    card::register_effect(d, c, e3, false);
}
fn loc_of(d: &mut Duel, tp: u8) -> u32 {
    if duel::get_field_group_count(d, tp, 0, LOCATION_ONFIELD) >= 3 {
        LOCATION_DECK | LOCATION_EXTRA
    } else {
        LOCATION_DECK
    }
}
fn e2_cond(d: &mut Duel, e: EffectId) -> bool {
    let tp = effect::get_handler_player(d, e);
    let loc = loc_of(d, tp);
    duel::is_existing_matching_card(d, Some(&|d, c| spcostfilter(d, c, loc, e, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
fn spcostfilter(d: &mut Duel, c: CardId, loc: u32, e: EffectId, tp: u8) -> bool {
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_MALISS as u16])
        && card::is_able_to_remove(d, c, Some(tp), None, None)
        && duel::is_existing_matching_card(d, Some(&|d, cc| spfilter(d, cc, e, tp, Some(c), false)), tp, loc, 0, 1, NO_EX)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: Option<CardId>, ignore_zone_chk: bool) -> bool {
    if !(card::is_monster(d, c)
        && card::is_set_card(d, c, &[SET_MALISS as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None))
    {
        return false;
    }
    if mc.is_some() || !ignore_zone_chk {
        if card::is_location(d, c, LOCATION_EXTRA) {
            return card::is_type(d, c, TYPE_LINK)
                && duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 > 0;
        } else {
            return duel::get_m_zone_count(d, tp, mc.into(), None, None, None).0 > 0;
        }
    }
    true
}
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    effect::set_label(d, e, &[100]);
    let label_obj = effect::get_label_object(d, e).as_effect().nil();
    if a.chk == Some(0) {
        effect::set_label(d, label_obj, &[0]);
        return true;
    }
    if effect::get_label(d, label_obj) > 0 {
        effect::set_label(d, label_obj, &[0]);
        let loc = loc_of(d, tp);
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        let g = duel::select_matching_card(d, tp, Some(&|d, c| spcostfilter(d, c, loc, e, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
        duel::remove(d, &g, POS_FACEUP as u8, REASON_COST, None, None);
    }
    true
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        let h = effect::get_handler(d, e).nil();
        let ignore_zone_chk = card::is_status(d, h, STATUS_SET_TURN) && effect::get_label(d, e) == 100;
        effect::set_label(d, e, &[0]);
        let loc = loc_of(d, tp);
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp, None, ignore_zone_chk)), tp, loc, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_DECK | LOCATION_EXTRA) as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let loc = loc_of(d, tp);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, None, false)), tp, loc, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() && duel::special_summon(d, &g, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        let tc = group::get_first(d, &g).nil();
        let h = effect::get_handler(d, e).nil();
        // Cannot attack
        let e1 = effect::create_effect(d, h);
        effect::set_description(d, e1, 3206);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e1, false);
        // Cannot activate its effects
        let e2 = effect::clone(d, e1);
        effect::set_description(d, e2, 3302);
        effect::set_code(d, e2, EFFECT_CANNOT_TRIGGER);
        card::register_effect(d, tc, e2, false);
    }
}
