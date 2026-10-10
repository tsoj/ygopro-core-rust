//! Maiden of the Aqua
use super::prelude::*;

pub const ID: u32 = 17214465;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_UMI],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_CHANGE_ENVIRONMENT)
    effect::set_code(d, e1, EFFECT_CHANGE_ENVIRONMENT);
    // e1:SetValue(CARD_UMI)
    effect::set_value(d, e1, CARD_UMI as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
