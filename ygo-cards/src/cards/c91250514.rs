//! タン・ツイスター (Tongue Twister)
use super::prelude::*;

pub const ID: u32 = 91250514;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // draw
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    // return e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD)
    //     and (e:GetHandler():GetSummonType()&SUMMON_TYPE_TRIBUTE)==SUMMON_TYPE_TRIBUTE
    card::is_previous_location(d, c, LOCATION_ONFIELD) && (card::get_summon_type(d, c) & SUMMON_TYPE_TRIBUTE) == SUMMON_TYPE_TRIBUTE
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,2)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 2);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,e:GetHandler(),1,0,0)
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.Draw(tp,2,REASON_EFFECT)
    duel::draw(d, tp, 2, REASON_EFFECT);
    // Duel.BreakEffect()
    duel::break_effect(d);
    // if e:GetHandler():IsRelateToEffect(e) and not Duel.IsPlayerAffectedByEffect(e:GetHandlerPlayer(),CARD_SPIRIT_ELIMINATION) then
    let c = effect::get_handler(d, e).nil();
    if card::is_relate_to_effect(d, c, e) && duel::is_player_affected_by_effect(d, effect::get_handler_player(d, e), CARD_SPIRIT_ELIMINATION).is_none() {
        // Duel.Remove(e:GetHandler(),POS_FACEUP,REASON_EFFECT)
        duel::remove(d, CardSet::from_card(c), POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
