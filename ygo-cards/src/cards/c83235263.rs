//! Tyranno Infinity
use super::prelude::*;

pub const ID: u32 = 83235263;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // base attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SET_BASE_ATTACK)
    effect::set_code(d, e1, EFFECT_SET_BASE_ATTACK);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(s.atkval)
    effect::set_value_fn(d, e1, effect_card_value(|d, _e, c| atkval(d, c)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsRace,RACE_DINOSAUR),c:GetControler(),LOCATION_REMOVED,0,nil)*1000
    let tp = card::get_controler(d, c);
    duel::get_matching_group_count(d, Some(&|d, x| card::is_faceup(d, x) && card::is_race(d, x, RACE_DINOSAUR)), tp, LOCATION_REMOVED, 0, NO_EX) as i64 * 1000
}
