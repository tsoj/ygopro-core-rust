//! Heavenly Gate of the Mikanko
use super::prelude::*;

pub const ID: u32 = 17255673;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MIKANKO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Opponent's monsters must attack equipped monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_MUST_ATTACK)
    effect::set_code(d, e1, EFFECT_MUST_ATTACK);
    // e1:SetRange(LOCATION_FZONE)
    effect::set_range(d, e1, LOCATION_FZONE);
    // e1:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    // e1:SetCondition(s.atkcon)
    effect::set_condition(d, e1, Some(condition(atkcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_MUST_ATTACK_MONSTER)
    effect::set_code(d, e2, EFFECT_MUST_ATTACK_MONSTER);
    // e2:SetValue(aux.TargetBoolFunction(s.atkfilter))
    effect::set_value_fn(d, e2, effect_card_fn(|d, _e, c| atkfilter(d, c)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Prevent activations during battle
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e3, EFFECT_CANNOT_ACTIVATE);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetTargetRange(0,1)
    effect::set_target_range(d, e3, 0, 1);
    // e3:SetCondition(s.actcon)
    effect::set_condition(d, e3, Some(condition(actcon)));
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Attack another monster in a row
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e4, EVENT_DAMAGE_STEP_END);
    // e4:SetRange(LOCATION_FZONE)
    effect::set_range(d, e4, LOCATION_FZONE);
    // e4:SetCondition(s.cacon)
    effect::set_condition(d, e4, Some(condition(cacon)));
    // e4:SetCost(s.cacost)
    effect::set_cost(d, e4, Some(cost_target(cacost)));
    // e4:SetOperation(s.caop)
    effect::set_operation(d, e4, Some(operation(caop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.atkfilter(c)`
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetEquipCount()>0
    card::get_equip_count(d, c) > 0
}
/// `s.atkcon(e)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.atkfilter,e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil)
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&atkfilter), p, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.actcon(e)`
fn actcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc=Duel.GetBattleMonster(e:GetHandlerPlayer())
    let p = effect::get_handler_player(d, a.e);
    let (bc, _) = duel::get_battle_monster(d, p);
    // return bc and bc:IsFaceup() and bc:IsSetCard(SET_MIKANKO)
    match bc {
        Some(bc) => card::is_faceup(d, bc) && card::is_set_card(d, bc, &[SET_MIKANKO as u16]),
        None => false,
    }
}
/// `s.cacostfilter(c)`
fn cacostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_EQUIP) and c:IsAbleToGrave()
    card::is_type(d, c, TYPE_EQUIP) && card::is_able_to_grave(d, c, None)
}
/// `s.cacon(e,tp,eg,ep,ev,re,r,rp)`
fn cacon(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc=Duel.GetBattleMonster(tp)
    let (bc, _) = duel::get_battle_monster(d, a.tp);
    // return bc and bc:IsSetCard(SET_MIKANKO) and bc==Duel.GetAttacker() and bc:CanChainAttack(0,true)
    match bc {
        Some(bc) => {
            card::is_set_card(d, bc, &[SET_MIKANKO as u16])
                && Some(bc) == duel::get_attacker(d)
                && card::can_chain_attack(d, bc, Some(0), true)
        }
        None => false,
    }
}
/// `s.cacost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cacost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cacostfilter,tp,LOCATION_SZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cacostfilter), a.tp, LOCATION_SZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cacostfilter,tp,LOCATION_SZONE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&cacostfilter), a.tp, LOCATION_SZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.caop(e,tp,eg,ep,ev,re,r,rp)`
fn caop(d: &mut Duel, a: &EffArgs) {
    // local bc=Duel.GetAttacker()
    let bc = duel::get_attacker(d).nil();
    // if not bc:IsRelateToBattle() then return end
    if !card::is_relate_to_battle(d, bc) {
        return;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
    effect::set_code(d, e1, EFFECT_CANNOT_DIRECT_ATTACK);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_BATTLE|PHASE_DAMAGE_CAL)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE | PHASE_DAMAGE_CAL, 1);
    // bc:RegisterEffect(e1)
    card::register_effect(d, bc, e1, false);
    // Duel.ChainAttack()
    duel::chain_attack(d, None);
}
