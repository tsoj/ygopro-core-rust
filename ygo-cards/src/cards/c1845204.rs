//! Instant Fusion
use super::prelude::*;

pub const ID: u32 = 1845204;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon 1 level 5 or lower fusion monster from extra deck
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(1000))));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_type(d, c, TYPE_FUSION)
        && card::get_level(d, c) <= 5
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_FUSION, tp, false, false, None, None, None)
        && card::check_fusion_material(d, c, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let Some(tc) = group::get_first(d, &g) else { return };
    card::set_material(d, tc, CardSet::new());
    if duel::special_summon(d, tc, SUMMON_TYPE_FUSION, tp, tp, false, false, POS_FACEUP as u8, None) != 0 {
        // Cannot attack
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_description(d, e1, 3206);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
        effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e1, true);
        card::complete_procedure(d, tc);
        // Destroy it during end phase
        aux::delayed_operation(d, tc, PHASE_END, ID, e, tp, Some(std::sync::Arc::new(destroy_oper)), None, Some(0), None, None, None);
    }
}
/// `function(ag) Duel.Destroy(ag,REASON_EFFECT) end`
fn destroy_oper(d: &mut Duel, ag: &CardSet, _a: &EffArgs) -> bool {
    duel::destroy(d, ag.clone(), REASON_EFFECT, None, None);
    true
}
