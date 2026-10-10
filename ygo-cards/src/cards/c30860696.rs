//! Rocket Warrior
use super::prelude::*;

pub const ID: u32 = 30860696;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // invincible
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_condition(d, e1, Some(condition(ivcon)));
    effect::set_value(d, e1, 1);
    card::register_effect(d, c, e1, false);
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_AVOID_BATTLE_DAMAGE);
    effect::set_condition(d, e2, Some(condition(ivcon)));
    effect::set_value(d, e2, 1);
    card::register_effect(d, c, e2, false);
    // reduce atk
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_ATKCHANGE);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e3, EVENT_BATTLED);
    effect::set_condition(d, e3, Some(condition(racon)));
    effect::set_operation(d, e3, Some(operation(raop)));
    card::register_effect(d, c, e3, false);
}
/// `s.ivcon(e)`
fn ivcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetTurnPlayer()==e:GetHandlerPlayer()
    duel::get_turn_player(d) == effect::get_handler_player(d, a.e)
}
/// `s.racon(e,tp,eg,ep,ev,re,r,rp)`
fn racon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler()==Duel.GetAttacker() and Duel.GetAttackTarget()
    let h = effect::get_handler(d, a.e);
    h == duel::get_attacker(d) && duel::get_attack_target(d).is_some()
}
/// `s.raop(e,tp,eg,ep,ev,re,r,rp)`
fn raop(d: &mut Duel, a: &EffArgs) {
    // local d=Duel.GetAttackTarget()
    let tg = duel::get_attack_target(d).nil();
    // if not d:IsRelateToBattle() or d:IsFacedown() then return end
    if !card::is_relate_to_battle(d, tg) || card::is_facedown(d, tg) {
        return;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(-500)
    effect::set_value(d, e1, -500);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // d:RegisterEffect(e1)
    card::register_effect(d, tg, e1, false);
}
