//! Worm Gulse
use super::prelude::*;

pub const ID: u32 = 85754829;

pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[0xf], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(0xf)
    card::enable_counter_permit(d, c, 0xf, None, None);
    // add counter
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_CHANGE_POS)
    effect::set_code(d, e1, EVENT_CHANGE_POS);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.accon)
    effect::set_condition(d, e1, Some(condition(accon)));
    // e1:SetOperation(s.acop)
    effect::set_operation(d, e1, Some(operation(acop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // atkup
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(s.atkval)
    effect::set_value_fn(d, e2, effect_card_value(atkval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetCounter(0xf)*300
    card::get_counter(d, c, 0xf) as i64 * 300
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:GetPreviousPosition()&POS_FACEDOWN)~=0 and (c:GetPosition()&POS_FACEUP)~=0
    (card::get_previous_position(d, c) & POS_FACEDOWN) != 0 && (card::get_position(d, c) & POS_FACEUP) != 0
}
/// `s.accon(e,tp,eg,ep,ev,re,r,rp)`
fn accon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,e:GetHandler())
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, a.e).nil();
    group::is_exists(d, &eg, &cfilter, 1, h.into())
}
/// `s.acop(e,tp,...)`
fn acop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():AddCounter(0xf,1)
    let h = effect::get_handler(d, a.e).nil();
    card::add_counter(d, h, 0xf, 1, false);
}
