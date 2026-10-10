//! Ehren, Lightsworn Monk
use super::prelude::*;

pub const ID: u32 = 44178886;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // to deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_BATTLE_CONFIRM)
    effect::set_code(d, e1, EVENT_BATTLE_CONFIRM);
    // e1:SetTarget(s.targ)
    effect::set_target(d, e1, Some(cost_target(targ)));
    // e1:SetOperation(s.op)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // discard deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCategory(CATEGORY_DECKDES)
    effect::set_category(d, e2, CATEGORY_DECKDES);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.discon)
    effect::set_condition(d, e2, Some(condition(discon)));
    // e2:SetTarget(s.distg)
    effect::set_target(d, e2, Some(cost_target(distg)));
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.targ(e,tp,eg,ep,ev,re,r,rp,chk)`
fn targ(d: &mut Duel, a: &EffArgs) -> bool {
    // local t=Duel.GetAttackTarget()
    let t = duel::get_attack_target(d);
    // if chk==0 then return Duel.GetAttacker()==e:GetHandler() and t~=nil and not t:IsAttackPos() and t:IsAbleToDeck() end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::get_attacker(d) == Some(h) && t.is_some_and(|t| !card::is_attack_pos(d, t) && card::is_able_to_deck(d, t));
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,t,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, t.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.op(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, _a: &EffArgs) {
    // local t=Duel.GetAttackTarget()
    let t = duel::get_attack_target(d);
    // if t~=nil and t:IsRelateToBattle() and not t:IsAttackPos() then
    if let Some(t) = t {
        if card::is_relate_to_battle(d, t) && !card::is_attack_pos(d, t) {
            // Duel.SendtoDeck(t,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
            duel::send_to_deck(d, t, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        }
    }
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // return tp==Duel.GetTurnPlayer()
    a.tp == duel::get_turn_player(d)
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DECKDES,nil,0,tp,3)
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, a.tp, 3);
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // Duel.DiscardDeck(tp,3,REASON_EFFECT)
    duel::discard_deck(d, a.tp, 3, REASON_EFFECT);
}
