//! Mist Valley Falcon
use super::prelude::*;

pub const ID: u32 = 82199284;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // attack cost
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_ATTACK_COST);
    effect::set_cost(d, e1, Some(lua_fn(atcost)));
    effect::set_operation(d, e1, Some(operation(atop)));
    card::register_effect(d, c, e1, false);
}
fn atcost(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(2);
    let e = a.e(0);
    let c = effect::get_handler(d, e).nil();
    duel::is_existing_matching_card(d, Some(&card::is_able_to_hand_as_cost), tp, LOCATION_ONFIELD, 0, 1, c.into())
}
fn atop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if duel::is_attack_cost_paid(d) != 2 && card::is_location(d, c, LOCATION_MZONE) {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
        let paid = duel::is_attack_cost_paid(d);
        let g = duel::get_matching_group(d, Some(&card::is_able_to_hand_as_cost), a.tp, LOCATION_ONFIELD, 0, c.into());
        let tc = group::select_unselect(d, &g, Some(&CardSet::new()), a.tp, paid == 0, paid == 0, None, None);
        if let Some(tc) = tc {
            duel::send_to_hand(d, tc, None, REASON_COST, None);
            duel::attack_cost_paid(d, None);
        } else {
            duel::attack_cost_paid(d, Some(2));
        }
    }
}
