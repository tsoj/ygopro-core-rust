//! Half or Nothing
use super::prelude::*;

pub const ID: u32 = 15552258;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_BATTLE_START));
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return Duel.IsTurnPlayer(1-tp) and Duel.IsBattlePhase() end`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp) && duel::is_battle_phase(d, None)
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let (tp, id) = (a.tp, ID);
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, NO_EX);
    let opt = if group::get_count(&g) == 0 {
        duel::select_option(d, 1 - tp, true, &[aux::stringid(id, 1)]) + 1
    } else {
        duel::select_option(d, 1 - tp, true, &[aux::stringid(id, 0), aux::stringid(id, 1)])
    };
    if opt == 1 {
        duel::skip_phase(d, 1 - tp, PHASE_BATTLE, RESET_PHASE | PHASE_BATTLE_STEP, 1, 0);
        return;
    }
    let _ = group::get_first(d, &g);
    for tc in group::iter(d, &g) {
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // tc:GetAttack()/2 (Lua float division; the core rounds half away from zero)
        let atk = card::get_attack(d, tc);
        effect::set_value(d, e1, lua_int(atk as f64 / 2.0));
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE, 1);
        card::register_effect(d, tc, e1, false);
    }
}
