//! Forced Requisition
use super::prelude::*;

pub const ID: u32 = 74923978;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_DISCARD);
    effect::set_condition(d, e1, Some(condition(cond)));
    card::register_effect(d, c, e1, false);
    // handes
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_HANDES);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_code(d, e2, EVENT_DISCARD);
    effect::set_condition(d, e2, Some(condition(cond)));
    effect::set_target(d, e2, Some(cost_target(target)));
    effect::set_operation(d, e2, Some(operation(op)));
    card::register_effect(d, c, e2, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(Card.IsPreviousControler,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| card::get_previous_controler(d, c) == a.tp, 1, NO_EX)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_relate_to_effect(d, h, a.e);
    }
    let eg = a.eg.clone().nil();
    let ct = group::filter_count(d, &eg, &|d, c| card::get_previous_controler(d, c) == a.tp, NO_EX);
    effect::set_label(d, a.e, &[ct as i64]);
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, 1 - a.tp, ct as i32);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    let ct = effect::get_label(d, a.e) as u16;
    duel::discard_hand(d, 1 - a.tp, None, ct, ct, REASON_EFFECT | REASON_DISCARD, NO_EX);
}
