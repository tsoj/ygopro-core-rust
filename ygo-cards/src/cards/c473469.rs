//! Driving Snow
use super::prelude::*;

pub const ID: u32 = 473469;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_trap(d, c)
        && card::is_previous_location(d, c, LOCATION_ONFIELD)
        && card::is_previous_controler(d, c, tp)
        && (card::get_reason(d, c) & (REASON_DESTROY | REASON_EFFECT)) == (REASON_DESTROY | REASON_EFFECT)
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    if a.rp == a.tp {
        return false;
    }
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| filter(d, c, a.tp), 1, NO_EX)
}
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_spell_trap(d, c)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && desfilter(d, chkc) && chkc != c;
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&desfilter), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    let g = duel::select_target(d, a.tp, Some(&desfilter), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, c.into());
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
