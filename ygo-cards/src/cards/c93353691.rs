//! Driven Daredevil
use super::prelude::*;

pub const ID: u32 = 93353691;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro summon
    // Synchro.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_WARRIOR),1,1,Synchro.NonTunerEx(Card.IsRace,RACE_WARRIOR),1,99)
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_WARRIOR, sc, st, Some(tp)))),
        1,
        1,
        Some(synchro::non_tuner_ex(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_WARRIOR, sc, st, Some(tp)))),
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
    // indes
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetValue(s.indes)
    effect::set_value_fn(d, e1, effect_card_fn(indes));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_BATTLED)
    effect::set_code(d, e2, EVENT_BATTLED);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.indes(e,c)
fn indes(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // local ec=e:GetHandler()
    let ec = effect::get_handler(d, e).nil();
    // return ec==Duel.GetAttackTarget() and c:GetAttack()>ec:GetAttack()
    duel::get_attack_target(d) == Some(ec) && card::get_attack(d, c) > card::get_attack(d, ec)
}
// function s.descon(e,tp,eg,ep,ev,re,r,rp)
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return ev==1 and Duel.GetAttacker():IsRelateToBattle()
    //     and Duel.GetAttacker():GetAttack()>e:GetHandler():GetAttack()
    if a.ev != 1 {
        return false;
    }
    let atk = duel::get_attacker(d).nil();
    if !card::is_relate_to_battle(d, atk) {
        return false;
    }
    let atk = duel::get_attacker(d).nil();
    let h = effect::get_handler(d, a.e).nil();
    card::get_attack(d, atk) > card::get_attack(d, h)
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,Duel.GetAttacker(),1,0,0)
    let atk = duel::get_attacker(d);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, atk.map(CardSet::from_card), 1, 0, 0);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, _a: &EffArgs) {
    // local a=Duel.GetAttacker()
    let a = duel::get_attacker(d).nil();
    // if a:IsRelateToBattle() then
    if card::is_relate_to_battle(d, a) {
        // Duel.Destroy(a,REASON_EFFECT)
        duel::destroy(d, CardSet::from_card(a), REASON_EFFECT, None, None);
    }
}
