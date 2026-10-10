//! Judgment of Anubis
use super::prelude::*;

pub const ID: u32 = 55256016;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_on_field(d, c) && card::is_spell_trap(d, c)
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    if a.tp == a.ep || !duel::is_chain_negatable(d, a.ev as u8) {
        return false;
    }
    if !(effect::is_spell_effect(d, a.re.nil()) && effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE)) {
        return false;
    }
    match duel::get_operation_info(d, a.ev, CATEGORY_DESTROY) {
        None => false,
        Some(info) => match info.op_cards {
            None => false,
            Some(tg) => {
                let fc = group::filter_count(d, &tg, &cfilter, NO_EX) as i64;
                info.op_count as i64 + fc - tg.len() as i64 > 0
            }
        },
    }
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    if duel::is_player_affected_by_effect(d, a.tp, EFFECT_DISCARD_COST_CHANGE).is_some() {
        return true;
    }
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), a.tp, LOCATION_HAND, 0, 1, c.into());
    }
    duel::discard_hand(d, a.tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    let rh = effect::get_handler(d, a.re.nil()).nil();
    if card::is_destructable(d, rh, None) && card::is_relate_to_effect(d, rh, a.re.nil()) {
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    if duel::negate_activation(d, a.ev as u8) {
        let rh = effect::get_handler(d, a.re.nil()).nil();
        if card::is_relate_to_effect(d, rh, a.re.nil()) {
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
    let dg = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, NO_EX);
    if !dg.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0)) {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
        let des = group::select(d, &dg, a.tp, 1, 1, false, NO_EX).nil();
        duel::hint_selection(d, des.clone(), true);
        let tc = group::get_first(d, &des).nil();
        let atk = card::get_attack(d, tc);
        duel::break_effect(d);
        if duel::destroy(d, des, REASON_EFFECT, None, None) > 0 {
            duel::damage(d, 1 - a.tp, atk as i64, REASON_EFFECT, false, None);
        }
    }
}
