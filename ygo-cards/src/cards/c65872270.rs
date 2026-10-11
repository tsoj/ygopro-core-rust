//! Igknight Burst
use super::prelude::*;

pub const ID: u32 = 65872270;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_IGKNIGHT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // to hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e3, CATEGORY_TOHAND);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // if s.descon(e,tp,eg,ep,ev,re,r,rp) and s.destg(e,tp,eg,ep,ev,re,r,rp,0) and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    let mut a0 = a.clone();
    a0.chk = Some(0);
    if descon(d, a) && destg(d, &a0) && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0)) {
        // e:SetCategory(CATEGORY_DESTROY+CATEGORY_TOHAND)
        effect::set_category(d, a.e, CATEGORY_DESTROY + CATEGORY_TOHAND);
        // e:SetOperation(s.desop)
        effect::set_operation(d, a.e, Some(operation(desop)));
        // s.destg(e,tp,eg,ep,ev,re,r,rp,1)
        let mut a1 = a.clone();
        a1.chk = Some(1);
        destg(d, &a1);
    } else {
        // e:SetCategory(0)
        effect::set_category(d, a.e, 0);
        // e:SetOperation(nil)
        effect::set_operation(d, a.e, None);
    }
    true
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp) and (Duel.IsMainPhase())
    duel::is_turn_player(d, a.tp) && duel::is_main_phase(d, None)
}
/// `s.desfilter(c)`
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_IGKNIGHT)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_IGKNIGHT as u16])
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local h=e:GetHandler()
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.desfilter,tp,LOCATION_ONFIELD,0,1,e:GetHandler())
    //     and Duel.IsExistingMatchingCard(Card.IsAbleToHand,tp,0,LOCATION_ONFIELD,1,nil)
    //     and e:GetHandler():GetFlagEffect(id)==0 end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| desfilter(d, c)), a.tp, LOCATION_ONFIELD, 0, 1, h.into())
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX)
            && card::get_flag_effect(d, h, ID) == 0;
    }
    // e:GetHandler():RegisterFlagEffect(id,RESET_PHASE|PHASE_END,0,1)
    card::register_flag_effect(d, h, ID, RESET_PHASE | PHASE_END, 0, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,nil,1,tp,LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, None, 1, a.tp, LOCATION_ONFIELD as i32);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local g=Duel.GetMatchingGroup(Card.IsAbleToHand,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // local ct=#g
    let mut ct = g.len();
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // if ct>3 then ct=3 end
    if ct > 3 {
        ct = 3;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local dg=Duel.SelectMatchingCard(tp,s.desfilter,tp,LOCATION_ONFIELD,0,1,ct,e:GetHandler())
    let dg = duel::select_matching_card(d, a.tp, Some(&|d, c| desfilter(d, c)), a.tp, LOCATION_ONFIELD, 0, 1, ct as u16, false, h.into()).nil();
    // local ct2=Duel.Destroy(dg,REASON_EFFECT)
    let ct2 = duel::destroy(d, dg, REASON_EFFECT, None, None);
    // if ct2>0 then
    if ct2 > 0 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
        // local rg=g:Select(tp,ct2,ct2,nil)
        let rg = group::select(d, &g, a.tp, ct2 as u16, ct2 as u16, false, NO_EX).nil();
        // Duel.HintSelection(rg)
        duel::hint_selection(d, rg.clone(), true);
        // Duel.SendtoHand(rg,nil,REASON_EFFECT)
        duel::send_to_hand(d, rg, None, REASON_EFFECT, None);
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_IGKNIGHT) and c:IsAbleToHand()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_IGKNIGHT as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_EXTRA,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| thfilter(d, c)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_EXTRA,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| thfilter(d, c)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
