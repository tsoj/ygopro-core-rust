//! Assault Lion
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 100200293;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddNormalSummonProcedure(c,true,true,1,1,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0),s.tribfilter)
    let _e0 = aux::add_normal_summon_procedure(d, c, true, true, Some(1), Some(1), Some(SUMMON_TYPE_TRIBUTE), Some(aux::stringid(ID, 0)), Some(Arc::new(tribfilter)), None);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
    effect::set_code(d, e1, EFFECT_CHANGE_BATTLE_DAMAGE);
    // e1:SetCondition(function(e) return e:GetHandler():GetBattleTarget() end)
    effect::set_condition(d, e1, Some(lua_fn(battle_target_cond)));
    // e1:SetValue(aux.ChangeBattleDamage(1,DOUBLE_DAMAGE))
    effect::set_value_fn(d, e1, aux::change_battle_damage(1, DOUBLE_DAMAGE as i64));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2a, aux::stringid(ID, 1));
    // e2a:SetCategory(CATEGORY_TOHAND+CATEGORY_SUMMON+CATEGORY_ATKCHANGE+CATEGORY_REMOVE)
    effect::set_category(d, e2a, CATEGORY_TOHAND + CATEGORY_SUMMON + CATEGORY_ATKCHANGE + CATEGORY_REMOVE);
    // e2a:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2a, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2a:SetProperty(EFFECT_FLAG_DELAY,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e2a, EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e2a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2a, EVENT_SUMMON_SUCCESS);
    // e2a:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2a, LOCATION_GRAVE);
    // e2a:SetCountLimit(1,id)
    effect::set_count_limit(d, e2a, 1, ID, 0, 0);
    // e2a:SetCondition(s.thcon)
    effect::set_condition(d, e2a, Some(condition(thcon)));
    // e2a:SetTarget(s.thtg)
    effect::set_target(d, e2a, Some(cost_target(thtg)));
    // e2a:SetOperation(s.thop)
    effect::set_operation(d, e2a, Some(operation(thop)));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=e2a:Clone()
    let e2b = effect::clone(d, e2a);
    // e2b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2b, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
}
/// `s.tribfilter(c,tp)`
fn tribfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsRace(RACE_BEASTWARRIOR) and (c:IsControler(tp) or c:IsFaceup())
    card::is_race(d, c, RACE_BEASTWARRIOR) && (card::is_controler(d, c, tp) || card::is_faceup(d, c))
}
/// `s.thconfilter(c,tp)`
fn thconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsRace(RACE_BEASTWARRIOR) and c:IsControler(tp) and c:IsFaceup()
    card::is_race(d, c, RACE_BEASTWARRIOR) && card::is_controler(d, c, tp) && card::is_faceup(d, c)
}
/// `function(e) return e:GetHandler():GetBattleTarget() end`
fn battle_target_cond(d: &mut Duel, a: Args) -> bool {
    // local h = e:GetHandler()
    let h = effect::get_handler(d, a.e(0)).nil();
    card::get_battle_target(d, h).is_some()
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.thconfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| thconfilter(d, c, a.tp), 1, NO_EX)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHand() end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SendtoHand(c,nil,REASON_EFFECT)>0 and c:IsLocation(LOCATION_HAND) then
    if card::is_relate_to_effect(d, c, a.e) && duel::send_to_hand(d, c, None, REASON_EFFECT, None) > 0 && card::is_location(d, c, LOCATION_HAND) {
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // if c:IsSummonable(true,nil,1) and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if card::is_summonable(d, c, true, None, 1, None) && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
            // Duel.Summon(tp,c,true,nil,1)
            duel::summon(d, tp, c, true, None, 1, None);
            // c:UpdateAttack(500,RESET_EVENT|(RESETS_STANDARD_DISABLE&~RESET_TOFIELD))
            card::update_attack(d, c, 500, Some(RESET_EVENT | (RESETS_STANDARD_DISABLE & !RESET_TOFIELD)), None, None);
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetDescription(3300)
            effect::set_description(d, e1, 3300);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
            effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
            // e1:SetValue(LOCATION_REMOVED)
            effect::set_value(d, e1, LOCATION_REMOVED as i64);
            // e1:SetReset(RESET_EVENT|RESETS_REDIRECT)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
        }
    }
}
