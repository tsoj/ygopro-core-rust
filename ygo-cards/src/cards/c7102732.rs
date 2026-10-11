//! Clear Rage Golem
use super::prelude::*;

pub const ID: u32 = 7102732;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_CLEAR_WORLD],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e,tp) return (Duel.IsAbleToEnterBP() or Duel.IsBattlePhase()) and Duel.IsTurnPlayer(tp) and not Duel.HasFlagEffect(tp,id) end)
    effect::set_condition(d, e1, Some(condition(e1_cond)));
    // e1:SetOperation(s.diratkop)
    effect::set_operation(d, e1, Some(operation(diratkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetCode(EFFECT_CLEAR_WORLD_IMMUNE)
    effect::set_code(d, e3, EFFECT_CLEAR_WORLD_IMMUNE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(1,0)
    effect::set_target_range(d, e3, 1, 0);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e4, CATEGORY_DAMAGE);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e4:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e4, EVENT_BATTLE_DAMAGE);
    // e4:SetCondition(function(e,tp,eg,ep) return ep==1-tp end)
    effect::set_condition(d, e4, Some(condition(e4_cond)));
    // e4:SetTarget(s.damtg)
    effect::set_target(d, e4, Some(cost_target(damtg)));
    // e4:SetOperation(s.damop)
    effect::set_operation(d, e4, Some(operation(damop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `function(e,tp) return (Duel.IsAbleToEnterBP() or Duel.IsBattlePhase()) and Duel.IsTurnPlayer(tp) and not Duel.HasFlagEffect(tp,id) end`
fn e1_cond(d: &mut Duel, a: &EffArgs) -> bool {
    (duel::is_able_to_enter_bp(d) || duel::is_battle_phase(d, None)) && duel::is_turn_player(d, a.tp) && !duel::has_flag_effect(d, a.tp, ID, None)
}
/// `function(e,tp,eg,ep) return ep==1-tp end`
fn e4_cond(_d: &mut Duel, a: &EffArgs) -> bool {
    a.ep == 1 - a.tp
}
/// `s.diratkop(e,tp,eg,ep,ev,re,r,rp)`
fn diratkop(d: &mut Duel, a: &EffArgs) {
    // if Duel.HasFlagEffect(tp,id) then return end
    if duel::has_flag_effect(d, a.tp, ID, None) {
        return;
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(function(e,c) return c:ListsCode(CARD_CLEAR_WORLD) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::lists_code(d, c, &[CARD_CLEAR_WORLD]))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,2))
    aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 2)), 0, None);
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ct=Duel.GetFieldGroupCount(1-tp,LOCATION_HAND,0)
    let ct = duel::get_field_group_count(d, 1 - a.tp, LOCATION_HAND, 0);
    // if chk==0 then return ct>0 end
    if a.chk == Some(0) {
        return ct > 0;
    }
    // local dam=ct*300
    let dam = ct * 300;
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(dam)
    duel::set_target_param(d, dam as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,dam)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, dam);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, _a: &EffArgs) {
    // local player=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER]);
    let player = info[0].int() as u8;
    // local dam=Duel.GetFieldGroupCount(player,LOCATION_HAND,0)*300
    let dam = duel::get_field_group_count(d, player, LOCATION_HAND, 0) * 300;
    // if dam>0 then
    if dam > 0 {
        // Duel.Damage(player,dam,REASON_EFFECT)
        duel::damage(d, player, dam as i64, REASON_EFFECT, false, None);
    }
}
