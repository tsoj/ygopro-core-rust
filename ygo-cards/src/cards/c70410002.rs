//! The Accumulator
use super::prelude::*;

pub const ID: u32 = 70410002;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atk up
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_value_fn(d, e1, effect_card_value(atkval));
    card::register_effect(d, c, e1, false);
}
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsLinkMonster()
    card::is_faceup(d, c) && card::is_link_monster(d, c)
}
fn atkval(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    // local g=Duel.GetMatchingGroup(s.atkfilter,e:GetHandlerPlayer(),LOCATION_MZONE,LOCATION_MZONE,nil)
    let p = effect::get_handler_player(d, e);
    let g = duel::get_matching_group(d, Some(&atkfilter), p, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // return g:GetSum(Card.GetLink)*300
    group::get_sum(d, &g, &|d, c| card::get_link(d, c) as i64) * 300
}
