//! Mind Pollutant
use super::prelude::*;

pub const ID: u32 = 69257165;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // control
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e1, CATEGORY_CONTROL);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn ctffilter(d: &mut Duel, c: CardId, lv: i64) -> bool {
    // c:IsFaceup() and c:IsControlerCanBeChanged() and c:GetLevel()==lv
    card::is_faceup(d, c) && card::is_controler_can_be_changed(d, c, false, None) && card::get_level(d, c) as i64 == lv
}
fn ctfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // c:IsMonster() and c:IsDiscardable()
    //   and Duel.IsExistingTarget(s.ctffilter,tp,0,LOCATION_MZONE,1,nil,c:GetLevel())
    if !(card::is_monster(d, c) && card::is_discardable(d, c, None, None)) {
        return false;
    }
    let lv = card::get_level(d, c) as i64;
    duel::is_existing_target(d, Some(&|d, cc| ctffilter(d, cc, lv)), tp, 0, LOCATION_MZONE, 1, NO_EX)
}
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(100)
    effect::set_label(d, a.e, &[100]);
    // return true
    true
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.ctffilter(chkc,e:GetLabel()) end
    if let Some(chkc) = a.chkc() {
        let lbl = effect::get_label(d, e);
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && ctffilter(d, chkc, lbl);
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()~=100 then return false end
        if effect::get_label(d, e) != 100 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, e, &[0]);
        // return Duel.IsExistingMatchingCard(s.ctfilter,tp,LOCATION_HAND,0,1,nil,tp)
        return duel::is_existing_matching_card(d, Some(&|d, c| ctfilter(d, c, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DISCARD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DISCARD as u64);
    // local sg=Duel.SelectMatchingCard(tp,s.ctfilter,tp,LOCATION_HAND,0,1,1,nil,tp)
    let sg = duel::select_matching_card(d, tp, Some(&|d, c| ctfilter(d, c, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // local lv=sg:GetFirst():GetLevel()
    let first = group::get_first(d, &sg).nil();
    let lv = card::get_level(d, first);
    // e:SetLabel(lv)
    effect::set_label(d, e, &[lv as i64]);
    // Duel.SendtoGrave(sg,REASON_COST|REASON_DISCARD)
    duel::send_to_grave(d, sg, REASON_COST | REASON_DISCARD, None, None);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
    // local g=Duel.SelectTarget(tp,s.ctffilter,tp,0,LOCATION_MZONE,1,1,nil,lv)
    let g = duel::select_target(d, tp, Some(&|d, c| ctffilter(d, c, lv as i64)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, g, 1, 0, 0);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    if let Some(tc) = duel::first_target(d) {
        // if tc and tc:IsRelateToEffect(e) and tc:IsFaceup() and tc:GetLevel()==e:GetLabel() then
        if card::is_relate_to_effect(d, tc, a.e)
            && card::is_faceup(d, tc)
            && card::get_level(d, tc) as i64 == effect::get_label(d, a.e)
        {
            // Duel.GetControl(tc,tp,PHASE_END,1)
            duel::get_control(d, tc, tp, PHASE_END as u16, 1, None, None);
        }
    }
}
