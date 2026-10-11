//! Dark Scorpion - Gorg the Strong
use super::prelude::*;

pub const ID: u32 = 48768179;

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
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsAbleToDeck() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && card::is_able_to_deck(d, chkc);
    }
    // local b1=Duel.IsExistingTarget(Card.IsAbleToDeck,tp,0,LOCATION_MZONE,1,nil)
    let b1 = duel::is_existing_target(d, Some(&card::is_able_to_deck), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    // local b2=Duel.IsPlayerCanDiscardDeck(1-tp,1)
    let b2 = duel::is_player_can_discard_deck(d, 1 - a.tp, 1);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,1)}, {b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]).nil();
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    if op == 1 {
        // e:SetCategory(CATEGORY_TODECK)
        effect::set_category(d, a.e, CATEGORY_TODECK);
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
        // local g=Duel.SelectTarget(tp,Card.IsAbleToDeck,tp,0,LOCATION_MZONE,1,1,nil)
        let g = duel::select_target(d, a.tp, Some(&card::is_able_to_deck), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_TODECK, g, 1, a.tp, 0);
    } else {
        // e:SetCategory(CATEGORY_DECKDES)
        effect::set_category(d, a.e, CATEGORY_DECKDES);
        // e:SetProperty(0)
        effect::set_property(d, a.e, 0, 0);
        // Duel.SetOperationInfo(0,CATEGORY_DECKDES,0,0,1-tp,1)
        duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, 1 - a.tp, 1);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    if effect::get_label(d, a.e) == 1 {
        // Return 1 monster your opponent controls to the top of the Deck
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d).nil();
        // if not (tc:IsRelateToEffect(e) and tc:IsControler(1-tp)) then return end
        if !(card::is_relate_to_effect(d, tc, a.e) && card::is_controler(d, tc, 1 - a.tp)) {
            return;
        }
        // Duel.SendtoDeck(tc,nil,SEQ_DECKTOP,REASON_EFFECT)
        duel::send_to_deck(d, tc, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
    } else {
        // Send the top card of your opponent's Deck to the GY
        // Duel.DiscardDeck(1-tp,1,REASON_EFFECT)
        duel::discard_deck(d, 1 - a.tp, 1, REASON_EFFECT);
    }
}
