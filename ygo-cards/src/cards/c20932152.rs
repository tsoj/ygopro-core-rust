//! Quickdraw Synchron
use super::prelude::*;

pub const ID: u32 = 20932152;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    effect::set_operation(d, e1, Some(lua_fn(spop)));
    card::register_effect(d, c, e1, false);
    //
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_CANNOT_BE_SYNCHRO_MATERIAL);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_value_fn(d, e2, lua_fn(synlimit));
    card::register_effect(d, c, e2, false);
    //
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e3, ID);
    card::register_effect(d, c, e3, false);
}
fn synlimit(d: &mut Duel, a: Args) -> bool {
    let c = match a.card(1) {
        None => return false,
        Some(c) => c,
    };
    !card::lists_archetype_as_material(d, c, &[SET_SYNCHRON as u32])
}
fn spfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_monster(d, c) && card::is_able_to_grave_as_cost(d, c)
}
fn spcon(d: &mut Duel, a: Args) -> bool {
    let c = match a.card(1) {
        None => return true,
        Some(c) => c,
    };
    let tp = card::get_controler(d, c);
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&spfilter), tp, LOCATION_HAND, 0, 1, c.into())
}
fn spop(d: &mut Duel, a: Args) {
    let tp = a.p(1);
    let ex = match a.card(8) {
        Some(c) => c.into(),
        None => NO_EX,
    };
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&spfilter), tp, LOCATION_HAND, 0, 1, 1, false, ex).nil();
    duel::send_to_grave(d, g, REASON_COST, None, None);
}
