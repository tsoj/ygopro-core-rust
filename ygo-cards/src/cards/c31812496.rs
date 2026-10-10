//! Stone Statue of the Aztecs
use super::prelude::*;

pub const ID: u32 = 31812496;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Double damage
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_CHANGE_BATTLE_DAMAGE);
    effect::set_condition(d, e1, Some(lua_fn(dcon)));
    effect::set_value_fn(d, e1, aux::change_battle_damage(1, DOUBLE_DAMAGE as i64));
    card::register_effect(d, c, e1, false);
}
fn dcon(d: &mut Duel, a: Args) -> bool {
    duel::get_attack_target(d) == Some(effect::get_handler(d, a.e(0)).nil())
}
