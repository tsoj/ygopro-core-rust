//! Harpie Lady 1
use super::prelude::*;

pub const ID: u32 = 91932350;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_HARPIE_LADY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atkup
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTarget(aux.TargetBoolFunction(Card.IsAttribute,ATTRIBUTE_WIND))
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_attribute(d, c, ATTRIBUTE_WIND))));
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetValue(300)
    effect::set_value(d, e1, 300);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
