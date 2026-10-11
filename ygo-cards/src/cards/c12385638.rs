//! Kozmourning
use super::prelude::*;

pub const ID: u32 = 12385638;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_KOZMO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // to deck
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_code(d, e2, EFFECT_BATTLE_DESTROY_REDIRECT);
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    effect::set_target(d, e2, Some(effect_card_fn(tdtg)));
    effect::set_value(d, e2, LOCATION_DECKSHF as i64);
    card::register_effect(d, c, e2, false);
    // damage conversion
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    effect::set_range(d, e3, LOCATION_GRAVE);
    effect::set_cost(d, e3, Some(cost::self_banish()));
    effect::set_operation(d, e3, Some(operation(op)));
    card::register_effect(d, c, e3, false);
}
/// `s.tdtg(e,c)`
fn tdtg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_KOZMO as u16])
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EFFECT_REVERSE_DAMAGE);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_value_fn(d, e1, lua_fn(valcon));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
}
/// `s.valcon(e,re,r,rp,rc)`
fn valcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let r = a.int(2) as u32;
    if (r & REASON_BATTLE) != 0 {
        let tp = effect::get_handler_player(d, e);
        let rc = a.card(4).nil();
        let bc = card::get_battle_target(d, rc);
        if let Some(bc) = bc {
            if card::is_set_card(d, bc, &[SET_KOZMO as u16])
                && card::is_controler(d, bc, tp)
                && duel::get_flag_effect(d, tp, ID) == 0
            {
                duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
                return true;
            }
        }
    }
    false
}
