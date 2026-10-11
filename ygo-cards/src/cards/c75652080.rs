//! Double Cyclone
use super::prelude::*;

pub const ID: u32 = 75652080;

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
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap()
    card::is_spell_trap(d, c)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local c=e:GetHandler() (used by the selections below)
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,0,LOCATION_ONFIELD,1,nil)
    //     and Duel.IsExistingTarget(s.filter,tp,LOCATION_ONFIELD,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, 0, LOCATION_ONFIELD, 1, NO_EX)
            && duel::is_existing_target(d, Some(&filter), tp, LOCATION_ONFIELD, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g1=Duel.SelectTarget(tp,s.filter,tp,LOCATION_ONFIELD,0,1,1,e:GetHandler())
    let mut g1 = duel::select_target(d, tp, Some(&filter), tp, LOCATION_ONFIELD, 0, 1, 1, false, c.into()).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g2=Duel.SelectTarget(tp,s.filter,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g2 = duel::select_target(d, tp, Some(&filter), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g1,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g1), 2, 0, 0);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    // local sg=g:Filter(Card.IsRelateToEffect,nil,e)
    let e = a.e;
    let sg = group::filter(d, &g, &|d, c| card::is_relate_to_effect(d, c, e), NO_EX);
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.Destroy(sg,REASON_EFFECT)
        duel::destroy(d, sg, REASON_EFFECT, None, None);
    }
}
