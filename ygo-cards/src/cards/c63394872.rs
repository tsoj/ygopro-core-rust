//! Senet Switch
use super::prelude::*;

pub const ID: u32 = 63394872;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // move
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.seqtg)
    effect::set_target(d, e2, Some(cost_target(seqtg)));
    // e2:SetOperation(s.seqop)
    effect::set_operation(d, e2, Some(operation(seqop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.seqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn seqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and chkc:CheckAdjacent() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && card::check_adjacent(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.CheckAdjacent,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::check_adjacent(d, c)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 1));
    // local tc=Duel.SelectTarget(tp,Card.CheckAdjacent,tp,LOCATION_MZONE,0,1,1,nil):GetFirst()
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::check_adjacent(d, c)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g).nil();
    // e:SetLabel(tc:SelectAdjacent())
    if let Some(seq) = card::select_adjacent(d, tc, None) {
        effect::set_label(d, a.e, &[seq as i64]);
    }
    true
}
/// `s.seqop(e,tp,eg,ep,ev,re,r,rp)`
fn seqop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // local seq=e:GetLabel()
    let seq = effect::get_label(d, a.e);
    // if tc:IsImmuneToEffect(e) or not tc:IsRelateToEffect(e) or tc:IsControler(1-tp) or not Duel.CheckLocation(tp,LOCATION_MZONE,seq) then return end
    if card::is_immune_to_effect(d, tc, a.e) || !card::is_relate_to_effect(d, tc, a.e) || card::is_controler(d, tc, 1 - a.tp) || !duel::check_location(d, a.tp, LOCATION_MZONE, seq as u32) {
        return;
    }
    // Duel.MoveSequence(tc,seq)
    duel::move_sequence(d, tc, seq as u8, None);
}
