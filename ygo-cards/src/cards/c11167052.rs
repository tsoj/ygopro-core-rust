//! Elemental Training
use super::prelude::*;

pub const ID: u32 = 11167052;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[61557074], listed_series: &[SET_ELEMENTSABER, SET_ELEMENTAL_LORD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    card::register_effect(d, c, e1, false);
    // indes/untarget
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target_range(d, e2, LOCATION_ONFIELD, 0);
    effect::set_target(d, e2, Some(effect_card_fn(intg)));
    effect::set_value_fn(d, e2, aux::tgoval());
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
    effect::set_value(d, e3, 1);
    card::register_effect(d, c, e3, false);
    // special summon
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 0));
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e4, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e4, 0, Some(TIMING_END_PHASE));
    effect::set_range(d, e4, LOCATION_SZONE);
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    effect::set_cost(d, e4, Some(cost_target(spcost)));
    effect::set_target(d, e4, Some(cost_target(sptg)));
    effect::set_operation(d, e4, Some(operation(spop)));
    card::register_effect(d, c, e4, false);
    // discard & salvage
    let e5 = effect::create_effect(d, c);
    effect::set_description(d, e5, aux::stringid(ID, 1));
    effect::set_category(d, e5, CATEGORY_HANDES + CATEGORY_TOHAND);
    effect::set_type(d, e5, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e5, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e5, 0, Some(TIMING_END_PHASE));
    effect::set_range(d, e5, LOCATION_SZONE);
    effect::set_cost(d, e5, Some(cost_target(thcost)));
    effect::set_target(d, e5, Some(cost_target(thtg)));
    effect::set_operation(d, e5, Some(operation(thop)));
    card::register_effect(d, c, e5, false);
}
fn intg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_code(d, c, &[61557074])
}
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    effect::set_label(d, a.e, &[100]);
    if a.chk == Some(0) {
        return true;
    }
    true
}
fn filter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    let att = card::get_original_attribute(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| filter2(d, x, e, tp, att)), tp, LOCATION_DECK, 0, 1, NO_EX)
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
fn filter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8, att: u32) -> bool {
    card::is_set_card(d, c, &[SET_ELEMENTSABER as u16])
        && card::get_original_attribute(d, c) != att
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        if effect::get_label(d, e) != 100 {
            return false;
        }
        effect::set_label(d, e, &[0]);
        return duel::check_release_group_cost(d, tp, Some(&|d, c| filter1(d, c, e, tp)), 1, 1, false, None, NO_EX);
    }
    let rg = duel::select_release_group_cost(d, tp, Some(&|d, c| filter1(d, c, e, tp)), 1, 1, false, None, NO_EX);
    let first = group::get_first(d, &rg);
    effect::set_label_object(d, e, first.map_or(Value::Nil, Value::Card));
    duel::release(d, rg, REASON_COST, None);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    let lo = effect::get_label_object(d, a.e).as_card().nil();
    let att = card::get_original_attribute(d, lo);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| filter2(d, c, a.e, tp, att)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_grave_as_cost(d, c) && card::is_status(d, c, STATUS_EFFECT_ENABLED);
    }
    duel::send_to_grave(d, c, REASON_COST, None, None);
    true
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_ELEMENTAL_LORD as u16]) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let hg = duel::get_field_group(d, tp, LOCATION_HAND, 0);
    let ct = hg.len();
    if a.chk == Some(0) {
        return ct > 0 && duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_GRAVE, 0, ct as u32, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_HANDES, Some(hg), ct as u8, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, ct as u8, tp, LOCATION_GRAVE as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let hg = duel::get_field_group(d, tp, LOCATION_HAND, 0);
    let ct = duel::send_to_grave(d, hg, REASON_EFFECT | REASON_DISCARD, None, None);
    if ct <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(thfilter)), tp, LOCATION_GRAVE, 0, ct as u16, ct as u16, false, NO_EX).nil();
    if !g.is_empty() {
        duel::break_effect(d);
        duel::send_to_hand(d, g, None, REASON_EFFECT, None);
    }
}
