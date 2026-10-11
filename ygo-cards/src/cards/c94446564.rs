//! Matching Outfits
use super::prelude::*;

pub const ID: u32 = 94446564;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_REMOVE);
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
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local g1=Duel.GetDecktopGroup(tp,1)
        let g1 = duel::get_decktop_group(d, a.tp, 1);
        // local tc1=g1:GetFirst()
        let tc1 = group::get_first(d, &g1);
        // local g2=Duel.GetDecktopGroup(1-tp,1)
        let g2 = duel::get_decktop_group(d, 1 - a.tp, 1);
        // local tc2=g2:GetFirst()
        let tc2 = group::get_first(d, &g2);
        // return tc1 and tc2 and tc1:IsAbleToRemove(tp) and tc2:IsAbleToRemove(1-tp) and tc1:IsAbleToHand() and tc2:IsAbleToHand()
        return match (tc1, tc2) {
            (Some(t1), Some(t2)) => {
                card::is_able_to_remove(d, t1, Some(a.tp), None, None)
                    && card::is_able_to_remove(d, t2, Some(1 - a.tp), None, None)
                    && card::is_able_to_hand(d, t1, None)
                    && card::is_able_to_hand(d, t2, None)
            }
            _ => false,
        };
    }
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)==0 or Duel.GetFieldGroupCount(tp,0,LOCATION_DECK)==0 then return end
    if duel::get_field_group_count(d, tp, LOCATION_DECK, 0) == 0 || duel::get_field_group_count(d, tp, 0, LOCATION_DECK) == 0 {
        return;
    }
    // Duel.ConfirmDecktop(tp,1)
    duel::confirm_decktop(d, tp, 1);
    // Duel.ConfirmDecktop(1-tp,1)
    duel::confirm_decktop(d, 1 - tp, 1);
    // local g1=Duel.GetDecktopGroup(tp,1)
    let g1 = duel::get_decktop_group(d, tp, 1);
    // local tc1=g1:GetFirst()
    let tc1 = group::get_first(d, &g1).nil();
    // local g2=Duel.GetDecktopGroup(1-tp,1)
    let g2 = duel::get_decktop_group(d, 1 - tp, 1);
    // local tc2=g2:GetFirst()
    let tc2 = group::get_first(d, &g2).nil();
    // if (tc1:GetMainCardType()==tc2:GetMainCardType()) then
    let t1 = card::get_main_card_type(d, tc1);
    let t2 = card::get_main_card_type(d, tc2);
    if t1 == t2 {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.SendtoHand(tc1,nil,REASON_EFFECT)
        duel::send_to_hand(d, CardSet::from_card(tc1), None, REASON_EFFECT, None);
        // Duel.SendtoHand(tc2,nil,REASON_EFFECT)
        duel::send_to_hand(d, CardSet::from_card(tc2), None, REASON_EFFECT, None);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // Duel.ShuffleHand(1-tp)
        duel::shuffle_hand(d, 1 - tp);
    } else {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.Remove(tc1,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, CardSet::from_card(tc1), POS_FACEUP as u8, REASON_EFFECT, None, None);
        // Duel.Remove(tc2,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, CardSet::from_card(tc2), POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
