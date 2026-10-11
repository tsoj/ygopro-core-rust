//! Tenshin
use super::prelude::*;

pub const ID: u32 = 59563768;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // This card gains 400 ATK for each face-up Level 2 monster you control
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(function(e,c) return 400*Duel.GetMatchingGroupCount(...) end)
    effect::set_value_fn(d, e1, effect_card_value(tenshin_value));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function(e,c) return 400*Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsLevel,2),e:GetHandlerPlayer(),LOCATION_MZONE,0,nil) end
fn tenshin_value(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    let p = effect::get_handler_player(d, e);
    400 * duel::get_matching_group_count(d, Some(&|d, x| card::is_faceup(d, x) && card::is_level(d, x, &[2])), p, LOCATION_MZONE, 0, NO_EX) as i64
}
