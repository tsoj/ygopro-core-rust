//! Order to Charge
use super::prelude::*;

pub const ID: u32 = 78986941;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.rfilter(c,e)
fn rfilter(d: &mut Duel, c: CardId) -> bool {
    // local tpe=c:GetType()
    let tpe = card::get_type(d, c);
    // return (tpe&TYPE_NORMAL)~=0 and (tpe&TYPE_TOKEN)==0
    //     and c:IsFaceup() and c:IsReleasable() and c:IsReleasableByEffect()
    (tpe & TYPE_NORMAL) != 0
        && (tpe & TYPE_TOKEN) == 0
        && card::is_faceup(d, c)
        && card::is_releasable(d, c, None, None)
        && card::is_releasable_by_effect(d, c, None, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.rfilter(chkc,e) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && rfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.rfilter,tp,LOCATION_MZONE,0,1,nil,e)
    //     and Duel.IsExistingMatchingCard(aux.TRUE,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&rfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, None, tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RELEASE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RELEASE as u64);
    // local rg=Duel.SelectTarget(tp,s.rfilter,tp,LOCATION_MZONE,0,1,1,nil,e)
    duel::select_target(d, tp, Some(&rfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // local g=Duel.GetMatchingGroup(aux.TRUE,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, None, tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) and not tc:IsImmuneToEffect(e) and Duel.Release(tc,REASON_EFFECT)>0 then
    if card::is_faceup(d, tc)
        && card::is_relate_to_effect(d, tc, a.e)
        && !card::is_immune_to_effect(d, tc, a.e)
        && duel::release(d, tc, REASON_EFFECT, None) > 0
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local dg=Duel.SelectMatchingCard(tp,aux.TRUE,tp,0,LOCATION_MZONE,1,1,nil)
        let dg = duel::select_matching_card(d, tp, None, tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        // if #dg>0 then
        if !dg.is_empty() {
            // Duel.Destroy(dg,REASON_EFFECT)
            duel::destroy(d, &dg, REASON_EFFECT, None, None);
        }
    }
}
