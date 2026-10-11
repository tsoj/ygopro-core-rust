//! Chain Thrasher
use super::prelude::*;

pub const ID: u32 = 88190453;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // multiattack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_EXTRA_ATTACK)
    effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
    // e1:SetValue(s.val)
    effect::set_value_fn(d, e1, effect_card_value(val));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.val(e,c)`
fn val(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(Card.IsCode,e:GetHandlerPlayer(),LOCATION_GRAVE,0,nil,id)
    let p = effect::get_handler_player(d, e);
    duel::get_matching_group_count(d, Some(&|d, c| card::is_code(d, c, &[ID])), p, LOCATION_GRAVE, 0, NO_EX) as i64
}
