//! Chef's Special Recipe
use super::prelude::*;

pub const ID: u32 = 87955518;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[30243636], listed_series: &[SET_NOUVELLES], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate activation
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(negcon)));
    effect::set_target(d, e1, Some(cost_target(negtg)));
    effect::set_operation(d, e1, Some(operation(negop)));
    card::register_effect(d, c, e1, false);
    // Tribute as many monsters your opponent controls as possible
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_RELEASE);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_condition(d, e2, Some(condition(tribtcond)));
    effect::set_target(d, e2, Some(cost_target(tribttg)));
    effect::set_operation(d, e2, Some(operation(tribtop)));
    card::register_effect(d, c, e2, false);
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_ritual_monster(d, c) && card::is_set_card(d, c, &[SET_NOUVELLES as u16]) && card::is_faceup(d, c)
}
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    if !duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX) {
        return false;
    }
    let re = a.re.nil();
    if !(effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) || effect::is_monster_effect(d, re)) {
        return false;
    }
    duel::is_chain_negatable(d, a.ev as u8)
}
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    if card::is_destructable(d, rc, None) && card::is_relate_to_effect(d, rc, re) {
        duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, a.tp, 0);
    }
    true
}
fn negop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if !duel::negate_activation(d, a.ev as u8) {
        return;
    }
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    if !card::is_relate_to_effect(d, rc, re) {
        return;
    }
    if !duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_nouvelles_summoned(d, c))), tp, LOCATION_MZONE, 0, 1, NO_EX) {
        return;
    }
    if !duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
        return;
    }
    duel::break_effect(d);
    duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
}
fn burgerfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_code(d, c, &[30243636]) && card::is_faceup(d, c) && card::is_summon_player(d, c, tp)
}
fn tribtcond(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| burgerfilter(d, c, tp), 1, NO_EX)
}
fn releasable(d: &mut Duel, c: CardId) -> bool {
    card::is_releasable_by_effect(d, c, None, None)
}
fn tribttg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&releasable), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    let g = duel::get_matching_group(d, Some(&releasable), tp, 0, LOCATION_MZONE, NO_EX);
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_RELEASE, Some(g), n, 0, 0);
    true
}
fn tribtop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let g = duel::get_matching_group(d, Some(&releasable), tp, 0, LOCATION_MZONE, NO_EX);
    if g.len() > 0 {
        duel::release(d, g, REASON_EFFECT, None);
    }
}
