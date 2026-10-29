//! Colosseum - Cage of the Gladiator Beasts
use super::prelude::*;

pub const ID: u32 = 52518793;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GLADIATOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_counter_permit(d, c, 0x7, None, None);
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Add counter
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_TRIGGER_F + EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_condition(d, e2, Some(condition(accon)));
    effect::set_operation(d, e2, Some(operation(acop)));
    card::register_effect(d, c, e2, false);
    // atkup
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e3, LOCATION_FZONE);
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_GLADIATOR as u16]))));
    effect::set_value_fn(d, e3, effect_value(atkval));
    card::register_effect(d, c, e3, false);
    let e4 = effect::clone(d, e3);
    effect::set_code(d, e4, EFFECT_UPDATE_DEFENSE);
    card::register_effect(d, c, e4, false);
    // Destroy replace
    let e5 = effect::create_effect(d, c);
    effect::set_type(d, e5, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    effect::set_code(d, e5, EFFECT_DESTROY_REPLACE);
    effect::set_property(d, e5, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e5, LOCATION_FZONE);
    effect::set_target(d, e5, Some(cost_target(desreptg)));
    effect::set_operation(d, e5, Some(operation(desrepop)));
    card::register_effect(d, c, e5, false);
}
fn atkval(d: &mut Duel, e: EffectId) -> i64 {
    let h = effect::get_handler(d, e).nil();
    card::get_counter(d, h, 0x7) as i64 * 100
}
fn accon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| card::is_previous_location(d, c, LOCATION_DECK), 1, NO_EX)
}
fn acop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    card::add_counter(d, h, 0x7, 1, false);
}
fn desreptg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return !card::is_reason(d, h, REASON_REPLACE + REASON_RULE)
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_code(d, c, &[ID])), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::select_effect_yes_no(d, a.tp, h, Some(96))
}
fn desrepop(d: &mut Duel, a: &EffArgs) {
    duel::discard_hand(d, a.tp, Some(&|d, c| card::is_code(d, c, &[ID])), 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
}
