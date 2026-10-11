//! Assault Revival
use super::prelude::*;

pub const ID: u32 = 56252810;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ASSAULT_MODE], listed_names: &[CARD_ASSAULT_MODE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy all your monsters, special summon 1 "/Assault Mode" monster from GY
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[CARD_ASSAULT_MODE]) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&cfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_ASSAULT_MODE as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && filter(d, chkc, e, tp);
    }
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, None, tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // local dg=Duel.GetMatchingGroup(aux.TRUE,tp,LOCATION_MZONE,0,nil)
    let dg = duel::get_matching_group(d, None, tp, LOCATION_MZONE, 0, NO_EX);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    let n = dg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), n, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let dg = duel::get_matching_group(d, None, tp, LOCATION_MZONE, 0, NO_EX);
    // if Duel.Destroy(dg,REASON_EFFECT)~=#dg then return end
    if duel::destroy(d, dg.clone(), REASON_EFFECT, None, None) != dg.len() as i32 {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    let tc = duel::first_target(d).nil();
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::break_effect(d);
        if duel::special_summon(d, tc, 0, tp, tp, true, false, POS_FACEUP as u8, None) == 0 {
            return;
        }
        // Negate its effects
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_DISABLE);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e1, true);
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e2, true);
        // Cannot be tributed
        let e3 = effect::create_effect(d, c);
        effect::set_description(d, e3, 3303);
        effect::set_property(d, e3, EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e3, EFFECT_UNRELEASABLE_SUM);
        effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
        effect::set_value(d, e3, 1);
        card::register_effect(d, tc, e3, true);
        let e4 = effect::create_effect(d, c);
        effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e4, EFFECT_UNRELEASABLE_NONSUM);
        effect::set_reset(d, e4, RESET_EVENT | RESETS_STANDARD, 1);
        effect::set_value(d, e4, 1);
        card::register_effect(d, tc, e4, true);
        // Banish it if it leaves the field
        let e5 = effect::create_effect(d, c);
        effect::set_description(d, e5, 3300);
        effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e5, EFFECT_LEAVE_FIELD_REDIRECT);
        effect::set_property(d, e5, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_reset(d, e5, RESET_EVENT | RESETS_REDIRECT, 1);
        effect::set_value(d, e5, LOCATION_REMOVED as i64);
        card::register_effect(d, tc, e5, true);
    }
}
