//! Cyber Esper
use super::prelude::*;

pub const ID: u32 = 91663373;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EVENT_DRAW)
    effect::set_code(d, e1, EVENT_DRAW);
    // e1:SetCondition(s.cfcon)
    effect::set_condition(d, e1, Some(condition(cfcon)));
    // e1:SetOperation(s.cfop)
    effect::set_operation(d, e1, Some(operation(cfop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// s.cfcon(e,tp,eg,ep,ev,re,r,rp)
fn cfcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsAttackPos() and ep==1-tp
    let h = effect::get_handler(d, a.e).nil();
    card::is_attack_pos(d, h) && a.ep == 1 - a.tp
}
// s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLocation(LOCATION_HAND) and not c:IsPublic()
    card::is_location(d, c, LOCATION_HAND) && !card::is_public(d, c)
}
// s.cfop(e,tp,eg,ep,ev,re,r,rp)
fn cfop(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():IsRelateToEffect(e) and e:GetHandler():IsPosition(POS_FACEUP_ATTACK) then
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) && card::is_position(d, h, POS_FACEUP_ATTACK) {
        // local cg=eg:Filter(s.filter,nil)
        let cg = group::filter(d, &a.eg.clone().nil(), &filter, NO_EX);
        // Duel.ConfirmCards(tp,cg)
        duel::confirm_cards(d, a.tp, cg);
        // Duel.ShuffleHand(1-tp)
        duel::shuffle_hand(d, 1 - a.tp);
    }
}
