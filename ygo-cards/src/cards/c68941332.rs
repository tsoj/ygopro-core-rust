//! Rikka Sheet
use super::prelude::*;

pub const ID: u32 = 68941332;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Prevent the target's effects from being activated and gain control if a Plant is tributed
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e1, CATEGORY_CONTROL);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // e1:SetLabel(1)
    effect::set_label(d, e1, &[1]);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and (c:IsType(TYPE_EFFECT) or c:GetOriginalType()&TYPE_EFFECT>0)
    card::is_faceup(d, c) && (card::is_type(d, c, TYPE_EFFECT) || (card::get_original_type(d, c) & TYPE_EFFECT) > 0)
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsRikkaReleasable(tp) and Duel.GetMZoneCount(tp,c,tp,LOCATION_REASON_CONTROL)>0
    //   and Duel.IsExistingTarget(aux.AND(s.filter,Card.IsControlerCanBeChanged),tp,0,LOCATION_MZONE,1,c,true)
    aux::is_rikka_releasable(d, c, tp)
        && duel::get_m_zone_count(d, tp, c.into(), Some(tp), Some(LOCATION_REASON_CONTROL), None).0 > 0
        && duel::is_existing_target(d, Some(&|d, x| filter(d, x) && card::is_controler_can_be_changed(d, x, true, None)), tp, 0, LOCATION_MZONE, 1, c.into())
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // local a=Duel.IsExistingTarget(s.filter,tp,0,LOCATION_MZONE,1,nil)
    let ex_a = duel::is_existing_target(d, Some(&filter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    // local b=Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,nil,nil,tp)
    let b = duel::check_release_group_cost(d, a.tp, Some(&|d, c| cfilter(d, c, a.tp)), 1, 1, false, None, NO_EX);
    // if chk==0 then return a or b end
    if a.chk == Some(0) {
        return ex_a || b;
    }
    // if b and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if b && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
        // local g=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,1,false,nil,nil,tp)
        let g = duel::select_release_group_cost(d, a.tp, Some(&|d, c| cfilter(d, c, a.tp)), 1, 1, false, None, NO_EX);
        // Duel.Release(g,REASON_COST)
        duel::release(d, g, REASON_COST, None);
        // e:SetLabel(1)
        effect::set_label(d, a.e, &[1]);
    } else {
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
    }
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // if e:GetLabel()==1 then
    if effect::get_label(d, a.e) == 1 {
        // local g=Duel.SelectTarget(tp,aux.AND(s.filter,Card.IsControlerCanBeChanged),tp,0,LOCATION_MZONE,1,1,nil,true)
        let g = duel::select_target(
            d,
            a.tp,
            Some(&|d, x| filter(d, x) && card::is_controler_can_be_changed(d, x, true, None)),
            a.tp,
            0,
            LOCATION_MZONE,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        // Duel.SetOperationInfo(0,CATEGORY_CONTROL,g,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(g), 1, 0, 0);
    } else {
        // local g=Duel.SelectTarget(tp,s.filter,tp,0,LOCATION_MZONE,1,1,nil)
        let _g = duel::select_target(d, a.tp, Some(&filter), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) and not tc:IsImmuneToEffect(e) then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) && !card::is_immune_to_effect(d, tc, a.e) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // Target cannot activate its effects
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,2))
        effect::set_description(d, e1, aux::stringid(ID, 2));
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CANNOT_TRIGGER)
        effect::set_code(d, e1, EFFECT_CANNOT_TRIGGER);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // Gain control of the target
        // if e:GetLabel()==1 and tc:IsControlerCanBeChanged() and tc:IsControler(1-tp) then
        if effect::get_label(d, a.e) == 1 && card::is_controler_can_be_changed(d, tc, false, None) && card::is_controler(d, tc, 1 - a.tp) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.GetControl(tc,tp,PHASE_END,1)
            duel::get_control(d, CardSet::from_card(tc), a.tp, PHASE_END as u16, 1, None, None);
            // if tc:IsControler(1-tp) then return end
            if card::is_controler(d, tc, 1 - a.tp) {
                return;
            }
            // Target becomes a Plant monster
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetCode(EFFECT_CHANGE_RACE)
            effect::set_code(d, e2, EFFECT_CHANGE_RACE);
            // e2:SetValue(RACE_PLANT)
            effect::set_value(d, e2, RACE_PLANT as i64);
            // e2:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
        }
    }
}
