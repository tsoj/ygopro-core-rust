//! Attention!
use super::prelude::*;

pub const ID: u32 = 85352446;

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
    // e1:SetHintTiming(TIMING_ATTACK,TIMINGS_CHECK_MONSTER_E|TIMING_ATTACK)
    effect::set_hint_timing(d, e1, TIMING_ATTACK, Some(TIMINGS_CHECK_MONSTER_E | TIMING_ATTACK));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // local lv=c:GetLevel()
    let lv = card::get_level(d, c);
    // return lv>0 and lv<=5 and c:IsFaceup() and Duel.IsExistingMatchingCard(s.filter2,0,LOCATION_MZONE,LOCATION_MZONE,1,c,lv)
    lv > 0
        && lv <= 5
        && card::is_faceup(d, c)
        && duel::is_existing_matching_card(d, Some(&|d, cc| filter2(d, cc, lv)), 0, LOCATION_MZONE, LOCATION_MZONE, 1, c.into())
}
/// `s.filter2(c,lv)`
fn filter2(d: &mut Duel, c: CardId, lv: u32) -> bool {
    // local clv=c:GetLevel()
    let clv = card::get_level(d, c);
    // return c:IsFaceup() and clv>0 and lv~=clv
    card::is_faceup(d, c) && clv > 0 && lv != clv
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
    // local g=Duel.SelectTarget(tp,s.filter,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // local dg=Duel.GetMatchingGroup(s.filter2,0,LOCATION_MZONE,LOCATION_MZONE,nil,g:GetFirst():GetLevel())
    let tc = group::get_first(d, &g).nil();
    let lv = card::get_level(d, tc);
    let dg = duel::get_matching_group(d, Some(&|d, cc| filter2(d, cc, lv)), 0, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,dg,#dg,0,0)
    let n = dg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), n, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
            // local dg=Duel.GetMatchingGroup(s.filter2,0,LOCATION_MZONE,LOCATION_MZONE,tc,tc:GetLevel())
            let lv = card::get_level(d, tc);
            let dg = duel::get_matching_group(d, Some(&|d, cc| filter2(d, cc, lv)), 0, LOCATION_MZONE, LOCATION_MZONE, tc.into());
            // Duel.Destroy(dg,REASON_EFFECT)
            duel::destroy(d, dg, REASON_EFFECT, None, None);
        }
    }
}
