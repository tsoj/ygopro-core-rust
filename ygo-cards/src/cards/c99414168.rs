//! Elemental Mistress Doriado
use super::prelude::*;

pub const ID: u32 = 99414168;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[23965037],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Attribute
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_ADD_ATTRIBUTE)
    effect::set_code(d, e1, EFFECT_ADD_ATTRIBUTE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(ATTRIBUTE_EARTH|ATTRIBUTE_WIND|ATTRIBUTE_FIRE|ATTRIBUTE_WATER)
    effect::set_value(d, e1, (ATTRIBUTE_EARTH | ATTRIBUTE_WIND | ATTRIBUTE_FIRE | ATTRIBUTE_WATER) as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
