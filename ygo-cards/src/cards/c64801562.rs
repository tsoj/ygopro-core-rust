//! Heart of Clear Water
use super::prelude::*;

pub const ID: u32 = 64801562;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // Indes
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_value(d, e2, 1);
    card::register_effect(d, c, e2, false);
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
    effect::set_value_fn(d, e3, lua_fn(indval));
    card::register_effect(d, c, e3, false);
    // selfdes
    let e5 = effect::create_effect(d, c);
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e5, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e5, LOCATION_SZONE);
    effect::set_code(d, e5, EFFECT_SELF_DESTROY);
    effect::set_condition(d, e5, Some(condition(descon)));
    card::register_effect(d, c, e5, false);
}
fn indval(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let re = a.e(1);
    let h = effect::get_handler(d, e).nil();
    let tc = card::get_equip_target(d, h);
    let rc = effect::get_handler(d, re).nil();
    let in_chain = effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) && {
        let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]).first().and_then(|v| v.as_group().cloned()).nil();
        tc.map_or(false, |tc| group::is_contains(&g, tc))
    };
    in_chain || (effect::is_has_type(d, re, EFFECT_TYPE_CONTINUOUS) && tc.map_or(false, |tc| card::is_has_card_target(d, rc, tc)))
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    match card::get_equip_target(d, h) {
        Some(tc) => card::get_attack(d, tc) >= 1300,
        None => false,
    }
}
