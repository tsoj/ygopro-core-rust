//! Swap Cleric
use super::prelude::*;

pub const ID: u32 = 30968774;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Draw
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DRAW+CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_DRAW + CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_BE_MATERIAL)
    effect::set_code(d, e1, EVENT_BE_MATERIAL);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_DELAY, 0);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLocation(LOCATION_GRAVE) and r & REASON_LINK == REASON_LINK
    let h = effect::get_handler(d, a.e).nil();
    card::is_location(d, h, LOCATION_GRAVE) && (a.r & REASON_LINK) == REASON_LINK
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local sc=e:GetHandler():GetReasonCard()
    let h = effect::get_handler(d, a.e).nil();
    let sc = card::get_reason_card(d, h);
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) and sc:IsAttackAbove(500) and sc:IsLinkMonster() end
    if a.chk == Some(0) {
        if !duel::is_player_can_draw(d, tp, 1) {
            return false;
        }
        let sc = sc.nil();
        return card::is_attack_above(d, sc, 500) && card::is_link_monster(d, sc);
    }
    // Duel.SetTargetCard(sc)
    duel::set_target_card(d, sc.nil());
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,sc,1,LOCATION_MZONE,500)
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(CardSet::from_card(sc.nil())), 1, LOCATION_MZONE as u8, 500);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local sc=Duel.GetFirstTarget()
    let sc = duel::first_target(d).nil();
    // if not sc:IsFaceup() or sc:GetAttack()<500 or not sc:IsRelateToEffect(e) or not sc:IsLinkMonster() then return end
    if !card::is_faceup(d, sc) || card::get_attack(d, sc) < 500 || !card::is_relate_to_effect(d, sc, a.e) || !card::is_link_monster(d, sc) {
        return;
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(-500)
    effect::set_value(d, e1, -500);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // sc:RegisterEffect(e1)
    card::register_effect(d, sc, e1, false);
    // if not sc:IsImmuneToEffect(e1) and not sc:IsHasEffect(EFFECT_REVERSE_UPDATE) then
    if !card::is_immune_to_effect(d, sc, e1) && card::is_has_effect(d, sc, EFFECT_REVERSE_UPDATE, None).is_empty() {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
        let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
        // Duel.Draw(p,d,REASON_EFFECT)
        duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
    }
}
