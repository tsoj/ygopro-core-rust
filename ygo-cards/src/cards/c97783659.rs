//! Blood Sucker
use super::prelude::*;

pub const ID: u32 = 97783659;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // handes
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DECKDES)
    effect::set_category(d, e1, CATEGORY_DECKDES);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e1, EVENT_BATTLE_DAMAGE);
    // e1:SetCondition(s.ddcon)
    effect::set_condition(d, e1, Some(condition(ddcon)));
    // e1:SetTarget(s.ddtg)
    effect::set_target(d, e1, Some(cost_target(ddtg)));
    // e1:SetOperation(s.ddop)
    effect::set_operation(d, e1, Some(operation(ddop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.ddcon(e,tp,eg,ep,ev,re,r,rp)`
fn ddcon(_d: &mut Duel, a: &EffArgs) -> bool {
    // return ep~=tp
    a.ep != a.tp
}
/// `s.ddtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn ddtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DECKDES,0,0,1-tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, 1 - a.tp, 1);
    true
}
/// `s.ddop(e,tp,eg,ep,ev,re,r,rp)`
fn ddop(d: &mut Duel, a: &EffArgs) {
    // Duel.DiscardDeck(1-tp,1,REASON_EFFECT)
    duel::discard_deck(d, 1 - a.tp, 1, REASON_EFFECT);
}
