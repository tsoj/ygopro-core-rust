//! Mereologic Aggregator
use super::prelude::*;

pub const ID: u32 = 9940036;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    xyz::add_procedure(d, c, None, Some(9), 2, None, None, Some(xyz::INFINITE_MATS), None, false, None);
    // Send 1 monster from the Extra Deck to the GY
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(tgcond)));
    effect::set_target(d, e1, Some(cost_target(tgtg)));
    effect::set_operation(d, e1, Some(operation(tgop)));
    card::register_effect(d, c, e1, false);
    // Attach materials to itself
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_target(d, e2, Some(cost_target(atchtg)));
    effect::set_operation(d, e2, Some(operation(atchop)));
    card::register_effect(d, c, e2, false);
    // Negate the effects of 1 face-up card
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_category(d, e3, CATEGORY_DISABLE);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    effect::set_target(d, e3, Some(cost_target(distg)));
    effect::set_operation(d, e3, Some(operation(disop)));
    card::register_effect(d, c, e3, false);
}
fn tgcond(d: &mut Duel, a: &EffArgs) -> bool {
    card::is_xyz_summoned(d, effect::get_handler(d, a.e).nil())
}
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_monster(d, c) && card::is_able_to_grave(d, c, None)
}
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
fn tgop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&tgfilter), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
fn atchtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_GRAVE) && card::is_monster(d, chkc);
    }
    let c = effect::get_handler(d, a.e).nil();
    let ct = card::get_overlay_group(d, c).len();
    if a.chk == Some(0) {
        return ct > 0
            && card::check_remove_overlay_card(d, c, a.tp, 1, REASON_EFFECT)
            && duel::is_existing_target(d, Some(&card::is_monster), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    let g = duel::select_target(d, a.tp, Some(&card::is_monster), a.tp, LOCATION_GRAVE, 0, 1, ct as u16, false, NO_EX).nil();
    effect::set_label(d, a.e, &[g.len() as i64]);
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(g.clone()), g.len() as u8, a.tp, 0);
    true
}
fn atchop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let oct = card::get_overlay_group(d, c).len() as i64;
    let ct = effect::get_label(d, a.e);
    if oct == 0 || oct < ct {
        return;
    }
    // Lua: `if c:RemoveOverlayCard(...)` tests an integer, which is always true
    card::remove_overlay_card(d, c, a.tp, ct as u16, ct as u16, REASON_EFFECT);
    let tg = duel::get_target_cards(d, a.e).nil();
    if !tg.is_empty() {
        duel::overlay(d, c, tg, false);
    }
}
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_negatable(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&card::is_negatable), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_NEGATE as u64);
    let g = duel::select_target(d, a.tp, Some(&card::is_negatable), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, g, 1, 0, 0);
    true
}
fn disop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if ((card::is_faceup(d, tc) && !card::is_disabled(d, tc)) || card::is_type(d, tc, TYPE_TRAPMONSTER)) && card::is_relate_to_effect(d, tc, a.e) {
        duel::negate_related_chain(d, tc, RESET_TURN_SET);
        let c = effect::get_handler(d, a.e).nil();
        // Negate its effects
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_code(d, e1, EFFECT_DISABLE);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e1, false);
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        effect::set_value(d, e2, RESET_TURN_SET as i64);
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e2, false);
        if card::is_type(d, tc, TYPE_TRAPMONSTER) {
            let e3 = effect::create_effect(d, c);
            effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
            effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
            effect::set_code(d, e3, EFFECT_DISABLE_TRAPMONSTER);
            effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
            card::register_effect(d, tc, e3, false);
        }
    }
}
