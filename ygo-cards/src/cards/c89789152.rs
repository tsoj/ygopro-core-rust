//! Tachyon Spiral Galaxy
use super::prelude::*;

pub const ID: u32 = 89789152;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_GALAXY, SET_GALAXY_EYES_TACHYON_DRAGON],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Make 1 Dragon "Galaxy" monster unaffected by other card effects and prevent battle destruction
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    effect::set_hint_timing(d, e1, 0, Some(TIMING_BATTLE_START | TIMING_END_PHASE));
    card::register_effect(d, c, e1, false);
    // Can be activated from the hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_TRAP_ACT_IN_HAND);
    effect::set_condition(d, e2, Some(condition(handcon)));
    card::register_effect(d, c, e2, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_GALAXY as u16]) && card::is_race(d, c, RACE_DRAGON)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    let _g = duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d).nil();
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        // Unaffected by other card effects
        let e1 = effect::create_effect(d, c);
        effect::set_description(d, e1, 3100);
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
        effect::set_value_fn(d, e1, lua_fn(efilter));
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e1, false);
        // Cannot be destroyed by battle
        let e2 = effect::create_effect(d, c);
        effect::set_description(d, e2, 3000);
        effect::set_property(d, e2, EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
        effect::set_value(d, e2, 1);
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e2, false);
    }
}
/// `function(e,re)`
fn efilter(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    let owner = effect::get_owner(d, a.e(1)).nil();
    h != owner
}
fn hfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_GALAXY_EYES_TACHYON_DRAGON as u16])
}
fn handcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&hfilter), effect::get_handler_player(d, a.e), LOCATION_MZONE, 0, 1, NO_EX)
}
