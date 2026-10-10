//! Curse of Darkness
use super::prelude::*;

pub const ID: u32 = 84970821;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // damage
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS | EFFECT_TYPE_FIELD);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e2, EVENT_CHAINING);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_operation(d, e2, Some(aux::chainreg()));
    card::register_effect(d, c, e2, false);
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_CONTINUOUS | EFFECT_TYPE_FIELD);
    effect::set_code(d, e3, EVENT_CHAIN_SOLVED);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_operation(d, e3, Some(operation(damop)));
    card::register_effect(d, c, e3, false);
}
fn damop(d: &mut Duel, a: &EffArgs) {
    let re = a.re.nil();
    let _c = effect::get_handler(d, re);
    let is_activate = effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE);
    if is_activate && effect::is_spell_effect(d, re) {
        let h = effect::get_handler(d, a.e).nil();
        if card::get_flag_effect(d, h, 1) > 0 {
            duel::damage(d, a.rp, 1000, REASON_EFFECT, false, None);
        }
    }
}
