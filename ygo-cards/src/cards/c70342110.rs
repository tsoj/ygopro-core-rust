//! Dimensional Prison
use super::prelude::*;

pub const ID: u32 = 70342110;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_REMOVE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tg = duel::get_attacker(d);
    if let Some(chkc) = a.chkc() {
        return Some(chkc) == tg;
    }
    if a.chk == Some(0) {
        let tg = tg.nil();
        return card::is_on_field(d, tg) && card::is_able_to_remove(d, tg, None, None, None) && card::is_can_be_effect_target(d, tg, Some(a.e));
    }
    duel::set_target_card(d, tg.nil());
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(tg.nil())), 1, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && card::can_attack(d, tc) && !card::is_status(d, tc, STATUS_ATTACK_CANCELED) {
        duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
