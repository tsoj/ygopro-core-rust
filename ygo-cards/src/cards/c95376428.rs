//! Extra Net
use super::prelude::*;

pub const ID: u32 = 95376428;

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
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e2, CATEGORY_DRAW);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(operation_fn)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(Card.IsPreviousLocation,1,nil,LOCATION_EXTRA)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| card::is_previous_location(d, c, LOCATION_EXTRA), 1, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsRelateToEffect(e) end
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return card::is_relate_to_effect(d, c, a.e);
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // if eg:IsExists(Card.IsSummonPlayer,1,nil,tp) and Duel.IsPlayerCanDraw(1-tp,1)
    //     and Duel.SelectYesNo(1-tp,aux.Stringid(id,1)) then
    let eg = a.eg.clone().nil();
    if group::is_exists(d, &eg, &|d, c| card::is_summon_player(d, c, tp), 1, NO_EX)
        && duel::is_player_can_draw(d, 1 - tp, 1)
        && duel::select_yes_no(d, 1 - tp, aux::stringid(ID, 1))
    {
        // Duel.Draw(1-tp,1,REASON_EFFECT)
        duel::draw(d, 1 - tp, 1, REASON_EFFECT);
    }
    // if eg:IsExists(Card.IsSummonPlayer,1,nil,1-tp) and Duel.IsPlayerCanDraw(tp,1)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if group::is_exists(d, &eg, &|d, c| card::is_summon_player(d, c, 1 - tp), 1, NO_EX)
        && duel::is_player_can_draw(d, tp, 1)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
    {
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, tp, 1, REASON_EFFECT);
    }
}
