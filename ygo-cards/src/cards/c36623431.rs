//! Iron Core of Koa'ki Meiru
use super::prelude::*;

pub const ID: u32 = 36623431;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_KOAKI_MEIRU], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_PREDRAW)
    effect::set_code(d, e1, EVENT_PREDRAW);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetCondition(s.condition1)
    effect::set_condition(d, e1, Some(condition(condition1)));
    // e1:SetTarget(s.target1)
    effect::set_target(d, e1, Some(cost_target(target1)));
    // e1:SetOperation(s.operation1)
    effect::set_operation(d, e1, Some(operation(operation1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_PHASE|PHASE_DRAW)
    effect::set_code(d, e2, EVENT_PHASE | PHASE_DRAW);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCondition(s.condition2)
    effect::set_condition(d, e2, Some(condition(condition2)));
    // e2:SetCost(s.cost2)
    effect::set_cost(d, e2, Some(cost_target(cost2)));
    // e2:SetTarget(s.target2)
    effect::set_target(d, e2, Some(cost_target(target2)));
    // e2:SetOperation(s.operation2)
    effect::set_operation(d, e2, Some(operation(operation2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.condition1(e,tp,eg,ep,ev,re,r,rp)`
fn condition1(d: &mut Duel, a: &EffArgs) -> bool {
    // return tp==Duel.GetTurnPlayer() and Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>0
    //     and Duel.GetDrawCount(tp)>0
    a.tp == duel::get_turn_player(d) && duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) > 0 && duel::get_draw_count(d, a.tp) > 0
}
/// `s.target1(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target1(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local h=e:GetHandler()
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsAbleToHand() end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, h, None);
    }
    // local dt=Duel.GetDrawCount(tp)
    let dt = duel::get_draw_count(d, tp);
    // if dt~=0 then
    if dt != 0 {
        // _replace_count=0
        set_lua_global(d, "_replace_count", Value::Int(0));
        // _replace_max=dt
        set_lua_global(d, "_replace_max", Value::Int(dt as i64));
        // local e1=Effect.CreateEffect(e:GetHandler())
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
        duel::register_effect(d, e1, tp);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.operation1(e,tp,eg,ep,ev,re,r,rp)`
fn operation1(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // _replace_count=_replace_count+1
    let count = lua_global(d, "_replace_count").int() + 1;
    set_lua_global(d, "_replace_count", Value::Int(count));
    // if _replace_count<=_replace_max and c:IsRelateToEffect(e) then
    if count <= lua_global(d, "_replace_max").int() && card::is_relate_to_effect(d, c, a.e) {
        // Duel.SendtoHand(c,nil,REASON_EFFECT)
        duel::send_to_hand(d, CardSet::from_card(c), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,c)
        duel::confirm_cards(d, 1 - tp, CardSet::from_card(c));
    }
}
/// `s.condition2(e,tp,eg,ep,ev,re,r,rp)`
fn condition2(d: &mut Duel, a: &EffArgs) -> bool {
    // return tp==Duel.GetTurnPlayer()
    a.tp == duel::get_turn_player(d)
}
/// `s.costfilter(c)`
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_KOAKI_MEIRU) and c:IsMonster() and c:IsAbleToGraveAsCost()
    card::is_set_card(d, c, &[SET_KOAKI_MEIRU as u16]) && card::is_monster(d, c) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.cost2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&costfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,s.costfilter,1,1,REASON_COST,nil)
    duel::discard_hand(d, a.tp, Some(&costfilter), 1, 1, REASON_COST, NO_EX);
    true
}
/// `s.target2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsAbleToHand() end
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, h, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.operation2(e,tp,eg,ep,ev,re,r,rp)`
fn operation2(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SendtoHand(c,nil,REASON_EFFECT)
        duel::send_to_hand(d, CardSet::from_card(c), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,c)
        duel::confirm_cards(d, 1 - tp, CardSet::from_card(c));
    }
}
