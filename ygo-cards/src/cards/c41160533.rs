//! Rose Tentacles
//! ローズ・テンタクルス
use super::prelude::*;

pub const ID: u32 = 41160533;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);

    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_PHASE_START|PHASE_BATTLE_START)
    effect::set_code(d, e2, EVENT_PHASE_START | PHASE_BATTLE_START);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetOperation(s.maop)
    effect::set_operation(d, e2, Some(operation(maop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);

    // special summon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_DAMAGE);
    // e3:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e3, EVENT_BATTLE_DESTROYING);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCondition(s.damcon)
    effect::set_condition(d, e3, Some(condition(damcon)));
    // e3:SetTarget(s.damtg)
    effect::set_target(d, e3, Some(cost_target(damtg)));
    // e3:SetOperation(s.damop)
    effect::set_operation(d, e3, Some(operation(damop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.maop(e,tp,eg,ep,ev,re,r,rp)`
fn maop(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsTurnPlayer(1-tp) then return end
    if duel::is_turn_player(d, 1 - a.tp) {
        return;
    }
    // local ct=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsRace,RACE_PLANT),tp,0,LOCATION_MZONE,nil)
    let ct = duel::get_matching_group_count(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_PLANT)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // if ct~=0 then
    if ct != 0 {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_EXTRA_ATTACK)
        effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
        // e1:SetValue(ct)
        effect::set_value(d, e1, ct as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE|RESET_PHASE|PHASE_BATTLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE | RESET_PHASE | PHASE_BATTLE, 1);
        // e:GetHandler():RegisterEffect(e1)
        card::register_effect(d, h, e1, false);
    }
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetBattleTarget():IsRace(RACE_PLANT)
    let h = effect::get_handler(d, a.e).nil();
    let bt = card::get_battle_target(d, h).nil();
    card::is_race(d, bt, RACE_PLANT)
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(300)
    duel::set_target_param(d, 300);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,300)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 300);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
