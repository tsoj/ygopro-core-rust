//! The A. Forces
use super::prelude::*;

pub const ID: u32 = 403847;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // atk up
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    effect::set_target(d, e2, Some(effect_card_fn(tg)));
    effect::set_value_fn(d, e2, effect_card_value(val));
    card::register_effect(d, c, e2, false);
}
/// `s.tg(e,c)`
fn tg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsRace(RACE_WARRIOR)
    card::is_race(d, c, RACE_WARRIOR)
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_WARRIOR|RACE_SPELLCASTER)
    card::is_faceup(d, c) && card::is_race(d, c, RACE_WARRIOR | RACE_SPELLCASTER)
}
/// `s.val(e,c)`
fn val(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(s.filter,c:GetControler(),LOCATION_MZONE,0,nil)*200
    let p = card::get_controler(d, c);
    duel::get_matching_group_count(d, Some(&filter), p, LOCATION_MZONE, 0, NO_EX) as i64 * 200
}
