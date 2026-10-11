//! Return
use super::prelude::*;

pub const ID: u32 = 47247413;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_category(d, e1, CATEGORY_TODECK);
    effect::set_code(d, e1, EVENT_TO_HAND);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId, e: Option<EffectId>, tp: u8) -> bool {
    card::is_previous_location(d, c, LOCATION_GRAVE) && card::is_controler(d, c, tp) && (match e {
        None => true,
        Some(e) => card::is_relate_to_effect(d, c, e),
    })
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| filter(d, c, None, 1 - a.tp), 1, NO_EX)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let eg = a.eg.clone().nil();
    duel::set_target_card(d, eg.clone());
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(eg), 1, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let g = a.eg.clone().nil();
    let g = group::filter(d, &g, &|d, c| filter(d, c, Some(a.e), 1 - a.tp), NO_EX);
    if g.is_empty() {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_TODECK as u64);
    let rg = group::select(d, &g, 1 - a.tp, 1, 1, false, NO_EX).nil();
    duel::confirm_cards(d, a.tp, rg.clone());
    duel::send_to_deck(d, rg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
}
