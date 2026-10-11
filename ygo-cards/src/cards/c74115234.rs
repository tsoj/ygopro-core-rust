//! Orb of Yasaka
use super::prelude::*;

pub const ID: u32 = 74115234;

pub const INFO: ScriptInfo = ScriptInfo { listed_card_types: &[TYPE_SPIRIT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsType,TYPE_SPIRIT))
    aux::add_equip_procedure(d, c, None, Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_type(d, c, TYPE_SPIRIT))), None, None, None, None, None, None);
    // Gain LP equal to the damage the equipped monster inflicts
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD | EFFECT_TYPE_TRIGGER_F);
    // e1:SetCategory(CATEGORY_RECOVER)
    effect::set_category(d, e1, CATEGORY_RECOVER);
    // e1:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCondition(s.recon)
    effect::set_condition(d, e1, Some(condition(recon)));
    // e1:SetTarget(s.retg)
    effect::set_target(d, e1, Some(cost_target(retg)));
    // e1:SetOperation(s.reop)
    effect::set_operation(d, e1, Some(operation(reop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Return this card to the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE | EFFECT_TYPE_TRIGGER_F);
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCondition(s.retcon)
    effect::set_condition(d, e2, Some(condition(retcon)));
    // e2:SetTarget(s.rettg)
    effect::set_target(d, e2, Some(cost_target(rettg)));
    // e2:SetOperation(s.retop)
    effect::set_operation(d, e2, Some(operation(retop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.recon(e,tp,eg,ep,ev,re,r,rp)
fn recon(d: &mut Duel, a: &EffArgs) -> bool {
    // local ec=eg:GetFirst()
    let ec = group::get_first(d, &a.eg.clone().nil()).nil();
    // local bc=ec:GetBattleTarget()
    let bc = card::get_battle_target(d, ec);
    // e:SetLabelObject(bc)
    effect::set_label_object(d, a.e, bc.map_or(Value::Nil, Value::Card));
    // return e:GetHandler():GetEquipTarget()==eg:GetFirst() and ec:IsControler(tp)
    let h = effect::get_handler(d, a.e).nil();
    card::get_equip_target(d, h) == Some(ec)
        && card::is_controler(d, ec, a.tp)
        // and bc:IsLocation(LOCATION_GRAVE) and bc:IsMonster() and bc:IsReason(REASON_BATTLE)
        && card::is_location(d, bc.nil(), LOCATION_GRAVE)
        && card::is_monster(d, bc.nil())
        && card::is_reason(d, bc.nil(), REASON_BATTLE)
}
// function s.retg(e,tp,eg,ep,ev,re,r,rp,chk)
fn retg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local v=e:GetLabelObject():GetBaseAttack()
    let lo = effect::get_label_object(d, a.e).as_card().nil();
    let mut v = card::get_base_attack(d, lo);
    // if v<0 then v=0 end
    if v < 0 {
        v = 0;
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(v)
    duel::set_target_param(d, v as u32);
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,v)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, v);
    true
}
// function s.reop(e,tp,eg,ep,ev,re,r,rp)
fn reop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Recover(p,d,REASON_EFFECT)
    duel::recover(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
// function s.retcon(e,tp,eg,ep,ev,re,r,rp)
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ec=c:GetPreviousEquipTarget()
    let ec = card::get_previous_equip_target(d, c);
    // return c:IsReason(REASON_LOST_TARGET) and ec and ec:IsLocation(LOCATION_HAND) and ec:IsPreviousControler(tp)
    if !card::is_reason(d, c, REASON_LOST_TARGET) {
        return false;
    }
    match ec {
        Some(ec) => card::is_location(d, ec, LOCATION_HAND) && card::is_previous_controler(d, ec, a.tp),
        None => false,
    }
}
// function s.rettg(e,tp,eg,ep,ev,re,r,rp,chk)
fn rettg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsAbleToHand() end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
// function s.retop(e,tp,eg,ep,ev,re,r,rp)
fn retop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SendtoHand(c,nil,REASON_EFFECT)
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,c)
        duel::confirm_cards(d, 1 - a.tp, c);
    }
}
