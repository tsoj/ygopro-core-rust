//! Patchwork Fluffal
use super::prelude::*;

pub const ID: u32 = 81481818;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // setcode
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_ADD_SETCODE)
    effect::set_code(d, e1, EFFECT_ADD_SETCODE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(SET_FRIGHTFUR)
    effect::set_value(d, e1, SET_FRIGHTFUR as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // fusion substitute
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_FUSION_SUBSTITUTE)
    effect::set_code(d, e2, EFFECT_FUSION_SUBSTITUTE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(s.subval)
    effect::set_value_fn(d, e2, effect_card_fn(subval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// s.listed_series={SET_FRIGHTFUR}
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FRIGHTFUR], ..ScriptInfo::DEFAULT };
/// `s.subval(e,c)`
fn subval(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsSetCard(SET_FRIGHTFUR)
    card::is_set_card(d, c, &[SET_FRIGHTFUR as u16])
}
