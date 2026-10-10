//! Ground Collapse
use super::prelude::*;

pub const ID: u32 = 90502999;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    card::register_effect(d, c, e1, false);
    // disable field
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_code(d, e2, EFFECT_DISABLE_FIELD);
    effect::set_operation(d, e2, Some(lua_fn(disop)));
    effect::set_label_object(d, e2, Value::Effect(e1));
    card::register_effect(d, c, e2, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let own = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, Some(PLAYER_NONE as u8), Some(0), None).0;
        let opp = duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, Some(PLAYER_NONE as u8), Some(0), None).0;
        return own + opp > 1;
    }
    let dis = duel::select_disable_field(d, a.tp, 2, LOCATION_MZONE, LOCATION_MZONE, Some(0), false);
    duel::hint(d, HINT_ZONE as u8, a.tp, dis as u64);
    effect::set_label(d, a.e, &[dis as i64]);
    true
}
/// `s.disop(e,tp)`
fn disop(d: &mut Duel, a: Args) -> i64 {
    let e = a.e(0);
    let lo = effect::get_label_object(d, e).as_effect().nil();
    effect::get_label(d, lo)
}
