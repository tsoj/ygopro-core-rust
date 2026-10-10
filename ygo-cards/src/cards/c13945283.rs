//! Wall of Illusion
use super::prelude::*;

pub const ID: u32 = 13945283;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // to hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_BATTLED);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e);
    duel::get_attack_target(d) == h && !card::is_status(d, duel::get_attacker(d).nil(), STATUS_BATTLE_DESTROYED)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let atk = duel::get_attacker(d);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, atk.map(CardSet::from_card), 1, 0, 0);
    true
}
fn op(d: &mut Duel, _a: &EffArgs) {
    let atk = duel::get_attacker(d).nil();
    if !card::is_relate_to_battle(d, atk) {
        return;
    }
    duel::send_to_hand(d, atk, None, REASON_EFFECT, None);
}
