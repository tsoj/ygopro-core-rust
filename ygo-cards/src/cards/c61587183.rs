//! Dark Scorpion - Chick the Yellow
use super::prelude::*;

pub const ID: u32 = 61587183;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e1, EVENT_BATTLE_DAMAGE);
    // e1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return ep==1-tp end)
    effect::set_condition(d, e1, Some(condition(|_d, a| a.ep == 1 - a.tp)));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsAbleToHand() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_able_to_hand(d, chkc, None);
    }
    // local b1=Duel.IsExistingTarget(Card.IsAbleToHand,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
    let b1 = duel::is_existing_target(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    // local b2=Duel.GetFieldGroupCount(tp,0,LOCATION_DECK)>0
    let b2 = duel::get_field_group_count(d, a.tp, 0, LOCATION_DECK) > 0;
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]).nil();
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    if op == 1 {
        // e:SetCategory(CATEGORY_TOHAND)
        effect::set_category(d, a.e, CATEGORY_TOHAND);
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
        // local g=Duel.SelectTarget(tp,Card.IsAbleToHand,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
        let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    } else {
        // e:SetCategory(0)
        effect::set_category(d, a.e, 0);
        // e:SetProperty(0)
        effect::set_property(d, a.e, 0, 0);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // if e:GetLabel()==1 then
    if effect::get_label(d, a.e) == 1 {
        // Return 1 card on the field to the hand
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d);
        // if tc:IsRelateToEffect(e) then
        if card::is_relate_to_effect(d, tc.nil(), a.e) {
            // Duel.SendtoHand(tc,nil,REASON_EFFECT)
            duel::send_to_hand(d, tc.nil(), None, REASON_EFFECT, None);
        }
    } else {
        // Look at the top card of the opponent's Deck
        // local g=Duel.GetDecktopGroup(1-tp,1)
        let g = duel::get_decktop_group(d, 1 - a.tp, 1);
        // if #g>0 then
        if !g.is_empty() {
            // Duel.ConfirmCards(tp,g)
            duel::confirm_cards(d, a.tp, g.clone());
            // Duel.Hint(HINT_SELECTMSG,tp,0)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, 0);
            // local ac=Duel.SelectOption(tp,aux.Stringid(id,3),aux.Stringid(id,4))
            let ac = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 3), aux::stringid(ID, 4)]);
            // if ac==1 then Duel.MoveSequence(g:GetFirst(),1) end
            if ac == 1 {
                let first = group::get_first(d, &g).nil();
                duel::move_sequence(d, first, 1, None);
            }
        }
    }
}
