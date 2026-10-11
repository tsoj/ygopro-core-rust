//! Jurrac Brachis
use super::prelude::*;

pub const ID: u32 = 8594079;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // battle indestructable
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    // e2:SetCondition(s.indcon)
    effect::set_condition(d, e2, Some(condition(indcon)));
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.indcon(e)
fn indcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_JURRAC),0,LOCATION_MZONE,LOCATION_MZONE,1,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    duel::is_existing_matching_card(
        d,
        Some(&aux::faceup_filter(|d, x| card::is_set_card(d, x, &[SET_JURRAC as u16]))),
        0,
        LOCATION_MZONE,
        LOCATION_MZONE,
        1,
        h.into(),
    )
}
