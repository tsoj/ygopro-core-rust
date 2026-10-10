//! Bystial Druiswurm
use super::prelude::*;

pub const ID: u32 = 6637331;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself from the hand (Ignition)
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(|d, a| !spquickcon(d, a))));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Special Summon itself from the hand (Quick if the opponent controls monsters)
    let e2 = effect::clone(d, e1);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER_E));
    effect::set_condition(d, e2, Some(condition(spquickcon)));
    card::register_effect(d, c, e2, false);
    // Send 1 Special Summoned monster the opponent controls to the GY
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_TOGRAVE);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_condition(d, e3, Some(condition(|d, a| {
        let c = effect::get_handler(d, a.e).nil();
        card::is_previous_location(d, c, LOCATION_ONFIELD)
    })));
    effect::set_target(d, e3, Some(cost_target(tgtg)));
    effect::set_operation(d, e3, Some(operation(tgop)));
    card::register_effect(d, c, e3, false);
}
/// `s.spfilter(c,tp)`
fn spfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_attribute(d, c, ATTRIBUTE_LIGHT + ATTRIBUTE_DARK)
        && card::is_able_to_remove(d, c, None, None, None)
        && aux::sp_elim_filter(d, c, true, false)
        && card::is_faceup(d, c)
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE | LOCATION_GRAVE) && spfilter(d, chkc, tp);
    }
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| spfilter(d, x, tp)), tp, LOCATION_MZONE | LOCATION_GRAVE, LOCATION_GRAVE, 1, NO_EX)
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let g = duel::select_target(d, tp, Some(&|d, x| spfilter(d, x, tp)), tp, LOCATION_MZONE | LOCATION_GRAVE, LOCATION_GRAVE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, tp, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, LOCATION_HAND as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0 && card::is_location(d, tc, LOCATION_REMOVED) && card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.spquickcon(e,tp,eg,ep,ev,re,r,rp)`
fn spquickcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_field_group_count(d, a.tp, 0, LOCATION_MZONE) > 0
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_special_summoned(d, c) && card::is_able_to_grave(d, c, None)
}
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && tgfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tgfilter), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_target(d, tp, Some(&tgfilter), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, g, 1, tp, 0);
    true
}
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::send_to_grave(d, tc, REASON_EFFECT, None, None);
    }
}
