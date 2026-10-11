//! Gravity Behemoth
use super::prelude::*;

pub const ID: u32 = 56931015;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Normal summon it with no tributes
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_SUMMON_PROC);
    effect::set_condition(d, e1, Some(lua_fn(ntcon)));
    card::register_effect(d, c, e1, false);
    // Destroy all field spells on the field
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_category(d, e2, CATEGORY_DESTROY);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_cost(d, e2, Some(cost_target(descost)));
    effect::set_target(d, e2, Some(cost_target(destg)));
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
}
/// `function s.ntcon(e,c,minc)`
fn ntcon(d: &mut Duel, a: Args) -> bool {
    let Some(c) = a.card(1) else {
        return duel::get_turn_count(d, None) != 1;
    };
    if a.int(2) != 0 || card::get_level(d, c) <= 4 {
        return false;
    }
    let tp = card::get_controler(d, c);
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::get_field_group_count(d, tp, LOCATION_ONFIELD, LOCATION_ONFIELD) == 0
}
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::get_attack_announced_count(d, h) == 0;
    }
    // Cannot attack this turn
    let e1 = effect::create_effect(d, h);
    effect::set_description(d, e1, 3206);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    card::register_effect(d, h, e1, false);
    true
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let g = duel::get_matching_group(d, None, 0, LOCATION_FZONE, LOCATION_FZONE, NO_EX);
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), g.len() as u8, 0, 0);
    true
}
fn desop(d: &mut Duel, _a: &EffArgs) {
    let g = duel::get_matching_group(d, None, 0, LOCATION_FZONE, LOCATION_FZONE, NO_EX);
    if !g.is_empty() {
        duel::destroy(d, &g, REASON_EFFECT, None, None);
    }
}
