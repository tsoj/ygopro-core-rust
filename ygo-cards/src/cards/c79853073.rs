//! Cipher Soldier
use super::prelude::*;

pub const ID: u32 = 79853073;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atkdef up
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_TRIGGER_F + EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EVENT_PRE_DAMAGE_CALCULATE);
    effect::set_condition(d, e1, Some(condition(con)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn con(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    // return bc and bc:IsRace(RACE_WARRIOR)
    match card::get_battle_target(d, c) {
        Some(bc) => card::is_race(d, bc, RACE_WARRIOR),
        None => false,
    }
}
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE_CAL, 1);
        effect::set_value(d, e1, 2000);
        card::register_effect(d, c, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
        card::register_effect(d, c, e2, false);
    }
}
