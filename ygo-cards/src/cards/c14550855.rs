//! Infernity Inferno
use super::prelude::*;

pub const ID: u32 = 14550855;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_INFERNITY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_HANDES+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_HANDES + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_INFERNITY) and c:IsAbleToGrave()
    card::is_set_card(d, c, &[SET_INFERNITY as u16]) && card::is_able_to_grave(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)>0
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, tp, LOCATION_HAND, 0) > 0
            && duel::is_existing_matching_card(d, Some(&filter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, tp, 1);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)==0 then return end
    if duel::get_field_group_count(d, tp, LOCATION_HAND, 0) == 0 {
        return;
    }
    // local ac=Duel.GetMatchingGroupCount(s.filter,tp,LOCATION_DECK,0,nil)
    let mut ac = duel::get_matching_group_count(d, Some(&filter), tp, LOCATION_DECK, 0, NO_EX);
    // if ac==0 then return end
    if ac == 0 {
        return;
    }
    // if ac>2 then ac=2 end
    if ac > 2 {
        ac = 2;
    }
    // local ct=Duel.DiscardHand(tp,aux.TRUE,1,ac,REASON_DISCARD|REASON_EFFECT)
    let ct = duel::discard_hand(d, tp, None, 1, ac as u16, REASON_DISCARD | REASON_EFFECT, NO_EX);
    // Duel.BreakEffect()
    duel::break_effect(d);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,ct,ct,nil)
    let g = duel::select_matching_card(d, tp, Some(&filter), tp, LOCATION_DECK, 0, ct as u16, ct as u16, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_EFFECT)
    duel::send_to_grave(d, g, REASON_EFFECT, None, None);
}
