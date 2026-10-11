//! Indomitable Gladiator Beast
use super::prelude::*;

pub const ID: u32 = 55136228;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GLADIATOR_BEAST as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Make 1 "Gladiator Beast" monster you control gain 500 ATK until the end of this turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP as u32, None);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add this card to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCost(s.rthcost)
    effect::set_cost(d, e2, Some(cost_target(rthcost)));
    // e2:SetTarget(s.rthtg)
    effect::set_target(d, e2, Some(cost_target(rthtg)));
    // e2:SetOperation(s.rthop)
    effect::set_operation(d, e2, Some(operation(rthop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and chkc:IsSetCard(SET_GLADIATOR_BEAST) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_controler(d, chkc, a.tp)
            && card::is_set_card(d, chkc, &[SET_GLADIATOR_BEAST as u16])
            && card::is_faceup(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.FaceupFilter(Card.IsSetCard,SET_GLADIATOR_BEAST),tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(
            d,
            Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_GLADIATOR_BEAST as u16]))),
            a.tp,
            LOCATION_MZONE,
            0,
            1,
            NO_EX,
        );
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATKDEF as u64);
    // Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsSetCard,SET_GLADIATOR_BEAST),tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(
        d,
        a.tp,
        Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_GLADIATOR_BEAST as u16]))),
        a.tp,
        LOCATION_MZONE,
        0,
        1,
        1,
        false,
        NO_EX,
    );
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and tc:IsControler(tp) and tc:IsSetCard(SET_GLADIATOR_BEAST) then
    if card::is_relate_to_effect(d, tc, a.e)
        && card::is_faceup(d, tc)
        && card::is_controler(d, tc, a.tp)
        && card::is_set_card(d, tc, &[SET_GLADIATOR_BEAST as u16])
    {
        // It gains 500 ATK until the end of this turn
        // tc:UpdateAttack(500,RESETS_STANDARD_PHASE_END,e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        card::update_attack(d, tc, 500, Some(RESETS_STANDARD_PHASE_END), Some(h), None);
    }
}
/// `s.rthcostfilter(c)`
fn rthcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_GLADIATOR_BEAST) and c:IsAbleToDeckAsCost()
    card::is_set_card(d, c, &[SET_GLADIATOR_BEAST as u16]) && card::is_able_to_deck_as_cost(d, c)
}
/// `s.rthcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rthcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.rthcostfilter,tp,LOCATION_GRAVE,0,2,c) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&rthcostfilter), a.tp, LOCATION_GRAVE, 0, 2, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(tp,s.rthcostfilter,tp,LOCATION_GRAVE,0,2,2,c)
    let g = duel::select_matching_card(d, a.tp, Some(&rthcostfilter), a.tp, LOCATION_GRAVE, 0, 2, 2, false, c.into()).nil();
    // Duel.HintSelection(g)
    duel::hint_selection(d, &g, true);
    // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_COST)
    duel::send_to_deck(d, &g, None, SEQ_DECKSHUFFLE as i32, REASON_COST, None);
    true
}
/// `s.rthtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rthtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHand() end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.rthop(e,tp,eg,ep,ev,re,r,rp)`
fn rthop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SendtoHand(c,nil,REASON_EFFECT)
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
    }
}
