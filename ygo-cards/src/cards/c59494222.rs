//! Gravekeeper's Inscription
use super::prelude::*;

pub const ID: u32 = 59494222;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Apply 1 of these effects
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
// function s.flagcheck(tp)
fn flagcheck(d: &mut Duel, tp: u8) -> (bool, bool, bool) {
    (
        duel::get_flag_effect(d, tp, ID + 1) == 0,
        duel::get_flag_effect(d, tp, ID + 2) == 0,
        duel::get_flag_effect(d, tp, ID + 3) == 0,
    )
}
// function s.condition(...) return Duel.IsPhase(PHASE_MAIN1) and not Duel.CheckPhaseActivity() end
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_phase(d, PHASE_MAIN1) && !duel::check_phase_activity(d)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (b1, b2, b3) = flagcheck(d, a.tp);
    if a.chk == Some(0) {
        return b1 || b2 || b3;
    }
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    let (b1, b2, b3) = flagcheck(d, a.tp);
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2)), (b3, aux::stringid(ID, 3))]);
    let Some(op) = op else { return };
    let code = if op == 1 {
        EFFECT_CANNOT_ACTIVATE
    } else if op == 2 {
        EFFECT_CANNOT_REMOVE
    } else {
        EFFECT_CANNOT_SPECIAL_SUMMON
    };
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_description(d, e1, aux::stringid(ID, op as u32 + 3));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e1, code);
    effect::set_target_range(d, e1, 1, 1);
    if op == 1 {
        effect::set_value_fn(d, e1, lua_fn(activate_value));
    } else {
        effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_GRAVE))));
    }
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 1);
    duel::register_effect(d, e1, a.tp);
    duel::register_flag_effect(d, a.tp, ID + op as u32, RESET_PHASE | PHASE_END, 0, 1, 0);
}
// e1:SetValue(function(_,re) return re:GetActivateLocation()==LOCATION_GRAVE end)
fn activate_value(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    effect::get_activate_location(d, re) == LOCATION_GRAVE
}
