//! Cat's Ear Tribe
use super::prelude::*;

pub const ID: u32 = 95841282;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SET_BASE_ATTACK)
    effect::set_code(d, e1, EFFECT_SET_BASE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    // e1:SetTarget(s.atktg)
    effect::set_target(d, e1, Some(effect_card_fn(atktg)));
    // e1:SetCondition(s.atkcon)
    effect::set_condition(d, e1, Some(condition(atkcon)));
    // e1:SetValue(200)
    effect::set_value(d, e1, 200);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.atkcon(e)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local ph=Duel.GetCurrentPhase()
    let ph = duel::get_current_phase(d);
    // return (ph==PHASE_DAMAGE or ph==PHASE_DAMAGE_CAL) and Duel.GetTurnPlayer()~=e:GetHandlerPlayer()
    //     and Duel.GetAttackTarget()==e:GetHandler()
    (ph == PHASE_DAMAGE || ph == PHASE_DAMAGE_CAL)
        && duel::get_turn_player(d) != effect::get_handler_player(d, a.e)
        && duel::get_attack_target(d) == Some(effect::get_handler(d, a.e).nil())
}
/// `s.atktg(e,c)`
fn atktg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c==e:GetHandler():GetBattleTarget()
    card::get_battle_target(d, effect::get_handler(d, e).nil()) == Some(c)
}
