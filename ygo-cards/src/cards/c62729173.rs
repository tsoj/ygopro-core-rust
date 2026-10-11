//! Steelswarm Genome
use super::prelude::*;

pub const ID: u32 = 62729173;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_STEELSWARM], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // double tribute
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_DOUBLE_TRIBUTE)
    effect::set_code(d, e1, EFFECT_DOUBLE_TRIBUTE);
    // e1:SetValue(s.condition)
    effect::set_value_fn(d, e1, effect_card_fn(cond));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,c)`
fn cond(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsSetCard(SET_STEELSWARM)
    card::is_set_card(d, c, &[SET_STEELSWARM as u16])
}
