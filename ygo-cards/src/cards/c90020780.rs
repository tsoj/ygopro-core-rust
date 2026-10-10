//! Shinobi Necro
use super::prelude::*;

pub const ID: u32 = 90020780;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Draw 1, then discard 1
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DRAW + CATEGORY_HANDES);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(dcondition)));
    effect::set_target(d, e1, Some(cost_target(dtarget)));
    effect::set_operation(d, e1, Some(operation(doperation)));
    card::register_effect(d, c, e1, false);
    // If banished from GY, special summon itself
    let e2 = effect::create_effect(d, c);
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_REMOVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(spcondition)));
    effect::set_target(d, e2, Some(cost_target(sptarget)));
    effect::set_operation(d, e2, Some(operation(spoperation)));
    card::register_effect(d, c, e2, false);
}
fn dfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_previous_location(d, c, LOCATION_GRAVE) && card::is_previous_controler(d, c, tp) && card::is_race(d, c, RACE_ZOMBIE)
}
fn dcondition(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let eg = a.eg();
    if group::is_contains(&eg, c) {
        return false;
    }
    group::is_exists(d, &eg, &|d, x| dfilter(d, x, a.tp), 1, c.into())
}
fn dtarget(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1);
    }
    duel::set_target_player(d, a.tp);
    duel::set_target_param(d, 1);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, a.tp, 1);
    true
}
fn doperation(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let (p, n) = (info[0].int() as u8, info[1].int() as u32);
    if duel::draw(d, p, n, REASON_EFFECT) != 0 {
        duel::shuffle_hand(d, tp);
        duel::break_effect(d);
        duel::discard_hand(d, tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
    }
}
fn spcondition(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, c, LOCATION_GRAVE)
        && (card::is_reason(d, c, REASON_EFFECT)
            || (card::is_reason(d, c, REASON_COST) && effect::is_has_type(d, a.re.nil(), !EFFECT_TYPE_FIELD)))
}
fn sptarget(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn spoperation(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
        // Banish it if it leaves the field
        let e1 = effect::create_effect(d, c);
        effect::set_description(d, e1, 3300);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
        effect::set_value(d, e1, LOCATION_REMOVED as i64);
        card::register_effect(d, c, e1, true);
    }
}
