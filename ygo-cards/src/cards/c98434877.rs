//! Suijin
use super::prelude::*;

pub const ID: u32 = 98434877;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atkdown
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EVENT_PRE_DAMAGE_CALCULATE);
    effect::set_property(d, e1, EFFECT_FLAG_NO_TURN_RESET, 0);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp) and Duel.GetAttackTarget()==e:GetHandler()
    duel::is_turn_player(d, 1 - a.tp) && duel::get_attack_target(d) == effect::get_handler(d, a.e)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetAttacker():IsCanBeEffectTarget(e) end
    if a.chk == Some(0) {
        let att = duel::get_attacker(d).nil();
        return card::is_can_be_effect_target(d, att, Some(a.e));
    }
    // Duel.SetTargetCard(Duel.GetAttacker())
    let att = duel::get_attacker(d).nil();
    duel::set_target_card(d, att);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE_CAL, 1);
        effect::set_value(d, e1, 0);
        card::register_effect(d, tc, e1, false);
    }
}
