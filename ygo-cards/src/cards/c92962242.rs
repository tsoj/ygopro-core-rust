//! Jioh the Gravity Ninja
use super::prelude::*;

pub const ID: u32 = 92962242;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Flip up to 2 monsters face-down
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_POSITION + CATEGORY_SET);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(postg)));
    effect::set_operation(d, e1, Some(operation(posop)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EVENT_FLIP);
    card::register_effect(d, c, e3, false);
    // Destroy 1 monster
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 4));
    effect::set_category(d, e4, CATEGORY_DESTROY);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e4, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    effect::set_code(d, e4, EVENT_FLIP);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_count_limit(d, e4, 1, ID, 1, 0);
    effect::set_condition(d, e4, Some(condition(descon)));
    effect::set_target(d, e4, Some(cost_target(destg)));
    effect::set_operation(d, e4, Some(operation(desop)));
    card::register_effect(d, c, e4, false);
}
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_can_turn_set(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&card::is_can_turn_set), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    let g = duel::select_target(d, tp, Some(&card::is_can_turn_set), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 2, false, NO_EX);
    let n = g.as_ref().map_or(0, |x| x.len()) as u8;
    duel::set_operation_info(d, 0, CATEGORY_POSITION, g, n, tp, POS_FACEDOWN_DEFENSE as i32);
    true
}
fn posop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let mut g = duel::get_target_cards(d, a.e).nil();
    if g.is_empty() || duel::change_position(d, g.clone(), POS_FACEDOWN_DEFENSE as u8, None, None, None, false) == 0 {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    group::match_(d, &mut g, &|d, cc| card::is_controler(d, cc, 1 - tp), NO_EX);
    group::match_(d, &mut g, &|d, cc| card::is_position(d, cc, POS_FACEDOWN_DEFENSE), NO_EX);
    for tc in group::iter(d, &g) {
        // Cannot change its battle position
        let e1 = effect::create_effect(d, c);
        effect::set_description(d, e1, 3313);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_code(d, e1, EFFECT_CANNOT_CHANGE_POSITION);
        effect::set_reset(d, e1, RESET_EVENT + RESETS_STANDARD, 1);
        card::register_effect(d, tc, e1, false);
    }
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    let eg = a.eg.clone().nil();
    !group::is_contains(&eg, h)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_on_field(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    let g = duel::select_target(d, tp, None, tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    let n = g.as_ref().map_or(0, |x| x.len()) as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, n, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
