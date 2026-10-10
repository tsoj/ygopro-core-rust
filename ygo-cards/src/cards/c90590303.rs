//! Number 41: Bagooska the Terribly Tired Tapir
use super::prelude::*;

pub const ID: u32 = 90590303;

pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(41), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    xyz::add_procedure(d, c, None, Some(4), 2, None, None, None, None, false, None);
    card::enable_revive_limit(d, c);
    // maintain
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(mtcon)));
    effect::set_operation(d, e1, Some(operation(mtop)));
    card::register_effect(d, c, e1, false);
    // indes/cannot target
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_condition(d, e2, Some(condition(tgcon)));
    effect::set_value_fn(d, e2, aux::tgoval());
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
    effect::set_value_fn(d, e3, lua_fn(indval));
    card::register_effect(d, c, e3, false);
    // pos
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    effect::set_code(d, e4, EFFECT_SET_POSITION);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_target_range(d, e4, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_condition(d, e4, Some(condition(poscon)));
    effect::set_target(d, e4, Some(effect_card_fn(postg)));
    effect::set_value(d, e4, POS_FACEUP_DEFENSE as i64);
    card::register_effect(d, c, e4, false);
    // disable
    let e5 = effect::create_effect(d, c);
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e5, EVENT_CHAIN_SOLVING);
    effect::set_range(d, e5, LOCATION_MZONE);
    effect::set_condition(d, e5, Some(condition(discon)));
    effect::set_operation(d, e5, Some(operation(disop)));
    card::register_effect(d, c, e5, false);
}
fn mtcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, a.tp)
}
fn mtop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::check_remove_overlay_card(d, c, a.tp, 1, REASON_COST) {
        card::remove_overlay_card(d, c, a.tp, 1, 1, REASON_COST);
    } else {
        duel::destroy(d, c, REASON_COST, None, None);
    }
}
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_attack_pos(d, c)
}
/// `s.indval(e,re,tp)`
fn indval(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(2);
    tp != effect::get_handler_player(d, e)
}
fn poscon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_defense_pos(d, c)
}
fn postg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_faceup(d, c)
}
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION, CHAININFO_TRIGGERING_POSITION]);
    let (loc, pos) = (info[0].int(), info[1].int());
    let c = effect::get_handler(d, a.e).nil();
    let re = a.re.nil();
    card::is_defense_pos(d, c) && effect::is_monster_effect(d, re) && loc == LOCATION_MZONE as i64 && (pos & POS_DEFENSE as i64) != 0
}
fn disop(d: &mut Duel, a: &EffArgs) {
    duel::negate_effect(d, a.ev as u8);
}
