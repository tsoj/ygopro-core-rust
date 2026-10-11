//! Miracle Dig
use super::prelude::*;

pub const ID: u32 = 6343408;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsMonster()
    card::is_faceup(d, c) && card::is_monster(d, c)
}
/// `s.condition(e,tp,...)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_REMOVED,0,5,nil)
    duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_REMOVED, 0, 5, NO_EX)
}
/// `s.target(e,tp,...,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_REMOVED) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_REMOVED) && card::is_controler(d, chkc, tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_REMOVED,0,3,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, LOCATION_REMOVED, 0, 3, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,0))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 0));
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_REMOVED,0,3,3,nil)
    let g = duel::select_target(d, tp, Some(&filter), tp, LOCATION_REMOVED, 0, 3, 3, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,3,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, g, 3, 0, 0);
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local tg=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]);
    let tg = info[0].as_group().cloned().nil();
    // local sg=tg:Filter(Card.IsRelateToEffect,nil,e)
    let sg = group::filter(d, &tg, &|d, c| card::is_relate_to_effect(d, c, e), NO_EX);
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.SendtoGrave(sg,REASON_EFFECT|REASON_RETURN)
        duel::send_to_grave(d, &sg, REASON_EFFECT | REASON_RETURN, None, None);
    }
}
