//! Mind Castlin
use super::prelude::*;

pub const ID: u32 = 12172567;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: 1 Tuner + 1+ non-Tuner monsters
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Switch control of 1 opponent's face-up monster and this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e1, CATEGORY_CONTROL);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.cttg1)
    effect::set_target(d, e1, Some(cost_target(cttg1)));
    // e1:SetOperation(s.ctop1)
    effect::set_operation(d, e1, Some(operation(ctop1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Switch control of 1 face-up monster on each field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e2, CATEGORY_CONTROL);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.ctcon)
    effect::set_condition(d, e2, Some(condition(ctcon)));
    // e2:SetTarget(s.cttg2)
    effect::set_target(d, e2, Some(cost_target(cttg2)));
    // e2:SetOperation(s.ctop2)
    effect::set_operation(d, e2, Some(operation(ctop2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.ctfilter(c,tp,e)`: `e` is nil in the calls that do not pass an effect.
fn ctfilter(d: &mut Duel, c: CardId, tp: u8, e: Option<EffectId>) -> bool {
    // return c:IsFaceup() and c:IsAbleToChangeControler() and Duel.GetMZoneCount(c:GetControler(),c,tp,LOCATION_REASON_CONTROL)>0
    //     and (not e or c:IsCanBeEffectTarget(e))
    let p = card::get_controler(d, c);
    card::is_faceup(d, c)
        && card::is_able_to_change_controler(d, c)
        && duel::get_m_zone_count(d, p, c.into(), Some(tp), Some(LOCATION_REASON_CONTROL), None).0 > 0
        && (e.is_none() || card::is_can_be_effect_target(d, c, e))
}
/// `s.cttg1(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn cttg1(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE) and s.ctfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && ctfilter(d, chkc, a.tp, None);
    }
    // if chk==0 then return c:IsAbleToChangeControler() and Duel.GetMZoneCount(tp,c,tp,LOCATION_REASON_CONTROL)>0
    //     and Duel.IsExistingTarget(s.ctfilter,tp,0,LOCATION_MZONE,1,nil,tp) end
    if a.chk == Some(0) {
        return card::is_able_to_change_controler(d, c)
            && duel::get_m_zone_count(d, a.tp, c.into(), Some(a.tp), Some(LOCATION_REASON_CONTROL), None).0 > 0
            && duel::is_existing_target(d, Some(&|d, x| ctfilter(d, x, a.tp, None)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONTROL as u64);
    // local g=Duel.SelectTarget(tp,s.ctfilter,tp,0,LOCATION_MZONE,1,1,nil,tp)
    let mut g = duel::select_target(d, a.tp, Some(&|d, x| ctfilter(d, x, a.tp, None)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // g:AddCard(c)
    group::add_card(&mut g, &CardSet::from_card(c));
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,g,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(g), 2, a.tp, 0);
    true
}
/// `s.ctop1(e,tp,eg,ep,ev,re,r,rp)`
fn ctop1(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) && card::is_relate_to_effect(d, tc.nil(), a.e) {
        // Duel.SwapControl(c,tc)
        duel::swap_control(d, c, tc.nil(), 0, 0);
    }
}
/// `s.ctcon(e,tp,eg,ep,ev,re,r,rp)`
fn ctcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsSynchroSummoned()
    card::is_previous_location(d, c, LOCATION_MZONE) && card::is_synchro_summoned(d, c)
}
/// `s.controler_value(c)`: `Card.GetControler` as a number for `aux.dpcheck`
fn controler_value(d: &mut Duel, c: CardId) -> i64 {
    card::get_controler(d, c) as i64
}
/// `s.cttg2(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn cttg2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.ctfilter,tp,LOCATION_MZONE,0,1,nil,tp)
    //     and Duel.IsExistingTarget(s.ctfilter,tp,0,LOCATION_MZONE,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| ctfilter(d, x, a.tp, None)), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&|d, x| ctfilter(d, x, a.tp, None)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(s.ctfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,tp,e)
    let g = duel::get_matching_group(d, Some(&|d, x| ctfilter(d, x, a.tp, Some(a.e))), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // local tg=aux.SelectUnselectGroup(g,e,tp,2,2,aux.dpcheck(Card.GetControler),1,tp,HINTMSG_CONTROL)
    let dp = aux::dpcheck(controler_value);
    let tg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&dp), 1, a.tp, Some(HINTMSG_CONTROL as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,tg,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(tg), 2, a.tp, 0);
    true
}
/// `s.ctop2(e,tp,eg,ep,ev,re,r,rp)`
fn ctop2(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg==2 then
    if tg.len() == 2 {
        // Duel.SwapControl(tg:GetFirst(),tg:GetNext())
        let ids = group::iter(d, &tg);
        duel::swap_control(d, ids[0], ids[1], 0, 0);
    }
}
