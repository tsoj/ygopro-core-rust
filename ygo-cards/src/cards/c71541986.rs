//! Xyz Dimension Splash
use super::prelude::*;

pub const ID: u32 = 71541986;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_REMOVE);
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, c, LOCATION_ONFIELD) && card::is_previous_position(d, c, POS_FACEDOWN)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_attribute(d, c, ATTRIBUTE_WATER) && card::get_level(d, c) == 8 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        let blocked = duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some();
        return !blocked
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 1
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 2, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, tp, LOCATION_DECK as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 2 {
        return;
    }
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, NO_EX);
    if g.len() < 2 {
        return;
    }
    let c = effect::get_handler(d, e).nil();
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let sg = group::select(d, &g, tp, 2, 2, false, NO_EX).nil();
    let _ = group::get_first(d, &sg);
    for tc in group::iter(d, &sg) {
        duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_CANNOT_ATTACK_ANNOUNCE);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e1, false);
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e2, EFFECT_DISABLE);
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e2, false);
        let e3 = effect::create_effect(d, c);
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e3, EFFECT_DISABLE_EFFECT);
        effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e3, false);
        let e4 = effect::create_effect(d, c);
        effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
        effect::set_range(d, e4, LOCATION_MZONE);
        effect::set_code(d, e4, EFFECT_UNRELEASABLE_SUM);
        effect::set_value(d, e4, 1);
        effect::set_reset(d, e4, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e4, false);
        let e5 = effect::clone(d, e4);
        effect::set_code(d, e5, EFFECT_UNRELEASABLE_NONSUM);
        card::register_effect(d, tc, e5, false);
    }
    duel::special_summon_complete(d);
}
