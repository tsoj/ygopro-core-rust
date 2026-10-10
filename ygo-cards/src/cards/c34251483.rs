//! Goblin Marauding Squad
use super::prelude::*;

pub const ID: u32 = 34251483;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    effect::set_operation(d, e1, Some(operation(atkop)));
    card::register_effect(d, c, e1, false);
    // to defense
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_PHASE | PHASE_BATTLE);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(poscon)));
    effect::set_operation(d, e2, Some(operation(posop)));
    card::register_effect(d, c, e2, false);
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    effect::set_target_range(d, e1, 0, 1);
    effect::set_value_fn(d, e1, lua_fn(aclimit));
    effect::set_reset(d, e1, RESET_CHAIN, 1);
    duel::register_effect(d, e1, a.tp);
}
/// `function(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    // re:IsHasType(EFFECT_TYPE_ACTIVATE) or re:GetHandler():IsMonster()
    if effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) {
        return true;
    }
    let h = effect::get_handler(d, re).nil();
    card::is_monster(d, h)
}
fn poscon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetAttackedCount()>0
    let h = effect::get_handler(d, a.e).nil();
    card::get_attacked_count(d, h) > 0
}
fn posop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsAttackPos() then
    if card::is_attack_pos(d, c) {
        // Duel.ChangePosition(c,POS_FACEUP_DEFENSE)
        duel::change_position(d, c, POS_FACEUP_DEFENSE as u8, None, None, None, false);
    }
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_CANNOT_CHANGE_POSITION);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_COPY_INHERIT, 0);
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 3);
    card::register_effect(d, c, e1, false);
}
