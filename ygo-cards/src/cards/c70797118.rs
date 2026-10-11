//! 雷電娘々
//! Thunder Nyan Nyan
use super::prelude::*;

pub const ID: u32 = 70797118;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy itself if you control a non-LIGHT monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_SELF_DESTROY)
    effect::set_code(d, e1, EFFECT_SELF_DESTROY);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(function(e) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsAttributeExcept,ATTRIBUTE_LIGHT),e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil) end)
    effect::set_condition(d, e1, Some(condition(cond)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `function(e) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsAttributeExcept,ATTRIBUTE_LIGHT),e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil) end`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let hp = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(
        d,
        Some(&|d, c| card::is_faceup(d, c) && card::is_attribute_except(d, c, ATTRIBUTE_LIGHT, None, 0, None)),
        hp,
        LOCATION_MZONE,
        0,
        1,
        NO_EX,
    )
}
