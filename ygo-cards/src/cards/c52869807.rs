//! Blackwing - Gust the Backblast
use super::prelude::*;

pub const ID: u32 = 52869807;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BLACKWING], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    card::register_effect(d, c, e1, false);
    // atk down
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    effect::set_condition(d, e2, Some(lua_fn(atkcon)));
    effect::set_target(d, e2, Some(effect_card_fn(atktg)));
    effect::set_value(d, e2, -300);
    card::register_effect(d, c, e2, false);
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    let c = match a.card(1) {
        Some(c) => c,
        None => return true,
    };
    let tp = card::get_controler(d, c);
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::get_field_group_count(d, tp, LOCATION_ONFIELD, 0) == 0
}
/// `s.atkcon(e)`
fn atkcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let ph = duel::get_current_phase(d);
    let at = duel::get_attack_target(d);
    let tp = effect::get_handler_player(d, e);
    if !(ph == PHASE_DAMAGE || ph == PHASE_DAMAGE_CAL) {
        return false;
    }
    match at {
        Some(at) => card::is_controler(d, at, tp) && card::is_set_card(d, at, &[SET_BLACKWING as u16]),
        None => false,
    }
}
/// `s.atktg(e,c)`
fn atktg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    duel::get_attacker(d) == Some(c)
}
