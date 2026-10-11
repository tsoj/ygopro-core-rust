//! Pop-Up
use super::prelude::*;

pub const ID: u32 = 15248873;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_field_card(d, a.tp, LOCATION_FZONE, 0).is_none() && duel::get_field_card(d, 1 - a.tp, LOCATION_FZONE, 0).is_some()
}
/// `s.filter(c,tp)`: `c:IsFieldSpell() and c:GetActivateEffect():IsActivatable(tp,true,true)`
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_field_spell(d, c) && {
        let e = card::get_activate_effect(d, c).first().copied().nil();
        effect::is_activatable(d, e, tp, true, true)
    }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    if !duel::check_phase_activity(d) {
        duel::register_flag_effect(d, tp, CARD_MAGICAL_MIDBREAKER, RESET_CHAIN, 0, 1, 0);
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 0));
    let g = duel::select_matching_card(d, tp, Some(&|d, c| filter(d, c, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    duel::activate_field_spell(d, tc, a, None);
}
