//! Vivid Tail
use super::prelude::*;

pub const ID: u32 = 7608148;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_TOHAND);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E as u32));
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Set itself
    let e2 = effect::create_effect(d, c);
    effect::set_category(d, e2, CATEGORY_LEAVE_GRAVE | CATEGORY_TOHAND | CATEGORY_SET);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER_E as u32));
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_target(d, e2, Some(cost_target(settg)));
    effect::set_operation(d, e2, Some(operation(setop)));
    card::register_effect(d, c, e2, false);
}
fn tohand_ok(d: &mut Duel, c: CardId) -> bool {
    card::is_able_to_hand(d, c, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, tp) && card::is_able_to_hand(d, chkc, None) && chkc != c;
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tohand_ok), tp, LOCATION_ONFIELD, 0, 1, c.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    let g = duel::select_target(d, tp, Some(&tohand_ok), tp, LOCATION_ONFIELD, 0, 1, 1, false, c.into());
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
    }
}
fn faceup_tohand(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_able_to_hand(d, c, None)
}
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, tp) && faceup_tohand(d, chkc);
    }
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_s_setable(d, c, false, None) && duel::is_existing_target(d, Some(&faceup_tohand), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    let g = duel::select_target(d, tp, Some(&faceup_tohand), tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), n, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn setop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d);
    if card::is_relate_to_effect(d, c, a.e) && card::is_s_setable(d, c, false, None) && duel::s_set(d, tp, c, None, true) > 0 {
        // Banish it when it leaves the field
        let e1 = effect::create_effect(d, c);
        effect::set_description(d, e1, 3300);
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
        effect::set_value(d, e1, LOCATION_REMOVED as i64);
        card::register_effect(d, c, e1, false);
        let tc = tc.nil();
        if card::is_relate_to_effect(d, tc, a.e)
            && card::is_faceup(d, tc)
            && duel::send_to_hand(d, tc, None, REASON_EFFECT, None) > 0
            && card::is_location(d, tc, LOCATION_HAND)
        {
            // Cannot activate cards or effects with the same name
            let e2 = effect::create_effect(d, c);
            effect::set_type(d, e2, EFFECT_TYPE_FIELD);
            effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
            effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
            effect::set_target_range(d, e2, 1, 0);
            effect::set_value_fn(d, e2, lua_fn(aclimit));
            let codes = card::get_code(d, tc);
            effect::set_label(d, e2, &codes.iter().map(|x| *x as i64).collect::<Vec<i64>>());
            effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
            duel::register_effect(d, e2, tp);
        }
    }
}
fn aclimit(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    let h = effect::get_handler(d, re).nil();
    let label = effect::get_label(d, a.e(0));
    card::is_code(d, h, &[label as u32])
}
