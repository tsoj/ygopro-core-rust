//! Extra Gate
use super::prelude::*;

pub const ID: u32 = 7405310;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)~=0
    //     and Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,0,LOCATION_EXTRA,1,nil) end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, tp, LOCATION_HAND, 0) != 0
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_EXTRA, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_LVRANK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_LVRANK as u64);
    // local lv=Duel.AnnounceLevel(tp)
    let lv = duel::announce_number_range(d, tp, None, None, &[]).nil().0;
    // Duel.SetTargetParam(lv)
    duel::set_target_param(d, lv as u32);
    true
}
/// `s.filter(c,lv)`
fn filter(d: &mut Duel, c: CardId, lv: i64) -> bool {
    // return c:GetLevel()==lv
    card::get_level(d, c) as i64 == lv
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local lv=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let lv = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
    // local g=Duel.GetMatchingGroup(s.filter,1-tp,LOCATION_EXTRA,0,nil,lv)
    let g = duel::get_matching_group(d, Some(&|d, c| filter(d, c, lv)), 1 - tp, LOCATION_EXTRA, 0, NO_EX);
    // if #g~=0 then
    if !g.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_REMOVE as u64);
        // local rg=g:FilterSelect(1-tp,Card.IsAbleToRemove,1,1,nil)
        let rg = group::filter_select(d, &g, 1 - tp, &|d, c| card::is_able_to_remove(d, c, None, None, None), 1, 1, false, NO_EX).nil();
        // if #rg~=0 then
        if !rg.is_empty() {
            // Duel.Remove(rg,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, rg, POS_FACEUP as u8, REASON_EFFECT, None, None);
        } else {
            // local cg=Duel.GetFieldGroup(1-tp,LOCATION_EXTRA,0)
            let cg = duel::get_field_group(d, 1 - tp, LOCATION_EXTRA, 0);
            // Duel.ConfirmCards(tp,cg)
            duel::confirm_cards(d, tp, cg);
        }
    } else {
        // Duel.DiscardHand(tp,nil,1,1,REASON_EFFECT|REASON_DISCARD)
        duel::discard_hand(d, tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
    }
}
