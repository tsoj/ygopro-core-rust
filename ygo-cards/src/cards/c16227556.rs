//! Inspection
use super::prelude::*;

pub const ID: u32 = 16227556;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    //
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(cfcon)));
    effect::set_cost(d, e2, Some(cost::pay_lp(cost::PayLp::Amount(500))));
    effect::set_operation(d, e2, Some(operation(cfop)));
    card::register_effect(d, c, e2, false);
}
// function s.cfcon(e,tp,eg,ep,ev,re,r,rp)
fn cfcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp) and Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)~=0
    duel::is_turn_player(d, 1 - a.tp) && duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) != 0
}
// function s.cfop(e,tp,eg,ep,ev,re,r,rp)
fn cfop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_HAND):RandomSelect(tp,1)
    let fg = duel::get_field_group(d, a.tp, 0, LOCATION_HAND);
    let g = group::random_select(d, &fg, a.tp, 1);
    // if #g~=0 then
    if !g.is_empty() {
        // Duel.ConfirmCards(tp,g)
        duel::confirm_cards(d, a.tp, g);
        // Duel.ShuffleHand(1-tp)
        duel::shuffle_hand(d, 1 - a.tp);
    }
}
