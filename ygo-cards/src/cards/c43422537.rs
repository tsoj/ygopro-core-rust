//! Double Summon
use super::prelude::*;

pub const ID: u32 = 43422537;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // local ct=0
        let mut ct = 0i64;
        // local ce={Duel.IsPlayerAffectedByEffect(tp,EFFECT_SET_SUMMON_COUNT_LIMIT)}
        // for _,te in ipairs(ce) do ct=math.max(ct,te:GetValue()) end
        if let Some(te) = duel::is_player_affected_by_effect(d, a.tp, EFFECT_SET_SUMMON_COUNT_LIMIT) {
            ct = ct.max(effect::get_value(d, te).int());
        }
        return ct < 2;
    }
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_SET_SUMMON_COUNT_LIMIT);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_value(d, e1, 2);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
}
