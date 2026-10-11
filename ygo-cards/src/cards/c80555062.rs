//! Bujingi Warg
use super::prelude::*;

pub const ID: u32 = 80555062;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BUJIN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(effect_card_fn(target)));
    // e1:SetCondition(s.con)
    effect::set_condition(d, e1, Some(lua_fn(con)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.con(e)
fn con(d: &mut Duel, a: Args) -> bool {
    // return e:GetHandler():IsDefensePos()
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_defense_pos(d, h)
}
// function s.target(e,c)
fn target(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c~=e:GetHandler() and c:IsSetCard(SET_BUJIN)
    let h = effect::get_handler(d, e).nil();
    c != h && card::is_set_card(d, c, &[SET_BUJIN as u16])
}
