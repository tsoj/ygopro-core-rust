//! Machina Sniper
//! (マシンナーズ・スナイパー)
use super::prelude::*;

pub const ID: u32 = 23782705;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MACHINA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atk limit
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_BE_BATTLE_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_BATTLE_TARGET);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetTarget(s.tg)
    effect::set_target(d, e1, Some(effect_card_fn(tg)));
    // e1:SetValue(aux.imval1)
    effect::set_value_fn(d, e1, aux::imval1());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tg(e,c)`
fn tg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsSetCard(SET_MACHINA) and c:GetCode()~=id
    card::is_set_card(d, c, &[SET_MACHINA as u16]) && card::get_code(d, c).first().copied() != Some(ID)
}
