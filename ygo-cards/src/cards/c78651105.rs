//! Beast King Barbaros
use super::prelude::*;

pub const ID: u32 = 78651105;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon & set with no tribute
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_SUMMON_PROC);
    effect::set_condition(d, e1, Some(lua_fn(ntcon)));
    effect::set_operation(d, e1, Some(lua_fn(ntop)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_SET_PROC);
    card::register_effect(d, c, e2, false);
    // summon with 3 tribute
    let _e3 = aux::add_normal_summon_procedure(d, c, true, true, Some(3), Some(3), Some(SUMMON_TYPE_TRIBUTE + 1), Some(aux::stringid(ID, 1)), None, None);
    // destroy
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 2));
    effect::set_category(d, e4, CATEGORY_DESTROY);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e4, EVENT_SUMMON_SUCCESS);
    effect::set_condition(d, e4, Some(condition(descon)));
    effect::set_target(d, e4, Some(cost_target(destg)));
    effect::set_operation(d, e4, Some(operation(desop)));
    card::register_effect(d, c, e4, false);
}
/// `s.ntcon(e,c,minc)`
fn ntcon(d: &mut Duel, a: Args) -> bool {
    let Some(c) = a.card(1) else { return true };
    let minc = a.int(2);
    if minc != 0 {
        return false;
    }
    if card::get_level(d, c) <= 4 {
        return false;
    }
    let tp = card::get_controler(d, c);
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
}
/// `s.ntop(e,tp,eg,ep,ev,re,r,rp,c)`
fn ntop(d: &mut Duel, a: Args) {
    let c = a.c(8);
    // change base attack
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_DISABLE & !RESET_TOFIELD), 1);
    effect::set_code(d, e1, EFFECT_SET_BASE_ATTACK);
    effect::set_value(d, e1, 1900);
    card::register_effect(d, c, e1, false);
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    card::get_summon_type(d, effect::get_handler(d, a.e).nil()) == SUMMON_TYPE_TRIBUTE + 1
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let sg = duel::get_matching_group(d, None, a.tp, 0, LOCATION_ONFIELD, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(sg.clone()), sg.len() as u8, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let sg = duel::get_matching_group(d, None, a.tp, 0, LOCATION_ONFIELD, NO_EX);
    duel::destroy(d, sg, REASON_EFFECT, None, None);
}
