//! High Rate Draw
use super::prelude::*;

pub const ID: u32 = 70569684;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy 2+ of your monsters, and if you do, for every 2 destroyed, draw 1
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    card::register_effect(d, c, e1, false);
    // Set itself from GY
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 2));
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_SET);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END));
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(setcon)));
    effect::set_target(d, e2, Some(cost_target(settg)));
    effect::set_operation(d, e2, Some(operation(setop)));
    card::register_effect(d, c, e2, false);
}
fn filter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    card::is_monster(d, c) && card::is_destructable(d, c, Some(e))
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 0) && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e)), a.tp, LOCATION_ONFIELD, 0, 2, NO_EX);
    }
    let g = duel::get_matching_group(d, Some(&|d, c| filter(d, c, a.e)), a.tp, LOCATION_ONFIELD, 0, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 2, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| filter(d, c, a.e)), a.tp, LOCATION_ONFIELD, 0, 2, 99, false, NO_EX).nil();
    let ct = duel::destroy(d, g, REASON_EFFECT, None, None);
    if ct > 1 {
        duel::draw(d, a.tp, (ct / 2) as u32, REASON_EFFECT);
    }
}
fn setcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_main_phase(d, None) && duel::is_turn_player(d, 1 - a.tp)
}
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // Lua `chkc` here is an undefined global (always nil), so the chkc branch never runs.
    let chkc: Option<CardId> = None;
    if let Some(chkc) = chkc {
        return card::is_location(d, chkc, LOCATION_ONFIELD) && card::is_faceup(d, chkc) && card::is_controler(d, chkc, a.tp) && filter(d, chkc, a.e);
    }
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_s_setable(d, h, false, None) && duel::is_existing_target(d, Some(&|d, c| filter(d, c, a.e)), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    let g = duel::select_target(d, a.tp, Some(&|d, c| filter(d, c, a.e)), a.tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
fn setop(d: &mut Duel, a: &EffArgs) {
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) && duel::destroy(d, tc, REASON_EFFECT, None, None) > 0 {
            let c = effect::get_handler(d, a.e).nil();
            if card::is_relate_to_effect(d, c, a.e) && card::is_s_setable(d, c, false, None) {
                duel::s_set(d, a.tp, c, None, true);
                // Banish it if it leaves the field
                let e1 = effect::create_effect(d, c);
                effect::set_description(d, e1, 3300);
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
                effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
                effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
                effect::set_value(d, e1, LOCATION_REMOVED as i64);
                card::register_effect(d, c, e1, false);
            }
        }
    }
}
