//! Anti-Magic Arrows
use super::prelude::*;

pub const ID: u32 = 97120394;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, TIMING_BATTLE_START, None);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `function() return Duel.IsPhase(PHASE_BATTLE_START) end`
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_phase(d, PHASE_BATTLE_START)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE);
    }
    duel::set_chain_limit(d, aux_false());
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    if !effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // For the rest of this turn after this card resolves, neither player can activate Spell/Trap Cards, or their effects
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 1));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    effect::set_target_range(d, e1, 1, 1);
    effect::set_value_fn(d, e1, lua_fn(aclimit));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
}
/// `function(e,re,tp) return re:IsSpellTrapEffect() end`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    effect::is_spell_trap_effect(d, re)
}
