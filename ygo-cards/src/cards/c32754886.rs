//! Black Illusion
use super::prelude::*;

pub const ID: u32 = 32754886;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Grant protection to your DARK spellcaster monsters with 2000+ ATK
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_race(d, c, RACE_SPELLCASTER) && card::is_attribute(d, c, ATTRIBUTE_DARK) && card::is_attack_above(d, c, 2000)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_MZONE, 0, NO_EX);
    let _tc = group::get_first(d, &g);
    for tc in group::iter(d, &g) {
        duel::negate_related_chain(d, tc, RESET_TURN_SET);
        // Cannot be destroyed by battle
        let e1 = effect::create_effect(d, c);
        effect::set_description(d, e1, 3000);
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
        effect::set_value(d, e1, 1);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e1, false);
        // Negate their effects
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e2, EFFECT_DISABLE);
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e2, false);
        let e3 = effect::create_effect(d, c);
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e3, EFFECT_DISABLE_EFFECT);
        effect::set_value(d, e3, RESET_TURN_SET as i64);
        effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e3, false);
        // Unaffected by opponent's card effects
        let e4 = effect::create_effect(d, c);
        effect::set_description(d, e4, 3110);
        effect::set_property(d, e4, EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e4, EFFECT_IMMUNE_EFFECT);
        effect::set_value_fn(d, e4, lua_fn(efilter));
        effect::set_reset(d, e4, RESETS_STANDARD_PHASE_END, 1);
        effect::set_owner_player(d, e4, a.tp);
        card::register_effect(d, tc, e4, false);
    }
}
/// `function(e,re) return e:GetOwnerPlayer()~=re:GetOwnerPlayer() end`
fn efilter(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let re = a.e(1);
    effect::get_owner_player(d, e) != effect::get_owner_player(d, re)
}
