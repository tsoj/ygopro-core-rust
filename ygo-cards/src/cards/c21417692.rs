//! Dark Elf
use super::prelude::*;

pub const ID: u32 = 21417692;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // attack cost
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_ATTACK_COST);
    effect::set_cost(d, e1, Some(lua_fn(atcost)));
    effect::set_operation(d, e1, Some(operation(atop)));
    card::register_effect(d, c, e1, false);
}
/// `function(e,c,tp)`
fn atcost(d: &mut Duel, a: Args) -> bool {
    duel::check_lp_cost(d, a.p(2), 1000)
}
fn atop(d: &mut Duel, a: &EffArgs) {
    if duel::is_attack_cost_paid(d) != 2 && card::is_location(d, effect::get_handler(d, a.e).nil(), LOCATION_MZONE) {
        duel::pay_lp_cost(d, a.tp, 1000);
        duel::attack_cost_paid(d, None);
    }
}
