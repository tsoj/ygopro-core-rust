//! Conscription
use super::prelude::*;

pub const ID: u32 = 31000575;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_DECK)~=0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, 0, LOCATION_DECK) != 0;
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.ConfirmDecktop(1-tp,1)
    let _ = duel::confirm_decktop(d, 1 - tp, 1);
    // local g=Duel.GetDecktopGroup(1-tp,1)
    let g = duel::get_decktop_group(d, 1 - tp, 1);
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // if not tc then return end
    let Some(tc) = tc else {
        return;
    };
    // if tc:IsSummonableCard() and tc:IsCanBeSpecialSummoned(e,0,tp,false,false) then
    if card::is_summonable_card(d, tc) && card::is_can_be_special_summoned(d, tc, a.e, 0, tp, false, false, None, None, None) {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    // elseif tc:IsAbleToHand() then
    } else if card::is_able_to_hand(d, tc, None) {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
        // Duel.ShuffleHand(1-tp)
        duel::shuffle_hand(d, 1 - tp);
    } else {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.SendtoGrave(tc,REASON_RULE)
        duel::send_to_grave(d, tc, REASON_RULE, None, None);
    }
}
