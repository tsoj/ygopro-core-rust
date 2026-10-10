//! Revived King Ha Des
use super::prelude::*;

pub const ID: u32 = 5309481;

pub const INFO: ScriptInfo = ScriptInfo { material: &[33420078], listed_names: &[33420078], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro summon
    // Synchro.AddProcedure(c,aux.FilterSummonCode(33420078),1,1,Synchro.NonTunerEx(Card.IsRace,RACE_ZOMBIE),1,99)
    synchro::add_procedure(
        d,
        c,
        Some(aux::filter_summon_code(&[33420078])),
        1,
        1,
        Some(synchro::non_tuner_ex(|d, x, sc, st, tp| card::is_race_ctx(d, x, RACE_ZOMBIE, sc, st, Some(tp)))),
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
    // Disable
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_BATTLED)
    effect::set_code(d, e1, EVENT_BATTLED);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.operation(e,tp,...)` (EVENT_BATTLED)
fn op(d: &mut Duel, a: &EffArgs) {
    // local a=Duel.GetAttacker()
    let att = duel::get_attacker(d);
    // local d=Duel.GetAttackTarget()
    let dt = duel::get_attack_target(d);
    // local p=e:GetHandler():GetControler()
    let h = effect::get_handler(d, a.e).nil();
    let p = card::get_controler(d, h);
    // if d==nil then return end
    let Some(dt) = dt else { return };
    let att = att.nil();
    // local tc=nil
    let mut tc = None;
    // if a:GetControler()==p and a:IsRace(RACE_ZOMBIE) and d:IsStatus(STATUS_BATTLE_DESTROYED) then tc=d
    if card::get_controler(d, att) == p && card::is_race(d, att, RACE_ZOMBIE) && card::is_status(d, dt, STATUS_BATTLE_DESTROYED) {
        tc = Some(dt);
    // elseif d:GetControler()==p and d:IsRace(RACE_ZOMBIE) and a:IsStatus(STATUS_BATTLE_DESTROYED) then tc=a end
    } else if card::get_controler(d, dt) == p && card::is_race(d, dt, RACE_ZOMBIE) && card::is_status(d, att, STATUS_BATTLE_DESTROYED) {
        tc = Some(att);
    }
    // if not tc then return end
    let Some(tc) = tc else { return };
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_DISABLE)
    effect::set_code(d, e1, EFFECT_DISABLE);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD&~(RESET_LEAVE|RESET_TOGRAVE))
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !(RESET_LEAVE | RESET_TOGRAVE)), 1);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
    // local e2=Effect.CreateEffect(e:GetHandler())
    let e2 = effect::create_effect(d, h);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_DISABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
    // e2:SetReset(RESET_EVENT|RESETS_STANDARD&~(RESET_LEAVE|RESET_TOGRAVE))
    effect::set_reset(d, e2, RESET_EVENT | (RESETS_STANDARD & !(RESET_LEAVE | RESET_TOGRAVE)), 1);
    // tc:RegisterEffect(e2)
    card::register_effect(d, tc, e2, false);
}
