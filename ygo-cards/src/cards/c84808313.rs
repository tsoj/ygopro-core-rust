//! Big Evolution Pill
use super::prelude::*;

pub const ID: u32 = 84808313;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    card::register_effect(d, c, e1, false);
    // reduce tribute
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_SUMMON_PROC);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target_range(d, e2, LOCATION_HAND, 0);
    effect::set_condition(d, e2, Some(lua_fn(ntcon)));
    effect::set_target(d, e2, Some(aux::field_summon_proc_tg(Some(effect_card_fn(|d, _e, c| card::is_race(d, c, RACE_DINOSAUR))), None)));
    card::register_effect(d, c, e2, false);
    // remain field
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e3, EFFECT_REMAIN_FIELD);
    card::register_effect(d, c, e3, false);
}
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&|d, c| card::is_race(d, c, RACE_DINOSAUR)), 1, 1, false, None, NO_EX);
    }
    let g = aux::select_release_group_cost(d, tp, Some(&|d, c| card::is_race(d, c, RACE_DINOSAUR)), 1, 1, false, None, NO_EX);
    duel::release(d, g, REASON_COST, None);
    true
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    card::set_turn_counter(d, c, 0);
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_condition(d, e1, Some(condition(descon)));
    effect::set_operation(d, e1, Some(operation(desop)));
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, c, e1, false);
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_SET_AVAILABLE, 0);
    effect::set_code(d, e3, 1082946);
    effect::set_label_object(d, e3, Value::Effect(e1));
    effect::set_owner_player(d, e3, tp);
    effect::set_operation(d, e3, Some(operation(reset)));
    effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, c, e3, false);
    true
}
fn reset(d: &mut Duel, a: &EffArgs) {
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    let mut b = a.clone();
    b.e = lo;
    b.re = Some(a.e);
    desop(d, &b);
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let ct = card::get_turn_counter(d, c) + 1;
    card::set_turn_counter(d, c, ct as u16);
    if ct == 3 {
        duel::destroy(d, c, REASON_RULE, None, None);
        if let Some(re) = a.re {
            effect::reset(d, re);
        }
    }
}
/// `function(e,c,minc)`
fn ntcon(d: &mut Duel, a: Args) -> bool {
    let Some(c) = a.card(1) else {
        return true;
    };
    let minc = a.int(2);
    let p = card::get_controler(d, c);
    minc == 0 && duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0 > 0
}
