//! Temple of the Six
use super::prelude::*;

pub const ID: u32 = 53819808;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SIX_SAMURAI], counter_place_list: &[COUNTER_BUSHIDO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(COUNTER_BUSHIDO)
    card::enable_counter_permit(d, c, COUNTER_BUSHIDO as u16, None, None);
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // add counter
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetOperation(s.ctop)
    effect::set_operation(d, e2, Some(operation(ctop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // atk down
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e4, EFFECT_UPDATE_ATTACK);
    // e4:SetRange(LOCATION_FZONE)
    effect::set_range(d, e4, LOCATION_FZONE);
    // e4:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e4, 0, LOCATION_MZONE);
    // e4:SetValue(s.val)
    effect::set_value_fn(d, e4, lua_fn(val));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.ctfilter(c)`
fn ctfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_SIX_SAMURAI as u16])
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // if eg:IsExists(s.ctfilter,1,nil) then
    let eg = a.eg.clone().nil();
    if group::is_exists(d, &eg, &ctfilter, 1, NO_EX) {
        // e:GetHandler():AddCounter(COUNTER_BUSHIDO,1)
        let h = effect::get_handler(d, a.e).nil();
        card::add_counter(d, h, COUNTER_BUSHIDO as u16, 1, false);
    }
}
/// `s.val(e)`
fn val(d: &mut Duel, a: Args) -> i64 {
    // return e:GetHandler():GetCounter(COUNTER_BUSHIDO)*-100
    let h = effect::get_handler(d, a.e(0)).nil();
    card::get_counter(d, h, COUNTER_BUSHIDO as u16) as i64 * -100
}
