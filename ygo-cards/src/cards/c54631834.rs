//! Terminal World
use super::prelude::*;

pub const ID: u32 = 54631834;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_condition(d, e1, Some(condition(cond)));
    card::register_effect(d, c, e1, false);
    // skip
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target_range(d, e2, 1, 1);
    effect::set_code(d, e2, EFFECT_SKIP_M2);
    card::register_effect(d, c, e2, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp) return Duel.IsPhase(PHASE_MAIN1) end
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_phase(d, PHASE_MAIN1)
}
