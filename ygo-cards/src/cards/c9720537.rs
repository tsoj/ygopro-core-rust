//! Owner's Seal
use super::prelude::*;

pub const ID: u32 = 9720537;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_CONTROL);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetControler()~=c:GetOwner()
    card::get_controler(d, c) != card::get_owner(d, c)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_field_group(d, a.tp, LOCATION_MZONE, LOCATION_MZONE);
    // aux.Next(g) starts with g:GetFirst()
    let _ = group::get_first(d, &g);
    for tc in group::iter(d, &g) {
        if !card::is_immune_to_effect(d, tc, a.e) {
            card::reset_effect(d, tc, EFFECT_SET_CONTROL, RESET_CODE);
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_SET_CONTROL);
            effect::set_value(d, e1, card::get_owner(d, tc) as i64);
            effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !(RESET_TOFIELD | RESET_TEMP_REMOVE | RESET_TURN_SET)), 1);
            card::register_effect(d, tc, e1, false);
        }
    }
}
