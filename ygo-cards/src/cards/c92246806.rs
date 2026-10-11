//! Yosenju Kama 2
use super::prelude::*;

pub const ID: u32 = 92246806;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_YOSENJU], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Perform a Normal Summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SUMMON)
    effect::set_category(d, e1, CATEGORY_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetTarget(s.sumtg)
    effect::set_target(d, e1, Some(cost_target(sumtg)));
    // e1:SetOperation(s.sumop)
    effect::set_operation(d, e1, Some(operation(sumop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Direct attack
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e2, EFFECT_DIRECT_ATTACK);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Reduce Damage
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
    effect::set_code(d, e3, EFFECT_CHANGE_BATTLE_DAMAGE);
    // e3:SetCondition(s.rdcon)
    effect::set_condition(d, e3, Some(condition(|d, a| rdcon(d, a))));
    // e3:SetValue(aux.ChangeBattleDamage(1,HALF_DAMAGE))
    effect::set_value_fn(d, e3, aux::change_battle_damage(1, HALF_DAMAGE as i64));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Return to the hand
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e4:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SUMMON_SUCCESS);
    // e4:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e4:SetOperation(s.regop)
    effect::set_operation(d, e4, Some(operation(regop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_YOSENJU) and not c:IsCode(id) and c:IsSummonable(true,nil)
    card::is_set_card(d, c, &[SET_YOSENJU as u16]) && !card::is_code(d, c, &[ID]) && card::is_summonable(d, c, true, None, 0, None)
}
/// `s.sumtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sumtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SUMMON,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SUMMON, None, 1, 0, 0);
    true
}
/// `s.sumop(e,tp,eg,ep,ev,re,r,rp)`
fn sumop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&filter), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if g.len() > 0 {
        // Duel.Summon(tp,g:GetFirst(),true,nil)
        let tc = group::get_first(d, &g).nil();
        duel::summon(d, a.tp, tc, true, None, 0, None);
    }
}
/// `s.rdcon(e)`
fn rdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e);
    // return Duel.GetAttackTarget()==nil and e:GetHandler():GetEffectCount(EFFECT_DIRECT_ATTACK)<2
    //   and Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>0
    if duel::get_attack_target(d).is_some() {
        return false;
    }
    let c = effect::get_handler(d, a.e).nil();
    card::get_effect_count(d, c, EFFECT_DIRECT_ATTACK) < 2 && duel::get_field_group_count(d, tp, 0, LOCATION_MZONE) > 0
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetTarget(s.rettg)
    effect::set_target(d, e1, Some(cost_target(rettg)));
    // e1:SetOperation(s.retop)
    effect::set_operation(d, e1, Some(operation(retop)));
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD&~(RESET_TEMP_REMOVE|RESET_TURN_SET)|RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !(RESET_TEMP_REMOVE | RESET_TURN_SET)) | RESET_PHASE | PHASE_END, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.rettg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rettg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,e:GetHandler(),1,0,0)
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.retop(e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SendtoHand(c,nil,REASON_EFFECT)
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
    }
}
