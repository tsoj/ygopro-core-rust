//! Lord of the Tachyon Galaxy
use super::prelude::*;

pub const ID: u32 = 44466810;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_GALAXY_EYES, SET_GALAXY_EYES_TACHYON_DRAGON, SET_NUMBER_C],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Can be activated from the hand
    let e0 = effect::create_effect(d, c);
    effect::set_description(d, e0, aux::stringid(ID, 0));
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e0, EFFECT_TRAP_ACT_IN_HAND);
    effect::set_condition(d, e0, Some(condition(handcon)));
    card::register_effect(d, c, e0, false);
    // Shuffle opponent monsters to the Deck
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 1));
    effect::set_category(d, e1, CATEGORY_TODECK);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_BATTLE_START | TIMING_BATTLE_END));
    effect::set_label(d, e1, &[ID as i64]);
    effect::set_condition(d, e1, Some(condition(|d, _a| duel::is_battle_phase(d, None))));
    effect::set_cost(d, e1, Some(cost_target(tdcost)));
    effect::set_target(d, e1, Some(cost_target(tdtg)));
    effect::set_operation(d, e1, Some(operation(tdop)));
    card::register_effect(d, c, e1, false);
    // The activation and effect of e1 cannot be negated if you control a "Number C" monster
    aux::global_check(d, ID, |d| {
        let ge1 = effect::create_effect(d, c);
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD);
        effect::set_code(d, ge1, EFFECT_CANNOT_INACTIVATE);
        effect::set_value_fn(d, ge1, lua_fn(effectfilter));
        duel::register_effect(d, ge1, 0);
        let ge2 = effect::clone(d, ge1);
        effect::set_code(d, ge2, EFFECT_CANNOT_DISEFFECT);
        duel::register_effect(d, ge2, 0);
    });
}
/// `function(e,ct)`
fn effectfilter(d: &mut Duel, a: Args) -> bool {
    let ct = a.int(1) as u8;
    let info = duel::get_chain_info(d, ct, &[CHAININFO_TRIGGERING_EFFECT, CHAININFO_TRIGGERING_PLAYER]);
    let te = info[0].as_effect().nil();
    let tp = info[1].int() as u8;
    effect::get_label(d, te) == ID as i64
        && duel::is_existing_matching_card(
            d,
            Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_NUMBER_C as u16])),
            tp,
            LOCATION_MZONE,
            0,
            1,
            NO_EX,
        )
}
fn handcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(
        d,
        Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_GALAXY_EYES_TACHYON_DRAGON as u16])),
        effect::get_handler_player(d, a.e),
        LOCATION_MZONE,
        0,
        1,
        NO_EX,
    )
}
fn tdcostfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_type(d, c, TYPE_XYZ) && card::is_set_card(d, c, &[SET_GALAXY_EYES as u16]) && card::is_faceup(d, c)
}
fn tdcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let xg = duel::get_matching_group(d, Some(&tdcostfilter), tp, LOCATION_MZONE, 0, NO_EX);
    if a.chk == Some(0) {
        return duel::check_remove_overlay_card(d, tp, 1, 0, 1, REASON_COST, Some(&xg));
    }
    duel::remove_overlay_card(d, tp, 1, 0, 1, 1, REASON_COST, Some(&xg));
    true
}
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_status(d, c, STATUS_SPSUMMON_TURN | STATUS_SUMMON_TURN) && card::is_able_to_deck(d, c)
}
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let g = duel::get_matching_group(d, Some(&tdfilter), a.tp, 0, LOCATION_MZONE, NO_EX);
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, a.tp, 0);
    true
}
fn tdop(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_matching_group(d, Some(&tdfilter), a.tp, 0, LOCATION_MZONE, NO_EX);
    if !g.is_empty() {
        duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
