//! Lose 1 Turn
use super::prelude::*;

pub const ID: u32 = 24348804;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_SPSUMMON)
    effect::set_hint_timing(d, e1, TIMING_SPSUMMON, None);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target1)
    effect::set_target(d, e1, Some(cost_target(target1)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // pos
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_POSITION)
    effect::set_category(d, e2, CATEGORY_POSITION);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTarget(s.target2)
    effect::set_target(d, e2, Some(cost_target(target2)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // e2:SetLabel(1)
    effect::set_label(d, e2, &[1]);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // disable
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_DISABLE)
    effect::set_code(d, e3, EFFECT_DISABLE);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    // e3:SetTarget(s.distg)
    effect::set_target(d, e3, Some(effect_card_fn(distg)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpecialSummoned()
    card::is_special_summoned(d, c)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil)
    !duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.filter1(c)`
fn filter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsPosition(POS_FACEUP_ATTACK) and c:IsType(TYPE_EFFECT)
    card::is_position(d, c, POS_FACEUP_ATTACK as u32) && card::is_type(d, c, TYPE_EFFECT)
}
/// `s.target1(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target1(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // e:SetLabel(0)
    effect::set_label(d, e, &[0]);
    // local res,teg,tep,tev,tre,tr,trp=Duel.CheckEvent(EVENT_SPSUMMON_SUCCESS,true)
    // if res then
    if let Some(evt) = duel::check_event(d, EVENT_SPSUMMON_SUCCESS) {
        // local g=teg:Filter(s.filter1,nil)
        let teg = evt.event_cards.clone().nil();
        let g = group::filter(d, &teg, &filter1, NO_EX);
        // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
            // Duel.SetTargetCard(g)
            duel::set_target_card(d, g.clone());
            // Duel.SetOperationInfo(0,CATEGORY_POSITION,g,#g,0,0)
            duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(g.clone()), g.len() as u8, 0, 0);
            // e:SetLabel(1)
            effect::set_label(d, e, &[1]);
            // e:GetHandler():RegisterFlagEffect(0,RESET_CHAIN,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,2))
            let h = effect::get_handler(d, e).nil();
            card::register_flag_effect(d, h, 0, RESET_CHAIN, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 2));
        }
    }
    true
}
/// `s.target2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target2(d: &mut Duel, a: &EffArgs) -> bool {
    // local eg = the event group
    let eg = a.eg.clone().nil();
    // if chk==0 then return eg:IsExists(s.filter1,1,nil) end
    if a.chk == Some(0) {
        return group::is_exists(d, &eg, &filter1, 1, NO_EX);
    }
    // local g=eg:Filter(s.filter1,nil)
    let g = group::filter(d, &eg, &filter1, NO_EX);
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,g,#g,0,0)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(g.clone()), g.len() as u8, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // if e:GetLabel()==0 or not e:GetHandler():IsRelateToEffect(e) then return end
    if effect::get_label(d, e) == 0 {
        return;
    }
    let h = effect::get_handler(d, e).nil();
    if !card::is_relate_to_effect(d, h, e) {
        return;
    }
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, e).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.ChangePosition(g,POS_FACEUP_DEFENSE,POS_FACEDOWN_DEFENSE,POS_FACEUP_DEFENSE,POS_FACEDOWN_DEFENSE)
        duel::change_position(d, g, POS_FACEUP_DEFENSE as u8, Some(POS_FACEDOWN_DEFENSE as u8), Some(POS_FACEUP_DEFENSE as u8), Some(POS_FACEDOWN_DEFENSE as u8), false);
    }
}
/// `s.distg(e,c)`
fn distg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsStatus(STATUS_SPSUMMON_TURN) and c:IsSpecialSummoned()
    card::is_status(d, c, STATUS_SPSUMMON_TURN) && card::is_special_summoned(d, c)
}
