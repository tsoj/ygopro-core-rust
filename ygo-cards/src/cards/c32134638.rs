//! Dinomist Ankylos
use super::prelude::*;

pub const ID: u32 = 32134638;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DINOMIST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pendulum summon
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // negate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e1, EVENT_CHAIN_SOLVING);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCondition(s.negcon)
    effect::set_condition(d, e1, Some(condition(negcon)));
    // e1:SetOperation(s.negop)
    effect::set_operation(d, e1, Some(operation(negop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // redirect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_BATTLE_DESTROY_REDIRECT)
    effect::set_code(d, e2, EFFECT_BATTLE_DESTROY_REDIRECT);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetValue(LOCATION_REMOVED)
    effect::set_value(d, e2, LOCATION_REMOVED as i64);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_DINOMIST))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_DINOMIST as u16]))));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.tfilter(c,tp)`
fn tfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_DINOMIST) and c:IsControler(tp) and c:IsLocation(LOCATION_ONFIELD)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_DINOMIST as u16]) && card::is_controler(d, c, tp) && card::is_location(d, c, LOCATION_ONFIELD)
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let ev = a.ev as u8;
    // local g=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, ev, &[CHAININFO_TARGET_CARDS as u32]).first().and_then(|v| v.as_group().cloned());
    // return e:GetHandler():GetFlagEffect(id)==0 and re:IsHasProperty(EFFECT_FLAG_CARD_TARGET)
    //     and g and g:IsExists(s.tfilter,1,e:GetHandler(),tp) and Duel.IsChainDisablable(ev)
    let h = effect::get_handler(d, e).nil();
    if card::get_flag_effect(d, h, ID) != 0 {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    let Some(g) = g else { return false };
    group::is_exists(d, &g, &|d, c| tfilter(d, c, tp), 1, h.into()) && duel::is_chain_disablable(d, ev)
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.SelectEffectYesNo(tp,e:GetHandler()) then
    let h = effect::get_handler(d, e).nil();
    if duel::select_effect_yes_no(d, tp, h, None) {
        // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1)
        card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
        // if Duel.NegateEffect(ev) then
        if duel::negate_effect(d, a.ev as u8) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Destroy(e:GetHandler(),REASON_EFFECT)
            duel::destroy(d, h, REASON_EFFECT, None, None);
        }
    }
}
