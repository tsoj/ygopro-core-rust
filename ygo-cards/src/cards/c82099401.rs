//! Crystal Seer
use super::prelude::*;

pub const ID: u32 = 82099401;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // flip
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_FLIP)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.operation(e,tp,...)`
fn op(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)<2 then return end
    if duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) < 2 {
        return;
    }
    // Duel.ConfirmDecktop(tp,2)
    duel::confirm_decktop(d, a.tp, 2);
    // local g=Duel.GetDecktopGroup(tp,2)
    let g = duel::get_decktop_group(d, a.tp, 2);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local add=g:Select(tp,1,1,nil)
        let add = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        // if add:GetFirst():IsAbleToHand() then
        let tc = group::get_first(d, &add).nil();
        if card::is_able_to_hand(d, tc, None) {
            // Duel.SendtoHand(add,nil,REASON_EFFECT)
            duel::send_to_hand(d, add.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,add)
            duel::confirm_cards(d, 1 - a.tp, add.clone());
            // Duel.ShuffleHand(tp)
            duel::shuffle_hand(d, a.tp);
        } else {
            // Duel.SendtoGrave(add,REASON_EFFECT)
            duel::send_to_grave(d, add, REASON_EFFECT, None, None);
        }
        // local back=Duel.GetDecktopGroup(tp,1)
        let back = duel::get_decktop_group(d, a.tp, 1);
        // Duel.MoveSequence(back:GetFirst(),1)
        let back_first = group::get_first(d, &back).nil();
        duel::move_sequence(d, back_first, 1, None);
    }
}
