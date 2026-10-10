//! Hino-Kagu-Tsuchi
use super::prelude::*;

pub const ID: u32 = 75745607;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Spirit.AddProcedure(c,EVENT_SUMMON_SUCCESS,EVENT_FLIP)
    spirit::add_procedure(d, c, &[EVENT_SUMMON_SUCCESS, EVENT_FLIP]);
    // Cannot be Special Summoned
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    card::register_effect(d, c, e1, false);
    // Opponent discards their hand next Draw Phase
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_BATTLE_DAMAGE);
    // e2:SetCondition(function(_,tp,_,ep) return ep==1-tp end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    effect::set_operation(d, e2, Some(operation(hdreg)));
    card::register_effect(d, c, e2, false);
}
fn e2_cond(_d: &mut Duel, a: &EffArgs) -> bool {
    a.ep == 1 - a.tp
}
fn hdreg(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    // Discard hand in the Draw Phase
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_PREDRAW);
    effect::set_operation(d, e1, Some(operation(hdop)));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_DRAW, 1);
    duel::register_effect(d, e1, a.tp);
    let h = aux::register_client_hint(d, c, 0, a.tp, 0, 1, Some(aux::stringid(ID, 1)), 0, None);
    effect::set_reset(d, h, RESET_PHASE | PHASE_DRAW, 1);
}
fn hdop(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_HAND);
    if !g.is_empty() {
        duel::send_to_grave(d, &g, REASON_DISCARD | REASON_EFFECT, None, None);
    }
}
