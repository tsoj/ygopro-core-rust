//! Gravekeeper's Shaman
use super::prelude::*;

pub const ID: u32 = 58139128;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GRAVEKEEPERS], listed_names: &[CARD_NECROVALLEY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // def up
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_UPDATE_DEFENSE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_value_fn(d, e1, effect_card_value(defval));
    card::register_effect(d, c, e1, false);
    // disable
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_code(d, e2, EVENT_CHAIN_ACTIVATING);
    effect::set_operation(d, e2, Some(operation(disop)));
    card::register_effect(d, c, e2, false);
    // cannot activate
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_code(d, e3, EFFECT_CANNOT_ACTIVATE);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e3, 0, 1);
    effect::set_condition(d, e3, Some(condition(econ)));
    effect::set_value_fn(d, e3, lua_fn(efilter1));
    card::register_effect(d, c, e3, false);
    // indestructable by the opponent's effects
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    effect::set_code(d, e4, EFFECT_INDESTRUCTABLE_EFFECT);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_property(d, e4, EFFECT_FLAG_IGNORE_RANGE + EFFECT_FLAG_SET_AVAILABLE, 0);
    effect::set_condition(d, e4, Some(condition(econ)));
    effect::set_target(d, e4, Some(effect_card_fn(etarget)));
    effect::set_value_fn(d, e4, lua_fn(efilter2));
    card::register_effect(d, c, e4, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_GRAVEKEEPERS as u16]) && card::is_monster(d, c)
}
fn defval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    let p = card::get_controler(d, c);
    duel::get_matching_group_count(d, Some(&filter), p, LOCATION_GRAVE, 0, NO_EX) as i64 * 200
}
fn disop(d: &mut Duel, a: &EffArgs) {
    let loc = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION])[0].int();
    let re = a.re.nil();
    if effect::is_monster_effect(d, re) && !is_gravekeeper_handler(d, re) && loc == LOCATION_GRAVE as i64 {
        duel::negate_effect(d, a.ev as u8);
    }
}
/// `re:GetHandler():IsSetCard(SET_GRAVEKEEPERS)`
fn is_gravekeeper_handler(d: &mut Duel, re: EffectId) -> bool {
    let h = effect::get_handler(d, re).nil();
    card::is_set_card(d, h, &[SET_GRAVEKEEPERS as u16])
}
fn econ(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_environment(d, CARD_NECROVALLEY, None, None)
}
fn efilter1(d: &mut Duel, a: Args) -> bool {
    // return re:GetHandler():IsType(TYPE_FIELD) and re:IsHasType(EFFECT_TYPE_ACTIVATE)
    let re = a.e(1);
    let h = effect::get_handler(d, re).nil();
    card::is_type(d, h, TYPE_FIELD) && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
}
fn etarget(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_type(d, c, TYPE_FIELD)
}
fn efilter2(d: &mut Duel, a: Args) -> bool {
    // return e:GetOwnerPlayer()~=re:GetOwnerPlayer()
    effect::get_owner_player(d, a.e(0)) != effect::get_owner_player(d, a.e(1))
}
