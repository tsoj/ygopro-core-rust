//! Gachi Gachi Gantetsu
use super::prelude::*;

pub const ID: u32 = 10002346;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Xyz Summon Procedure
    xyz::add_procedure(d, c, None, Some(2), 2, None, None, None, None, false, None);
    // Destruction replacement
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e1, EFFECT_DESTROY_REPLACE);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target(d, e1, Some(cost_target(reptg)));
    effect::set_operation(d, e1, Some(operation(repop)));
    card::register_effect(d, c, e1, false);
    // Monsters you control gain 200 ATK/DEF for each material attached to this card
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    effect::set_value_fn(d, e2, effect_card_value(val));
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    card::register_effect(d, c, e3, false);
}
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return !card::is_reason(d, c, REASON_REPLACE) && card::check_remove_overlay_card(d, c, a.tp, 1, REASON_EFFECT);
    }
    duel::select_effect_yes_no(d, a.tp, c, Some(96))
}
fn repop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    card::remove_overlay_card(d, c, a.tp, 1, 1, REASON_EFFECT);
}
fn val(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    let h = effect::get_handler(d, e).nil();
    card::get_overlay_count(d, h) as i64 * 200
}
