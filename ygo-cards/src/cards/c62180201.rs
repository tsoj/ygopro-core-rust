//! The Wicked Dreadroot
use super::prelude::*;

pub const ID: u32 = 62180201;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon with 3 tribute
    let _e1 = aux::add_normal_summon_procedure(d, c, true, false, Some(3), Some(3), None, None, None, None);
    let _e2 = aux::add_normal_set_procedure(d, c, true, false, Some(3), Some(3), None, None, None, None);
    // cannot special summon
    let e3 = effect::create_effect(d, c);
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e3, EFFECT_SPSUMMON_CONDITION);
    card::register_effect(d, c, e3, false);
    // half atk
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    effect::set_code(d, e4, EFFECT_SET_ATTACK_FINAL);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    effect::set_target_range(d, e4, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e4, Some(effect_card_fn(atktg)));
    effect::set_value_fn(d, e4, effect_card_value(atkval));
    card::register_effect(d, c, e4, false);
    let e5 = effect::clone(d, e4);
    effect::set_code(d, e5, EFFECT_SET_DEFENSE_FINAL);
    effect::set_value_fn(d, e5, effect_card_value(defval));
    card::register_effect(d, c, e5, false);
}
fn atktg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c~=e:GetHandler()
    let h = effect::get_handler(d, e).nil();
    c != h
}
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return math.ceil(c:GetAttack()/2)
    (card::get_attack(d, c) as f64 / 2.0).ceil() as i64
}
fn defval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return math.ceil(c:GetDefense()/2)
    (card::get_defense(d, c) as f64 / 2.0).ceil() as i64
}
