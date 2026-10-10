//! Crimson Nova Trinity the Dark Cubic Lord
use super::prelude::*;

pub const ID: u32 = 72664875;

pub const INFO: ScriptInfo = ScriptInfo { material_setcode: &[SET_CUBIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion summon procedure
    card::enable_revive_limit(d, c);
    fusion::add_proc_mix_n(d, c, false, false, vec![(fusion::Mat::Code(30270176), 3)]);
    // Special Summon condition
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e0, EFFECT_SPSUMMON_CONDITION);
    effect::set_value_fn(d, e0, aux::fuslimit());
    card::register_effect(d, c, e0, false);
    // Prevent effect target
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_value_fn(d, e1, aux::tgoval());
    card::register_effect(d, c, e1, false);
    // Prevent destruction by opponent's effect
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    effect::set_value_fn(d, e2, aux::indoval());
    card::register_effect(d, c, e2, false);
    // Halve LP
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e3, EVENT_ATTACK_ANNOUNCE);
    effect::set_operation(d, e3, Some(operation(hvop)));
    card::register_effect(d, c, e3, false);
    // Extra Attack
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e4, EVENT_BATTLE_DESTROYING);
    effect::set_condition(d, e4, Some(aux::bdcon()));
    effect::set_target(d, e4, Some(cost_target(atktg)));
    effect::set_operation(d, e4, Some(operation(atkop)));
    card::register_effect(d, c, e4, false);
    // Inflict damage
    let e5 = effect::create_effect(d, c);
    effect::set_category(d, e5, CATEGORY_DAMAGE);
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e5, EVENT_DAMAGE);
    effect::set_range(d, e5, LOCATION_MZONE);
    effect::set_property(d, e5, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_condition(d, e5, Some(condition(damcon)));
    effect::set_target(d, e5, Some(cost_target(damtg)));
    effect::set_operation(d, e5, Some(operation(damop)));
    card::register_effect(d, c, e5, false);
}
fn hvop(d: &mut Duel, a: &EffArgs) {
    let lp = duel::get_lp(d, 1 - a.tp);
    let half = (lp as f64 / 2.0).ceil() as i32;
    duel::set_lp(d, 1 - a.tp, half);
}
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return card::is_relate_to_battle(d, c) && card::is_has_effect(d, c, EFFECT_EXTRA_ATTACK, None).is_empty();
    }
    true
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_battle(d, c) {
        return;
    }
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
    effect::set_value(d, e1, 1);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE, 1);
    card::register_effect(d, c, e1, false);
}
fn damcon(_d: &mut Duel, a: &EffArgs) -> bool {
    (a.r & REASON_EFFECT) != 0 && a.ep == a.tp
}
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_target_player(d, 1 - a.tp);
    duel::set_target_param(d, a.ev);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, a.ev as i32);
    true
}
fn damop(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
