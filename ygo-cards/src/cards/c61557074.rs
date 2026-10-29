//! Palace of the Elemental Lords
use super::prelude::*;

pub const ID: u32 = 61557074;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ELEMENTSABER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // boost
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetValue(s.atkval)
    effect::set_value_fn(d, e2, effect_value(atkval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // search
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e4, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_FZONE)
    effect::set_range(d, e4, LOCATION_FZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetTarget(s.thtg)
    effect::set_target(d, e4, Some(cost_target(thtg)));
    // e4:SetOperation(s.thop)
    effect::set_operation(d, e4, Some(operation(thop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // change cost
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e5, aux::stringid(ID, 1));
    // e5:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD);
    // e5:SetCode(id)
    effect::set_code(d, e5, ID);
    // e5:SetRange(LOCATION_FZONE)
    effect::set_range(d, e5, LOCATION_FZONE);
    // e5:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e5, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e5:SetTargetRange(1,0)
    effect::set_target_range(d, e5, 1, 0);
    // e5:SetCondition(s.condition)
    effect::set_condition(d, e5, Some(condition(cond)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // local e6=e5:Clone()
    let e6 = effect::clone(d, e5);
    // e6:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE);
    // e6:SetProperty(EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e6, EFFECT_FLAG_CLIENT_HINT, 0);
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
// function s.atkval(e,c)
fn atkval(d: &mut Duel, e: EffectId) -> i64 {
    // local g=Duel.GetMatchingGroup(Card.IsMonster,e:GetHandlerPlayer(),LOCATION_GRAVE,0,nil)
    let p = effect::get_handler_player(d, e);
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c)), p, LOCATION_GRAVE, 0, NO_EX);
    // return g:GetClassCount(Card.GetAttribute)*200
    group::get_class_count(d, &g, &|d, c| card::get_attribute(d, c) as i64) as i64 * 200
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ELEMENTSABER) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_ELEMENTSABER as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SendtoHand(g,nil,REASON_EFFECT)~=0 then
    if !g.is_empty() && duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) != 0 {
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, &g);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetCode(EFFECT_SKIP_BP)
        effect::set_code(d, e1, EFFECT_SKIP_BP);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // if Duel.IsTurnPlayer(tp) then
        if duel::is_turn_player(d, tp) {
            // e1:SetLabel(Duel.GetTurnCount())
            let tc = duel::get_turn_count(d, None) as i64;
            effect::set_label(d, e1, &[tc]);
            // e1:SetCondition(s.skipcon)
            effect::set_condition(d, e1, Some(condition(skipcon)));
            // e1:SetReset(RESET_PHASE|PHASE_END|RESET_SELF_TURN,2)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_SELF_TURN, 2);
        } else {
            // e1:SetReset(RESET_PHASE|PHASE_END|RESET_SELF_TURN,1)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_SELF_TURN, 1);
        }
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
    }
}
// function s.skipcon(e)
fn skipcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetTurnCount()~=e:GetLabel()
    duel::get_turn_count(d, None) as i64 != effect::get_label(d, a.e)
}
// function s.condition(e)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetFlagEffect(id)==0
    let h = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, h, ID) == 0
}
