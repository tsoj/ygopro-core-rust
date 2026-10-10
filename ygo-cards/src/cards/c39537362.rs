//! Ordeal of a Traveler
use super::prelude::*;

pub const ID: u32 = 39537362;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Return an attacking monster to the hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD | EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_condition(d, e2, Some(condition(cond)));
    effect::set_target(d, e2, Some(cost_target(target)));
    effect::set_operation(d, e2, Some(operation(activate)));
    card::register_effect(d, c, e2, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let attacker = duel::get_attacker(d).nil();
    card::is_controler(d, attacker, 1 - a.tp)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0) > 0;
    }
    let attacker = duel::get_attacker(d).nil();
    duel::set_target_card(d, attacker);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    if duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0) == 0 {
        return;
    }
    let attacker = duel::get_attacker(d).nil();
    if !card::is_relate_to_effect(d, attacker, a.e) {
        return;
    }
    let hand = duel::get_field_group(d, a.tp, LOCATION_HAND, 0);
    let g = group::random_select(d, &hand, 1 - a.tp, 1);
    let tc = group::get_first(d, &g).nil();
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CARDTYPE as u64);
    let op = duel::select_option(d, 1 - a.tp, true, &[DECLTYPE_MONSTER as u64, DECLTYPE_SPELL as u64, DECLTYPE_TRAP as u64]);
    duel::confirm_cards(d, 1 - a.tp, tc);
    duel::shuffle_hand(d, a.tp);
    if (op != 0 && card::is_monster(d, tc)) || (op != 1 && card::is_spell(d, tc)) || (op != 2 && card::is_trap(d, tc)) {
        let attacker = duel::get_attacker(d).nil();
        duel::send_to_hand(d, attacker, None, REASON_EFFECT, None);
    }
}
