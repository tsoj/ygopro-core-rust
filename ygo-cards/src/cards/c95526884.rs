//! Hyper Psychic Blaster
use super::prelude::*;

pub const ID: u32 = 95526884;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro summon
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTunerEx(Card.IsRace,RACE_PSYCHIC),1,99)
    synchro::add_procedure(
        d,
        c,
        None,
        1,
        1,
        Some(synchro::non_tuner_ex(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_PSYCHIC, sc, st, Some(tp)))),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // pierce
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e1, EFFECT_PIERCE);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // recover
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_RECOVER)
    effect::set_category(d, e2, CATEGORY_RECOVER);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e2, EVENT_DAMAGE_STEP_END);
    // e2:SetCondition(s.reccon)
    effect::set_condition(d, e2, Some(condition(reccon)));
    // e2:SetTarget(s.rectg)
    effect::set_target(d, e2, Some(cost_target(rectg)));
    // e2:SetOperation(s.recop)
    effect::set_operation(d, e2, Some(operation(recop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.reccon(e,tp,eg,ep,ev,re,r,rp)
fn reccon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c,a,d,def=e:GetHandler(),Duel.GetAttacker(),Duel.GetAttackTarget(),0
    let c = effect::get_handler(d, a.e).nil();
    let at = duel::get_attacker(d);
    let dt = duel::get_attack_target(d);
    // if not d then return false end
    let Some(dt) = dt else { return false };
    // if not d:IsLocation(LOCATION_MZONE) then def=d:GetPreviousDefenseOnField()
    // else def=d:GetDefense() end
    let def: i64 = if !card::is_location(d, dt, LOCATION_MZONE) {
        card::get_previous_defense_on_field(d, dt) as i64
    } else {
        card::get_defense(d, dt) as i64
    };
    // local m=a:GetAttack()-def
    let m = card::get_attack(d, at.nil()) as i64 - def;
    // e:SetLabel(m)
    effect::set_label(d, a.e, &[m]);
    // return c==a and def>=0 and m>0 and (d:GetBattlePosition()&POS_DEFENSE)~=0
    Some(c) == at && def >= 0 && m > 0 && (card::get_battle_position(d, dt) & POS_DEFENSE) != 0
}
// function s.rectg(e,tp,eg,ep,ev,re,r,rp,chk)
fn rectg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(e:GetLabel())
    let label = effect::get_label(d, a.e);
    duel::set_target_param(d, label as u32);
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,e:GetLabel())
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, label as i32);
    true
}
// function s.recop(e,tp,eg,ep,ev,re,r,rp)
fn recop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Recover(p,d,REASON_EFFECT)
    duel::recover(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
