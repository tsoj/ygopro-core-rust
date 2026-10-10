//! Mirage of Nightmare
use super::prelude::*;

pub const ID: u32 = 41482598;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(clear)));
    card::register_effect(d, c, e1, false);
    // draw
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_DRAW);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
    effect::set_condition(d, e2, Some(condition(drcon)));
    effect::set_target(d, e2, Some(cost_target(drtg)));
    effect::set_operation(d, e2, Some(operation(drop)));
    effect::set_label_object(d, e2, Value::Effect(e1));
    card::register_effect(d, c, e2, false);
    // discard
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_HANDES);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_code(d, e3, EVENT_PHASE | PHASE_STANDBY);
    effect::set_condition(d, e3, Some(condition(dccon)));
    effect::set_target(d, e3, Some(cost_target(dctg)));
    effect::set_operation(d, e3, Some(operation(dcop)));
    effect::set_label_object(d, e3, Value::Effect(e1));
    card::register_effect(d, c, e3, false);
}
fn clear(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    effect::set_label(d, a.e, &[0]);
    true
}
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp) && duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0) < 4
}
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let ht = duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 4 - ht);
    true
}
fn drop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    let ht = duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0);
    if ht < 4 {
        duel::draw(d, a.tp, (4 - ht) as u32, REASON_EFFECT);
        let lo = effect::get_label_object(d, a.e).as_effect().nil();
        effect::set_label(d, lo, &[(4 - ht) as i64]);
    } else {
        let lo = effect::get_label_object(d, a.e).as_effect().nil();
        effect::set_label(d, lo, &[0]);
    }
}
fn dccon(d: &mut Duel, a: &EffArgs) -> bool {
    if !duel::is_turn_player(d, a.tp) {
        return false;
    }
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    effect::get_label(d, lo) != 0
}
fn dctg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let de = effect::get_label_object(d, a.e).as_effect().nil();
    let l = effect::get_label(d, de);
    effect::set_label(d, a.e, &[l]);
    effect::set_label(d, de, &[0]);
    let label = effect::get_label(d, a.e);
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, a.tp, label as i32);
    true
}
fn dcop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    let g = duel::get_field_group(d, a.tp, LOCATION_HAND, 0);
    let label = effect::get_label(d, a.e);
    let sg = group::random_select(d, &g, a.tp, label as usize);
    duel::send_to_grave(d, sg, REASON_EFFECT | REASON_DISCARD, None, None);
}
