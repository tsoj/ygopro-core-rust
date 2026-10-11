//! Royal Swamp Eel
use super::prelude::*;

pub const ID: u32 = 37953640;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SYNCHRO_MAT_RESTRICTION)
    effect::set_code(d, e1, EFFECT_SYNCHRO_MAT_RESTRICTION);
    // e1:SetValue(aux.TargetBoolFunction(Card.IsRace,RACE_FISH))
    effect::set_value_fn(d, e1, effect_card_fn(|d, _e, c| card::is_race(d, c, RACE_FISH)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
