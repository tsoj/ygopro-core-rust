//! Stygian Sergeants
use super::prelude::*;

pub const ID: u32 = 86137485;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Synchro.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_FIEND),1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_FIEND, sc, st, Some(tp)))), 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // chain attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
    // e1:SetCondition(s.atcon1)
    effect::set_condition(d, e1, Some(condition(atcon1)));
    // e1:SetOperation(s.atop1)
    effect::set_operation(d, e1, Some(operation(atop1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYING);
    // e2:SetCondition(s.atcon2)
    effect::set_condition(d, e2, Some(condition(atcon2)));
    // e2:SetOperation(s.atop2)
    effect::set_operation(d, e2, Some(operation(atop2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.atcon1(e,tp,eg,ep,ev,re,r,rp)`
fn atcon1(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    // return Duel.IsTurnPlayer(tp) and bc:IsLocation(LOCATION_GRAVE) and bc:IsMonster()
    //     and c:CanChainAttack() and c:IsStatus(STATUS_OPPO_BATTLE)
    duel::is_turn_player(d, tp) && {
        let bc = card::get_battle_target(d, c).nil();
        card::is_location(d, bc, LOCATION_GRAVE) && card::is_monster(d, bc)
    } && card::can_chain_attack(d, c, Some(2), false)
        && card::is_status(d, c, STATUS_OPPO_BATTLE)
}
/// `s.atop1(e,tp,eg,ep,ev,re,r,rp)`
fn atop1(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or c:IsFacedown() then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
        return;
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(800)
    effect::set_value(d, e1, 800);
    // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Duel.ChainAttack()
    duel::chain_attack(d, None);
}
/// `s.atcon2(e,tp,eg,ep,ev,re,r,rp)`
fn atcon2(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    // return Duel.IsTurnPlayer(1-tp) and c:IsRelateToBattle() and c:IsStatus(STATUS_OPPO_BATTLE)
    //     and bc:IsLocation(LOCATION_GRAVE) and bc:IsMonster()
    duel::is_turn_player(d, 1 - tp)
        && card::is_relate_to_battle(d, c)
        && card::is_status(d, c, STATUS_OPPO_BATTLE)
        && {
            let bc = card::get_battle_target(d, c).nil();
            card::is_location(d, bc, LOCATION_GRAVE) && card::is_monster(d, bc)
        }
}
/// `s.atop2(e,tp,eg,ep,ev,re,r,rp)`
fn atop2(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or c:IsFacedown() then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
        return;
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(800)
    effect::set_value(d, e1, 800);
    // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
