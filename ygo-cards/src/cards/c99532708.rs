//! Alien Mars
use super::prelude::*;

pub const ID: u32 = 99532708;

pub const INFO: ScriptInfo = ScriptInfo {
    counter_list: &[COUNTER_A],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // disable
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetCode(EFFECT_DISABLE)
    effect::set_code(d, e1, EFFECT_DISABLE);
    // e1:SetTarget(s.distg)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| distg(d, c))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.distg(e,c)`
fn distg(d: &mut Duel, c: CardId) -> bool {
    // return c:GetCounter(COUNTER_A)>0 and c:GetCode()~=id
    card::get_counter(d, c, COUNTER_A as u16) > 0 && card::get_code(d, c)[0] != ID
}
