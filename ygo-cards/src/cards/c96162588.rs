//! Rikka Flurries
use super::prelude::*;

pub const ID: u32 = 96162588;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_RIKKA],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Make the opponent tribute a monster
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_RELEASE);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_RELEASE);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(relcon)));
    effect::set_target(d, e2, Some(cost_target(reltg)));
    effect::set_operation(d, e2, Some(operation(relop)));
    card::register_effect(d, c, e2, false);
    // Destroy this card
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_DESTROY);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_condition(d, e3, Some(condition(descon)));
    effect::set_target(d, e3, Some(cost_target(destg)));
    effect::set_operation(d, e3, Some(operation(desop)));
    card::register_effect(d, c, e3, false);
}
fn relfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_previous_set_card(d, c, &[SET_RIKKA as u16]) && card::is_previous_location(d, c, LOCATION_MZONE) && card::is_previous_controler(d, c, tp)
}
fn relcon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| relfilter(d, c, a.tp), 1, NO_EX)
}
fn reltg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_releasable(d, c, None, None)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_RELEASE, None, 1, 0, LOCATION_MZONE as i32);
    true
}
fn relop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_releasable(d, c, None, None)), 1 - a.tp, LOCATION_MZONE, 0, NO_EX);
    if g.len() > 0 {
        duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_RELEASE as u64);
        let sg = group::select(d, &g, 1 - a.tp, 1, 1, false, NO_EX).nil();
        duel::hint_selection(d, sg.clone(), true);
        duel::release(d, sg, REASON_RULE, Some(1 - a.tp));
    }
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && !card::is_race(d, c, RACE_PLANT)), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        duel::destroy(d, c, REASON_EFFECT, None, None);
    }
}
