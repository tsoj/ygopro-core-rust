//! Exploder Dragonwing
use super::prelude::*;

pub const ID: u32 = 40529384;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro summon
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTunerEx(Card.IsRace,RACE_DRAGON),1,99)
    synchro::add_procedure(
        d,
        c,
        None,
        1,
        1,
        Some(synchro::non_tuner_ex(|d, x, sc, st, tp| card::is_race_ctx(d, x, RACE_DRAGON, sc, st, Some(tp)))),
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
    // destroy
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e1, EVENT_BATTLE_START);
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// The battle target of this card: `tc=Duel.GetAttacker(); if tc==c then tc=Duel.GetAttackTarget() end`
fn battle_target(d: &mut Duel, c: CardId) -> Option<CardId> {
    let tc = duel::get_attacker(d);
    if tc == Some(c) {
        duel::get_attack_target(d)
    } else {
        tc
    }
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=Duel.GetAttacker()
    // if tc==c then tc=Duel.GetAttackTarget() end
    let tc = battle_target(d, c);
    // if chk==0 then return tc and tc:IsFaceup() and tc:GetAttack()<=c:GetAttack() end
    if a.chk == Some(0) {
        let Some(tc) = tc else {
            return false;
        };
        return card::is_faceup(d, tc) && card::get_attack(d, tc) <= card::get_attack(d, c);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, tc.map(CardSet::from_card), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,tc:GetAttack())
    let atk = card::get_attack(d, tc.nil());
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, atk);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=Duel.GetAttacker()
    // if tc==c then tc=Duel.GetAttackTarget() end
    let tc = battle_target(d, c).nil();
    // if tc:IsRelateToBattle() and tc:GetAttack()<=c:GetAttack() then
    if card::is_relate_to_battle(d, tc) && card::get_attack(d, tc) <= card::get_attack(d, c) {
        // local atk=tc:GetAttack()
        let atk = card::get_attack(d, tc);
        // if Duel.Destroy(tc,REASON_EFFECT)~=0 then
        if duel::destroy(d, tc, REASON_EFFECT, None, None) != 0 {
            // Duel.Damage(1-tp,atk,REASON_EFFECT)
            duel::damage(d, 1 - tp, atk as i64, REASON_EFFECT, false, None);
        }
    }
}
