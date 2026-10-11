//! Air Cracking Storm
use super::prelude::*;

pub const ID: u32 = 98864751;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    aux::add_equip_procedure(d, c, None, Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_race(d, c, RACE_MACHINE))), None, None, None, None, None, None);
    // extra attack
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e3, EVENT_BATTLE_DESTROYING);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_condition(d, e3, Some(condition(drcon)));
    effect::set_cost(d, e3, Some(cost_target(cost_fn)));
    effect::set_operation(d, e3, Some(operation(drop_op)));
    card::register_effect(d, c, e3, false);
    aux::global_check(d, ID, move |d| {
        set_s_var(d, ID, 0, Value::Int(0));
        set_s_var(d, ID, 1, Value::Int(0));
        let ge1 = effect::create_effect(d, c);
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, ge1, EVENT_ATTACK_ANNOUNCE);
        effect::set_operation(d, ge1, Some(operation(checkop)));
        duel::register_effect(d, ge1, 0);
        aux::add_values_reset(d, reset_s);
    });
}
fn reset_s(d: &mut Duel) {
    set_s_var(d, ID, 0, Value::Int(0));
    set_s_var(d, ID, 1, Value::Int(0));
}
fn checkop(d: &mut Duel, a: &EffArgs) {
    let tc = group::get_first(d, &a.eg.clone().nil()).nil();
    if card::get_flag_effect(d, tc, ID) == 0 {
        let ep = a.ep as u64;
        let cur = s_var(d, ID, ep).int();
        set_s_var(d, ID, ep, Value::Int(cur + 1));
        card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    }
}
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    let ec = group::get_first(d, &a.eg.clone().nil());
    let h = effect::get_handler(d, a.e).nil();
    if card::get_equip_target(d, h) != ec {
        return false;
    }
    let bt = card::get_battle_target(d, ec.nil()).nil();
    card::is_reason(d, bt, REASON_BATTLE)
}
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        if s_var(d, ID, a.tp as u64).int() == 0 {
            return true;
        }
        let eqt = card::get_equip_target(d, h).nil();
        return card::get_flag_effect(d, eqt, ID) != 0;
    }
    let eqt = card::get_equip_target(d, h).nil();
    let fid = card::get_field_id(d, eqt) as i64;
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
    effect::set_property(d, e1, EFFECT_FLAG_OATH + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    effect::set_target(d, e1, Some(effect_card_fn(ftarget)));
    effect::set_label(d, e1, &[fid]);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
    true
}
fn ftarget(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    effect::get_label(d, e) != card::get_field_id(d, c) as i64
}
fn drop_op(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
    effect::set_value(d, e1, 1);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE, 1);
    let eqt = card::get_equip_target(d, h).nil();
    card::register_effect(d, eqt, e1, false);
}
