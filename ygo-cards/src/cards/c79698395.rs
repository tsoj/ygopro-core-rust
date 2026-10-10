//! Realm of Danger!
use super::prelude::*;

pub const ID: u32 = 79698395;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DANGER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // cannot be target
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(effect_card_fn(target)));
    // e2:SetValue(aux.tgoval)
    effect::set_value_fn(d, e2, aux::tgoval());
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // direct attack
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetTarget(s.datg)
    effect::set_target(d, e3, Some(cost_target(datg)));
    // e3:SetOperation(s.daop)
    effect::set_operation(d, e3, Some(operation(daop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.target(e,c)`
fn target(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsSetCard(SET_DANGER) and c:IsStatus(STATUS_SPSUMMON_TURN)
    card::is_set_card(d, c, &[SET_DANGER as u16]) && card::is_status(d, c, STATUS_SPSUMMON_TURN)
}
/// `s.dafilter(c)`
fn dafilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_DANGER)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_DANGER as u16])
}
/// `s.datg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn datg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.dafilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && dafilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.dafilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| dafilter(d, x)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // local g=Duel.SelectTarget(tp,s.dafilter,tp,LOCATION_MZONE,0,1,1,nil)
    let _ = duel::select_target(d, a.tp, Some(&|d, x| dafilter(d, x)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.daop(e,tp,eg,ep,ev,re,r,rp)`
fn daop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    // if not (c:IsRelateToEffect(e) and tc:IsRelateToEffect(e) and tc:IsFaceup()) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let tc = duel::first_target(d).nil();
    if !(card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc)) {
        return;
    }
    // c:SetCardTarget(tc)
    card::set_card_target(d, c, tc);
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, c);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // e1:SetCondition(s.rcon)
    effect::set_condition(d, e1, Some(condition(rcon)));
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_IGNORE_BATTLE_TARGET)
    effect::set_code(d, e2, EFFECT_IGNORE_BATTLE_TARGET);
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // tc:RegisterEffect(e2)
    card::register_effect(d, tc, e2, false);
}
/// `s.rcon(e)`
fn rcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetOwner():IsHasCardTarget(e:GetHandler())
    let owner = effect::get_owner(d, a.e).nil();
    let h = effect::get_handler(d, a.e).nil();
    card::is_has_card_target(d, owner, h)
}
