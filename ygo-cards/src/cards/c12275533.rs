//! Trifortressops
use super::prelude::*;

pub const ID: u32 = 12275533;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon itself from hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
    // Count the number of times opponent has summoned monster(s)
    aux::global_check(d, ID, |d| {
        let ge1 = effect::create_effect(d, c);
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, ge1, EVENT_SPSUMMON_SUCCESS);
        effect::set_operation(d, ge1, Some(operation(checkop)));
        duel::register_effect(d, ge1, 0);
        let ge2 = effect::clone(d, ge1);
        effect::set_code(d, ge2, EVENT_SUMMON_SUCCESS);
        duel::register_effect(d, ge2, 0);
        let ge3 = effect::clone(d, ge1);
        effect::set_code(d, ge3, EVENT_FLIP_SUMMON_SUCCESS);
        duel::register_effect(d, ge3, 0);
    });
}
fn checkop(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg.clone().nil();
    let _ = group::get_first(d, &eg);
    for tc in group::iter(d, &eg) {
        let p = card::get_summon_player(d, tc);
        duel::register_flag_effect(d, p, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    }
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_flag_effect(d, 1 - a.tp, ID) >= 3
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return !card::is_status(d, h, STATUS_CHAINING)
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, h, a.e, 0, a.tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    if duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
        // Unaffected by other card effects
        let e1 = effect::create_effect(d, c);
        effect::set_description(d, e1, 3100);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_range(d, e1, LOCATION_MZONE);
        effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        effect::set_value_fn(d, e1, lua_fn(efilter));
        card::register_effect(d, c, e1, false);
        // Loses 500 DEF during each standby phase
        let e2 = effect::create_effect(d, c);
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_range(d, e2, LOCATION_MZONE);
        effect::set_count_limit(d, e2, 1, 0, 0, 0);
        effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        effect::set_operation(d, e2, Some(operation(adjustop)));
        card::register_effect(d, c, e2, false);
    }
}
fn efilter(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let te = a.e(1);
    effect::get_owner(d, te) != effect::get_owner(d, e)
}
fn adjustop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_COPY_INHERIT, 0);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
    effect::set_code(d, e1, EFFECT_UPDATE_DEFENSE);
    effect::set_value(d, e1, -500);
    card::register_effect(d, c, e1, false);
}
