//! Double or Nothing!
use super::prelude::*;

pub const ID: u32 = 94770493;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Allow a monster, whose attack was negated, to make a second attack
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_ATTACK_DISABLED);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    let first = group::get_first(d, &eg);
    if let Some(chkc) = a.chkc() {
        return Some(chkc) == first;
    }
    if a.chk == Some(0) {
        let tc = first.nil();
        return card::is_faceup(d, tc) && card::is_can_be_effect_target(d, tc, Some(a.e));
    }
    duel::set_target_card(d, first.nil());
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d).nil();
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) && card::get_flag_effect(d, tc, ID) == 0 {
        card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
        // Can make another attack
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
        effect::set_value(d, e1, card::get_attack_announced_count(d, tc) as i64);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e1, false);
        // Double its ATK
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, e2, EVENT_BATTLE_START);
        effect::set_count_limit(d, e2, 1, 0, 0, 0);
        effect::set_operation(d, e2, Some(operation(atkop)));
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e2, false);
    }
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    // Double its ATK
    let owner = effect::get_owner(d, a.e).nil();
    let e1 = effect::create_effect(d, owner);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
    effect::set_range(d, e1, LOCATION_MZONE);
    let h = effect::get_handler(d, a.e).nil();
    let atk = card::get_attack(d, h) * 2;
    effect::set_value(d, e1, atk as i64);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 1);
    card::register_effect(d, h, e1, false);
}
