//! Gravelstorm
use super::prelude::*;

pub const ID: u32 = 48135190;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_TOHAND);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chkc().is_some() {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, 0, LOCATION_ONFIELD, 1, NO_EX)
            && duel::is_existing_target(d, Some(&filter), tp, LOCATION_ONFIELD, 0, 1, h.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    let mut g1 = duel::select_target(d, tp, Some(&filter), tp, LOCATION_ONFIELD, 0, 1, 1, false, h.into()).nil();
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    let g2 = duel::select_target(d, tp, Some(&filter), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    group::add_card(&mut g1, &g2);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g1), 2, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    let sg = group::filter(d, &g, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
    if !sg.is_empty() {
        duel::send_to_hand(d, sg, None, REASON_EFFECT, None);
    }
}
