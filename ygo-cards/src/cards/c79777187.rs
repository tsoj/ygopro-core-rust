//! Bubble Barrier
use super::prelude::*;

pub const ID: u32 = 79777187;

// s.listed_series={SET_PERFORMAGE,SET_PERFORMAPAL}
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PERFORMAGE, SET_PERFORMAPAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // avoid battle damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
    effect::set_code(d, e2, EFFECT_AVOID_BATTLE_DAMAGE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(effect_card_fn(target)));
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // indes
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_COUNT)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_COUNT);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e3, LOCATION_MZONE, 0);
    // e3:SetTarget(s.target)
    effect::set_target(d, e3, Some(effect_card_fn(target)));
    // e3:SetValue(s.indct)
    effect::set_value_fn(d, e3, lua_fn(indct));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.target(e,c)`
fn target(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsSetCard({SET_PERFORMAGE,SET_PERFORMAPAL})
    card::is_set_card(d, c, &[SET_PERFORMAGE as u16, SET_PERFORMAPAL as u16])
}
/// `s.indct(e,re,r,rp)`
fn indct(_d: &mut Duel, a: Args) -> i64 {
    // if (r&REASON_BATTLE+REASON_EFFECT)~=0 then return 1 else return 0 end
    if (a.int(2) as u32) & (REASON_BATTLE + REASON_EFFECT) != 0 {
        1
    } else {
        0
    }
}
