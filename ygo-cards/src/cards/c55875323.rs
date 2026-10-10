//! Electric Lizard
use super::prelude::*;

pub const ID: u32 = 55875323;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // A non-zombie monster that attacked this card cannot attack next turn
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_BATTLED);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler()==Duel.GetAttackTarget() and not Duel.GetAttacker():IsRace(RACE_ZOMBIE)
    let h = effect::get_handler(d, a.e).nil();
    if Some(h) != duel::get_attack_target(d) {
        return false;
    }
    let attacker = duel::get_attacker(d).nil();
    !card::is_race(d, attacker, RACE_ZOMBIE)
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetAttacker()
    let tc = duel::get_attacker(d).nil();
    // if tc:IsRelateToBattle() then
    if card::is_relate_to_battle(d, tc) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3206)
        effect::set_description(d, e1, 3206);
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CANNOT_ATTACK)
        effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
        // e1:SetLabel(Duel.GetTurnCount())
        let turn = duel::get_turn_count(d, None);
        effect::set_label(d, e1, &[turn as i64]);
        // e1:SetCondition(s.atkcon)
        effect::set_condition(d, e1, Some(lua_fn(atkcon)));
        // e1:SetOwnerPlayer(tp)
        effect::set_owner_player(d, e1, a.tp);
        // e1:SetReset(RESETS_STANDARD_PHASE_END|RESET_OPPO_TURN,2)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END | RESET_OPPO_TURN, 2);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
/// `s.atkcon(e)`
fn atkcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // return Duel.GetTurnCount()~=e:GetLabel() and Duel.GetTurnPlayer()~=e:GetOwnerPlayer()
    let turn = duel::get_turn_count(d, None) as i64;
    if turn == effect::get_label(d, e) {
        return false;
    }
    duel::get_turn_player(d) != effect::get_owner_player(d, e)
}
