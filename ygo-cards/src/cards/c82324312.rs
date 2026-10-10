//! Goblin Recon Squad
use super::prelude::*;

pub const ID: u32 = 82324312;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Look at 1 random card in opponent's hand. Send it to GY if spell
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_BATTLE_DAMAGE);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
    // Switch to defense position after attacking
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_PHASE | PHASE_BATTLE);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(poscon)));
    effect::set_operation(d, e2, Some(operation(posop)));
    card::register_effect(d, c, e2, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    a.ep == 1 - a.tp && duel::get_attack_target(d).is_none()
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.ep, LOCATION_HAND, 0) > 0;
    }
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_field_group(d, a.ep, LOCATION_HAND, 0);
    if g.is_empty() {
        return;
    }
    let sg = group::random_select(d, &g, a.ep, 1);
    duel::confirm_cards(d, a.tp, sg.clone());
    let f = group::get_first(d, &sg).nil();
    if card::is_spell(d, f) {
        duel::send_to_grave(d, sg, REASON_EFFECT, None, None);
    }
    duel::shuffle_hand(d, 1 - a.tp);
}
fn poscon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::get_attacked_count(d, h) > 0
}
fn posop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_attack_pos(d, c) {
        duel::change_position(d, c, POS_FACEUP_DEFENSE as u8, None, None, None, false);
    }
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_CANNOT_CHANGE_POSITION);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_COPY_INHERIT, 0);
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 3);
    card::register_effect(d, c, e1, false);
}
