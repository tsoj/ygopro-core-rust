//! Sunlight Unicorn
use super::prelude::*;

pub const ID: u32 = 10321588;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Excavate the top card of your Deck and add it to the hand if it is an Equip Spell Card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetDecktopGroup(tp,1)
    let g = duel::get_decktop_group(d, tp, 1);
    // if chk==0 then return #g>0 and g:GetFirst():IsAbleToHand() end
    if a.chk == Some(0) {
        let first = group::get_first(d, &g);
        return !g.is_empty() && first.is_some_and(|f| card::is_able_to_hand(d, f, None));
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)==0 then return end
    if duel::get_field_group_count(d, tp, LOCATION_DECK, 0) == 0 {
        return;
    }
    // Duel.ConfirmDecktop(tp,1)
    duel::confirm_decktop(d, tp, 1);
    // local g=Duel.GetDecktopGroup(tp,1)
    let g = duel::get_decktop_group(d, tp, 1);
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g).nil();
    // if tc:IsEquipSpell() then
    if card::is_equip_spell(d, tc) {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // if tc:IsAbleToHand() then
        if card::is_able_to_hand(d, tc, None) {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g, None, REASON_EFFECT, None);
            // Duel.ShuffleHand(tp)
            duel::shuffle_hand(d, tp);
        } else {
            // Duel.SendtoGrave(tc,REASON_RULE)
            duel::send_to_grave(d, tc, REASON_RULE, None, None);
        }
    } else {
        // Duel.MoveSequence(tc,SEQ_DECKBOTTOM)
        duel::move_sequence(d, tc, SEQ_DECKBOTTOM as u8, None);
    }
}
