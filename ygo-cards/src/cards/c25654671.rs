//! Naturia Spiderfang
use super::prelude::*;

pub const ID: u32 = 25654671;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atk limit
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    effect::set_condition(d, e1, Some(condition(atkcon)));
    card::register_effect(d, c, e1, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_CHAIN,aux.FALSE)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_CHAIN, lua_fn(|_d, _a: Args| false));
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetCustomActivityCount(id,1-e:GetHandlerPlayer(),ACTIVITY_CHAIN)==0
    let p = 1 - effect::get_handler_player(d, a.e);
    duel::get_custom_activity_count(d, ID, p, ACTIVITY_CHAIN) == 0
}
