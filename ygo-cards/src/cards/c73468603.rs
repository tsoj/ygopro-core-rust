//! Set Rotation
use super::prelude::*;

pub const ID: u32 = 73468603;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SET);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_type(d, c, TYPE_FIELD) && card::is_s_setable(d, c, false, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let g = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_DECK, 0, NO_EX);
    if a.chk == Some(0) {
        return group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) > 1;
    }
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let mut g = duel::get_matching_group(d, Some(&filter), tp, LOCATION_DECK, 0, NO_EX);
    if group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) < 2 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 0));
    let mut tg1 = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
    let first = group::get_first(d, &tg1).nil();
    let codes = card::get_code(d, first);
    group::remove(d, &mut g, &|d, c| card::is_code(d, c, &codes), NO_EX);
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 1));
    let tg2 = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
    duel::s_set(d, tp, &tg1, None, true);
    duel::s_set(d, tp, &tg2, Some(1 - tp), true);
    let f1 = group::get_first(d, &tg1).nil();
    card::register_flag_effect(d, f1, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
    let f2 = group::get_first(d, &tg2).nil();
    card::register_flag_effect(d, f2, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
    group::add_card(&mut tg1, &tg2);
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e1, 1, 1);
    effect::set_condition(d, e1, Some(condition(con)));
    effect::set_value_fn(d, e1, lua_fn(actlimit));
    duel::register_effect(d, e1, tp);
    let e2 = effect::create_effect(d, h);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_CANNOT_SSET);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e2, 1, 1);
    effect::set_condition(d, e2, Some(condition(con)));
    effect::set_target(d, e2, Some(lua_fn(setlimit)));
    duel::register_effect(d, e2, tp);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_facedown(d, c) && card::get_flag_effect(d, c, ID) != 0
}
/// `s.con(e)`
fn con(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&cfilter), 0, LOCATION_FZONE, LOCATION_FZONE, 1, NO_EX)
}
/// `s.actlimit(e,re,tp)`: `re and re:IsActiveType(TYPE_FIELD) and re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:GetHandler():GetFlagEffect(id)==0`
fn actlimit(d: &mut Duel, a: Args) -> bool {
    let Some(re) = a.v(1).as_effect() else {
        return false;
    };
    if !effect::is_active_type(d, re, TYPE_FIELD) || !effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) {
        return false;
    }
    let h = effect::get_handler(d, re).nil();
    card::get_flag_effect(d, h, ID) == 0
}
/// `s.setlimit(e,c,tp)`: `c:IsType(TYPE_FIELD) and c:GetFlagEffect(id)==0`
fn setlimit(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    card::is_type(d, c, TYPE_FIELD) && card::get_flag_effect(d, c, ID) == 0
}
