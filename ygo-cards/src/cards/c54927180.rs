//! Fukubiki
use super::prelude::*;

pub const ID: u32 = 54927180;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_DECK,LOCATION_DECK)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, LOCATION_DECK, LOCATION_DECK) > 0;
    }
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // if (Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)==0 or Duel.GetFieldGroupCount(tp,0,LOCATION_DECK)==0) then return end
    if duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) == 0
        || duel::get_field_group_count(d, a.tp, 0, LOCATION_DECK) == 0
    {
        return;
    }
    // if Duel.ShuffleDeck(tp)~=0 and Duel.ShuffleDeck(1-tp)~=0 then
    // (ShuffleDeck yields without a result: the Lua value is nil, and nil~=0 is true)
    duel::shuffle_deck(d, a.tp);
    duel::shuffle_deck(d, 1 - a.tp);
    // Duel.ConfirmDecktop(tp,1)
    duel::confirm_decktop(d, a.tp, 1);
    // Duel.ConfirmDecktop(1-tp,1)
    duel::confirm_decktop(d, 1 - a.tp, 1);
    // local tc1=Duel.GetDecktopGroup(tp,1):GetFirst()
    let tc1 = group::get_first(d, &duel::get_decktop_group(d, a.tp, 1));
    // local tc2=Duel.GetDecktopGroup(1-tp,1):GetFirst()
    let tc2 = group::get_first(d, &duel::get_decktop_group(d, 1 - a.tp, 1));
    // local atk1=tc1:GetTextAttack()
    let mut atk1 = card::get_text_attack(d, tc1.nil());
    // local atk2=tc2:GetTextAttack()
    let mut atk2 = card::get_text_attack(d, tc2.nil());
    // if atk1<0 then atk1=0 end
    if atk1 < 0 {
        atk1 = 0;
    }
    // if atk2<0 then atk2=0 end
    if atk2 < 0 {
        atk2 = 0;
    }
    // if atk1>atk2 and tc1:IsAbleToHand() and tc2:IsAbleToGrave() then
    if atk1 > atk2 && card::is_able_to_hand(d, tc1.nil(), None) && card::is_able_to_grave(d, tc2.nil(), None) {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.SendtoHand(tc1,tp,REASON_EFFECT)
        duel::send_to_hand(d, tc1.nil(), Some(a.tp), REASON_EFFECT, None);
        // Duel.SendtoGrave(tc2,REASON_EFFECT)
        duel::send_to_grave(d, tc2.nil(), REASON_EFFECT, None, None);
    // elseif atk1<atk2 and tc2:IsAbleToHand() and tc1:IsAbleToGrave() then
    } else if atk1 < atk2 && card::is_able_to_hand(d, tc2.nil(), None) && card::is_able_to_grave(d, tc1.nil(), None) {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.SendtoHand(tc2,1-tp,REASON_EFFECT)
        duel::send_to_hand(d, tc2.nil(), Some(1 - a.tp), REASON_EFFECT, None);
        // Duel.SendtoGrave(tc1,REASON_EFFECT)
        duel::send_to_grave(d, tc1.nil(), REASON_EFFECT, None, None);
    } else {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.MoveSequence(tc1,1)
        duel::move_sequence(d, tc1.nil(), 1, None);
        // Duel.MoveSequence(tc2,1)
        duel::move_sequence(d, tc2.nil(), 1, None);
    }
}
