//! Eternal Chaos
use super::prelude::*;

pub const ID: u32 = 25750986;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Send 1 LIGHT and 1 DARK monster from deck to GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.tgtg)
    effect::set_target(d, e1, Some(cost_target(tgtg)));
    // e1:SetOperation(s.tgop)
    effect::set_operation(d, e1, Some(operation(tgop)));
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_END_PHASE));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.rescon(atk)`: `function(sg,e,tp,mg)`
fn rescon_atk(atk: i64) -> impl Fn(&mut Duel, &CardSet, Option<EffectId>, u8, &CardSet, Option<CardId>) -> (bool, bool) {
    move |d, sg, _e, _tp, _mg, _c| {
        // return sg:IsExists(Card.IsAttribute,1,nil,ATTRIBUTE_DARK) and sg:IsExists(Card.IsAttribute,1,nil,ATTRIBUTE_LIGHT)
        //   and sg:GetSum(Card.GetAttack)<=atk
        let res = group::is_exists(d, sg, &|d, c| card::is_attribute(d, c, ATTRIBUTE_DARK), 1, NO_EX)
            && group::is_exists(d, sg, &|d, c| card::is_attribute(d, c, ATTRIBUTE_LIGHT), 1, NO_EX)
            && group::get_sum(d, sg, &|d, c| card::get_attack(d, c) as i64) <= atk;
        (res, false)
    }
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_DARK+ATTRIBUTE_LIGHT) and c:IsAbleToGrave()
    card::is_attribute(d, c, ATTRIBUTE_DARK + ATTRIBUTE_LIGHT) && card::is_able_to_grave(d, c, None)
}
/// `s.cfilter(c,e,tp,g)`
fn cfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, g: &CardSet) -> bool {
    // return c:IsFaceup() and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon(c:GetAttack()),0)
    if !card::is_faceup(d, c) {
        return false;
    }
    let rs = rescon_atk(card::get_attack(d, c) as i64);
    matches!(aux::select_unselect_group(d, g, Some(e), tp, Some(2), Some(2), Some(&rs), 0, tp, None, None, None, false), Err(true))
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&tgfilter), tp, LOCATION_DECK, 0, NO_EX);
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE) and s.cfilter(chkc,e,tp,g) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_MZONE) && cfilter(d, chkc, a.e, tp, &g);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.cfilter,tp,0,LOCATION_MZONE,1,nil,e,tp,g) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| cfilter(d, c, a.e, tp, &g)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.cfilter,tp,0,LOCATION_MZONE,1,1,nil,e,tp,g)
    let _ = duel::select_target(d, tp, Some(&|d, c| cfilter(d, c, a.e, tp, &g)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 2, tp, LOCATION_DECK as i32);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // activate check
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // e1:SetOperation(s.aclimit)
    effect::set_operation(d, e1, Some(operation(aclimit)));
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // activate limit
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // e2:SetCondition(s.econ)
    effect::set_condition(d, e2, Some(condition(econ)));
    // e2:SetValue(s.elimit)
    effect::set_value_fn(d, e2, lua_fn(elimit));
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if not tc or tc:IsFacedown() or not tc:IsRelateToEffect(e) then return end
    let Some(tc) = tc else { return };
    if card::is_facedown(d, tc) || !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&tgfilter), tp, LOCATION_DECK, 0, NO_EX);
    // local tg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon(tc:GetAttack()),1,tp,HINTMSG_TOGRAVE,nil,nil)
    let rs = rescon_atk(card::get_attack(d, tc) as i64);
    let tg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(&rs), 1, tp, Some(HINTMSG_TOGRAVE as u64), None, None, false).unwrap_or_default();
    // if #tg>0 then
    if group::get_count(&tg) > 0 {
        // Duel.SendtoGrave(tg,REASON_EFFECT)
        duel::send_to_grave(d, tg, REASON_EFFECT, None, None);
    }
}
/// `s.aclimit(e,tp,eg,ep,ev,re,r,rp)`
fn aclimit(d: &mut Duel, a: &EffArgs) {
    // if ep~=tp or not (re:GetActivateLocation()==LOCATION_GRAVE and re:IsMonsterEffect()) then return end
    if a.ep != a.tp {
        return;
    }
    let re = a.re.nil();
    if !(effect::get_activate_location(d, re) == LOCATION_GRAVE && effect::is_monster_effect(d, re)) {
        return;
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
}
/// `s.econ(e)`
fn econ(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFlagEffect(e:GetHandlerPlayer(),id)~=0
    let p = effect::get_handler_player(d, a.e);
    duel::get_flag_effect(d, p, ID) != 0
}
/// `s.elimit(e,te,tp)`
fn elimit(d: &mut Duel, a: Args) -> bool {
    // return te:GetActivateLocation()==LOCATION_GRAVE and te:IsMonsterEffect()
    let te = a.e(1);
    effect::get_activate_location(d, te) == LOCATION_GRAVE && effect::is_monster_effect(d, te)
}
