//! Legendary Jujitsu Master
use super::prelude::*;

pub const ID: u32 = 25773409;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // to deck
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TODECK);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_DAMAGE_STEP_END);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_attack_target(d) == effect::get_handler(d, a.e) && (card::get_battle_position(d, effect::get_handler(d, a.e).nil()) & POS_DEFENSE) != 0
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_TODECK, duel::get_attacker(d).map(CardSet::from_card), 1, 0, 0);
    true
}
fn op(d: &mut Duel, _a: &EffArgs) {
    let a_card = duel::get_attacker(d).nil();
    if !card::is_relate_to_battle(d, a_card) {
        return;
    }
    duel::send_to_deck(d, a_card, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
}
