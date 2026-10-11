//! Voltic Bicorn
use super::prelude::*;

pub const ID: u32 = 13995824;

pub const INFO: ScriptInfo = ScriptInfo { ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Synchro Summon
    // Synchro.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_BEAST),1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_BEAST, sc, st, Some(tp)))),
        1,
        1,
        Some(synchro::non_tuner(None)),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // deckdes
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DECKDES)
    effect::set_category(d, e1, CATEGORY_DECKDES);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e1, EVENT_DESTROYED);
    // e1:SetCondition(s.ddcon)
    effect::set_condition(d, e1, Some(condition(ddcon)));
    // e1:SetTarget(s.ddtg)
    effect::set_target(d, e1, Some(cost_target(ddtg)));
    // e1:SetOperation(s.ddop)
    effect::set_operation(d, e1, Some(operation(ddop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.ddcon(e,tp,eg,ep,ev,re,r,rp)
fn ddcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==1-tp and e:GetHandler():IsPreviousControler(tp)
    let c = effect::get_handler(d, a.e).nil();
    a.rp == 1 - a.tp && card::is_previous_controler(d, c, a.tp)
}
// function s.ddtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn ddtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DECKDES,0,0,PLAYER_ALL,7)
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, PLAYER_ALL as u8, 7);
    true
}
// function s.ddop(e,tp,eg,ep,ev,re,r,rp)
fn ddop(d: &mut Duel, a: &EffArgs) {
    // Duel.DiscardDeck(tp,7,REASON_EFFECT)
    duel::discard_deck(d, a.tp, 7, REASON_EFFECT);
    // Duel.DiscardDeck(1-tp,7,REASON_EFFECT)
    duel::discard_deck(d, 1 - a.tp, 7, REASON_EFFECT);
}
