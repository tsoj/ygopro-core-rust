//! Magicians' Combination
use super::prelude::*;

pub const ID: u32 = 86509711;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_DARK_MAGICIAN, CARD_DARK_MAGICIAN_GIRL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Negate that effect
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON | CATEGORY_DISABLE);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_CHAINING);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(negcon)));
    effect::set_cost(d, e2, Some(cost_target(negcost)));
    effect::set_target(d, e2, Some(cost_target(negtg)));
    effect::set_operation(d, e2, Some(operation(negop)));
    card::register_effect(d, c, e2, false);
    // Destroy 1 card on the field
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_DESTROY);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE | EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    effect::set_condition(d, e3, Some(condition(descon)));
    effect::set_target(d, e3, Some(cost_target(destg)));
    effect::set_operation(d, e3, Some(operation(desop)));
    card::register_effect(d, c, e3, false);
}
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    !card::is_status(d, h, STATUS_DESTROY_CONFIRMED) && duel::is_chain_disablable(d, a.ev as u8)
}
fn cfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    if !(card::is_code(d, c, &[CARD_DARK_MAGICIAN, CARD_DARK_MAGICIAN_GIRL]) && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0) {
        return false;
    }
    let code = card::get_code(d, c)[0];
    duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, code)), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code: u32) -> bool {
    card::is_code(d, c, &[CARD_DARK_MAGICIAN, CARD_DARK_MAGICIAN_GIRL])
        && !card::is_code(d, c, &[code])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn negcost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&|d, x| cfilter(d, x, e, tp)), 1, 1, false, None, NO_EX);
    }
    let g = duel::select_release_group_cost(d, tp, Some(&|d, x| cfilter(d, x, e, tp)), 1, 1, false, None, NO_EX);
    let tc = group::get_first(d, &g).nil();
    let code = card::get_code(d, tc)[0];
    effect::set_label(d, e, &[code as i64]);
    duel::release(d, g, REASON_COST, None);
    true
}
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    true
}
fn negop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let h = effect::get_handler(d, e).nil();
    if !card::is_relate_to_effect(d, h, e) {
        return;
    }
    let code = effect::get_label(d, e) as u32;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let sg = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(|d, x| spfilter(d, x, e, tp, code))), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    if !sg.is_empty() && duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        duel::negate_effect(d, a.ev as u8);
    }
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, c, LOCATION_SZONE) && card::is_previous_position(d, c, POS_FACEUP as u32)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let dg = duel::get_field_group(d, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD);
    if a.chk == Some(0) {
        return !dg.is_empty();
    }
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), 1, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    let dg = duel::select_matching_card(d, a.tp, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    if !dg.is_empty() {
        duel::destroy(d, dg, REASON_EFFECT, None, None);
    }
}
