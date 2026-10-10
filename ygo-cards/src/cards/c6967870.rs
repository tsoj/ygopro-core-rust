//! Dark Scorpion - Cliff the Trap Remover
use super::prelude::*;

pub const ID: u32 = 6967870;

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
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return ep==1-tp end`
fn cond(_d: &mut Duel, a: &EffArgs) -> bool {
    a.ep == 1 - a.tp
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsSpellTrap() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_spell_trap(d, chkc);
    }
    // local b1=Duel.IsExistingTarget(Card.IsSpellTrap,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
    let b1 = duel::is_existing_target(d, Some(&card::is_spell_trap), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    // local b2=Duel.IsPlayerCanDiscardDeck(1-tp,2)
    let b2 = duel::is_player_can_discard_deck(d, 1 - a.tp, 2);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,1)}, {b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
    let op_label = op.map_or(0, |v| v as i64);
    // e:SetLabel(op)
    let e = a.e;
    effect::set_label(d, e, &[op_label]);
    if op_label == 1 {
        // e:SetCategory(CATEGORY_DESTROY)
        effect::set_category(d, e, CATEGORY_DESTROY);
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, e, EFFECT_FLAG_CARD_TARGET, 0);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
        // local g=Duel.SelectTarget(tp,Card.IsSpellTrap,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
        let g = duel::select_target(d, a.tp, Some(&card::is_spell_trap), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    } else {
        // e:SetCategory(CATEGORY_DECKDES)
        effect::set_category(d, e, CATEGORY_DECKDES);
        // e:SetProperty(0)
        effect::set_property(d, e, 0, 0);
        // Duel.SetOperationInfo(0,CATEGORY_DECKDES,nil,0,1-tp,2)
        duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, 1 - a.tp, 2);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // if e:GetLabel()==1 then
    if effect::get_label(d, a.e) == 1 {
        // Destroy 1 Spell/Trap on the field
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d).nil();
        // if tc:IsRelateToEffect(e) then
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.Destroy(tc,REASON_EFFECT)
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    } else {
        // Send the top 2 cards of your opponent's Deck to the GY
        // Duel.DiscardDeck(1-tp,2,REASON_EFFECT)
        duel::discard_deck(d, 1 - a.tp, 2, REASON_EFFECT);
    }
}
