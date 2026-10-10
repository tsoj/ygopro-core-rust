//! Tainted Wisdom
use super::prelude::*;

pub const ID: u32 = 28725004;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // shuffle
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_CHANGE_POS);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    card::is_previous_position(d, effect::get_handler(d, a.e).nil(), POS_ATTACK)
        && card::is_position(d, effect::get_handler(d, a.e).nil(), POS_DEFENSE)
}
fn op(d: &mut Duel, a: &EffArgs) {
    duel::shuffle_deck(d, a.tp);
}
