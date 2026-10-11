//! Meteor Flare
//! メテオ・プロミネンス
use super::prelude::*;

pub const ID: u32 = 56856951;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // damage
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.damcon)
    effect::set_condition(d, e1, Some(condition(damcon)));
    // e1:SetCost(s.damcost)
    effect::set_cost(d, e1, Some(cost_target(damcost)));
    // e1:SetTarget(s.damtg)
    effect::set_target(d, e1, Some(cost_target(damtg)));
    // e1:SetOperation(s.damop)
    effect::set_operation(d, e1, Some(operation(damop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // salvage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_PREDRAW)
    effect::set_code(d, e2, EVENT_PREDRAW);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCondition(s.thcon)
    effect::set_condition(d, e2, Some(condition(thcon)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetLP(1-tp)>3000
    duel::get_lp(d, 1 - a.tp) > 3000
}
/// `s.damcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToGraveAsCost,tp,LOCATION_HAND,0,2,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_grave_as_cost(d, c)), a.tp, LOCATION_HAND, 0, 2, NO_EX);
    }
    // Duel.DiscardHand(tp,Card.IsAbleToGraveAsCost,2,2,REASON_COST)
    duel::discard_hand(d, a.tp, Some(&|d, c| card::is_able_to_grave_as_cost(d, c)), 2, 2, REASON_COST, NO_EX);
    true
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(2000)
    duel::set_target_param(d, 2000);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,2000)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 2000);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let dn = info[1].int();
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, p, dn, REASON_EFFECT, false, None);
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return tp==Duel.GetTurnPlayer() and Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>0
    //     and Duel.GetDrawCount(tp)>0
    a.tp == duel::get_turn_player(d) && duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) > 0 && duel::get_draw_count(d, a.tp) > 0
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsAbleToHand() end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_able_to_hand(d, h, None);
    }
    // local dt=Duel.GetDrawCount(tp)
    let dt = duel::get_draw_count(d, a.tp);
    // if dt~=0 then
    if dt != 0 {
        // _replace_count=0
        set_lua_global(d, "_replace_count", Value::Int(0));
        // _replace_max=dt
        set_lua_global(d, "_replace_max", Value::Int(dt as i64));
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e1:SetCode(EFFECT_DRAW_COUNT)
        effect::set_code(d, e1, EFFECT_DRAW_COUNT);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // e1:SetReset(RESET_PHASE|PHASE_DRAW)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_DRAW, 1);
        // e1:SetValue(0)
        effect::set_value(d, e1, 0);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // _replace_count=_replace_count+1
    let count = lua_global(d, "_replace_count").int() + 1;
    set_lua_global(d, "_replace_count", Value::Int(count));
    // if _replace_count<=_replace_max and c:IsRelateToEffect(e) then
    if count <= lua_global(d, "_replace_max").int() && card::is_relate_to_effect(d, c, a.e) {
        // Duel.SendtoHand(c,nil,REASON_EFFECT)
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,c)
        duel::confirm_cards(d, 1 - a.tp, CardSet::from_card(c));
    }
}
