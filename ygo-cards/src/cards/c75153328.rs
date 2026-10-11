//! Packet Swap
use super::prelude::*;

pub const ID: u32 = 75153328;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e1, CATEGORY_CONTROL);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter1(c)`
fn filter1(d: &mut Duel, c: CardId) -> bool {
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return c:IsAbleToChangeControler() and c:IsFaceup() and not c:IsLinkMonster()
    //   and Duel.GetMZoneCount(tp,c,tp,LOCATION_REASON_CONTROL)>0
    card::is_able_to_change_controler(d, c)
        && card::is_faceup(d, c)
        && !card::is_link_monster(d, c)
        && duel::get_m_zone_count(d, tp, (&CardSet::from_card(c)).into(), Some(tp), Some(LOCATION_REASON_CONTROL), None).0 > 0
}
/// `s.filter2(c)`
fn filter2(d: &mut Duel, c: CardId) -> bool {
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return c:IsLinkMonster() and c:IsAbleToChangeControler() and Duel.GetMZoneCount(tp,c,tp,LOCATION_REASON_CONTROL)>0
    card::is_link_monster(d, c)
        && card::is_able_to_change_controler(d, c)
        && duel::get_m_zone_count(d, tp, (&CardSet::from_card(c)).into(), Some(tp), Some(LOCATION_REASON_CONTROL), None).0 > 0
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter2,tp,0,LOCATION_MZONE,1,nil)
    //   and Duel.IsExistingTarget(s.filter1,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter2), a.tp, 0, LOCATION_MZONE, 1, NO_EX)
            && duel::is_existing_target(d, Some(&filter1), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONTROL as u64);
    // local g1=Duel.SelectTarget(tp,s.filter1,tp,LOCATION_MZONE,0,1,1,nil)
    let mut g1 = duel::select_target(d, a.tp, Some(&filter1), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONTROL as u64);
    // local g2=Duel.SelectTarget(tp,s.filter2,tp,0,LOCATION_MZONE,1,1,nil)
    let g2 = duel::select_target(d, a.tp, Some(&filter2), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,g1,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(g1), 2, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]);
    let g = info[0].as_group().cloned().nil();
    // local a=g:GetFirst()
    let first = group::get_first(d, &g).nil();
    // local b=g:GetNext()
    let second = group::iter(d, &g).get(1).copied().nil();
    // if a:IsRelateToEffect(e) and b:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, first, a.e) && card::is_relate_to_effect(d, second, a.e) {
        // Duel.SwapControl(a,b)
        duel::swap_control(d, first, second, 0, 0);
    }
}
