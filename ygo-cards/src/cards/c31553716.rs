//! Spear Dragon
use super::prelude::*;

pub const ID: u32 = 31553716;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // to defense
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_DAMAGE_STEP_END);
    effect::set_condition(d, e1, Some(condition(poscon)));
    effect::set_operation(d, e1, Some(operation(posop)));
    card::register_effect(d, c, e1, false);
    // pierce
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_PIERCE);
    card::register_effect(d, c, e2, false);
}
/// `s.poscon`
fn poscon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler()==Duel.GetAttacker() and e:GetHandler():IsRelateToBattle()
    let h = effect::get_handler(d, a.e);
    h == duel::get_attacker(d) && card::is_relate_to_battle(d, h.nil())
}
/// `s.posop`
fn posop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsAttackPos() then
    if card::is_attack_pos(d, c) {
        // Duel.ChangePosition(c,POS_FACEUP_DEFENSE)
        duel::change_position(d, c, POS_FACEUP_DEFENSE as u8, None, None, None, false);
    }
}
