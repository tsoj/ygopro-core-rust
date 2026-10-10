//! Helios - The Primordial Sun
use super::prelude::*;

pub const ID: u32 = 54493213;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // ATK
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_SET_ATTACK);
    effect::set_value_fn(d, e1, effect_card_value(value));
    card::register_effect(d, c, e1, false);
    // DEF
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_SET_DEFENSE);
    card::register_effect(d, c, e2, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsMonster()
    card::is_faceup(d, c) && card::is_monster(d, c)
}
/// `s.value(e,c)`
fn value(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(s.filter,c:GetControler(),LOCATION_REMOVED,LOCATION_REMOVED,nil)*100
    let p = card::get_controler(d, c);
    duel::get_matching_group_count(d, Some(&filter), p, LOCATION_REMOVED, LOCATION_REMOVED, NO_EX) as i64 * 100
}
