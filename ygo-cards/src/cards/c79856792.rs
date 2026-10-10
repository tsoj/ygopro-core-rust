//! Rainbow Dragon
use super::prelude::*;

pub const ID: u32 = 79856792;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CRYSTAL_BEAST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Special Summon procedure
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    card::register_effect(d, c, e1, false);
    // Special Summon condition
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e2, EFFECT_SPSUMMON_CONDITION);
    card::register_effect(d, c, e2, false);
    // Gain 1000 ATK for each "Crystal Beast" monster sent to the GY
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_ATKCHANGE);
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e3, EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e3, TIMING_DAMAGE_STEP as u32, None);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_condition(d, e3, Some(condition(atkcon)));
    effect::set_cost(d, e3, Some(cost_target(atkcost)));
    effect::set_operation(d, e3, Some(operation(atkop)));
    card::register_effect(d, c, e3, false);
    // Shuffle all cards on the field into the Deck
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 1));
    effect::set_category(d, e4, CATEGORY_TODECK);
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_condition(d, e4, Some(condition(|d, a| !card::has_flag_effect(d, effect::get_handler(d, a.e).nil(), ID, None))));
    effect::set_cost(d, e4, Some(cost_target(tdcost)));
    effect::set_target(d, e4, Some(cost_target(tdtg)));
    effect::set_operation(d, e4, Some(operation(tdop)));
    card::register_effect(d, c, e4, false);
    // Register a flag on Special Summon
    let e5 = effect::create_effect(d, c);
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e5, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e5, EVENT_SPSUMMON_SUCCESS);
    effect::set_operation(d, e5, Some(operation(spop)));
    card::register_effect(d, c, e5, false);
}
fn spcon(d: &mut Duel, a: Args) -> bool {
    let c = match a.card(1) {
        Some(c) => c,
        None => return true,
    };
    let tp = card::get_controler(d, c);
    let g = duel::get_matching_group(d, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16]))), tp, LOCATION_ONFIELD | LOCATION_GRAVE, 0, NO_EX);
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && group::get_class_count(d, &g, &|d, c| card::get_code(d, c).first().copied().unwrap_or(0) as i64) == 7
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, c, ID, RESET_EVENT | (RESETS_STANDARD & !(RESET_TOFIELD | RESET_TURN_SET)) | RESET_PHASE | PHASE_END, 0, 1, 0, 0);
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if card::has_flag_effect(d, c, ID, None) {
        return false;
    }
    aux::stat_change_damage_step_condition(d)
}
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16]) && card::is_able_to_grave_as_cost(d, c)
}
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    let g = duel::get_matching_group(d, Some(&atkfilter), a.tp, LOCATION_MZONE, 0, NO_EX);
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return !g.is_empty() && !group::is_contains(&g, h);
    }
    let n = g.len() as i64;
    duel::send_to_grave(d, g, REASON_COST, None, None);
    effect::set_label(d, a.e, &[n]);
    true
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // Increase ATK
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_value(d, e1, effect::get_label(d, a.e) * 1000);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        card::register_effect(d, c, e1, false);
    }
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16]) && card::is_monster(d, c) && aux::sp_elim_filter(d, c, true, false)
}
fn tdcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let g = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    let ct = group::filter_count(d, &g, &|d, c| card::is_able_to_remove_as_cost(d, c, None), NO_EX);
    if a.chk == Some(0) {
        return g.len() == ct;
    }
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_deck(d, c)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_deck(d, c)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, 0, 0);
    true
}
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_deck(d, c)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
}
