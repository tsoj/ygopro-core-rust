//! Alien Skull
use super::prelude::*;

pub const ID: u32 = 25920413;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ALIEN], counter_place_list: &[COUNTER_A as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    aux::add_lava_procedure(
        d,
        c,
        Some(1),
        POS_FACEUP as u32,
        Some(std::sync::Arc::new(|d: &mut Duel, c: CardId| card::is_faceup(d, c) && card::is_level_below(d, c, 3))),
        Some(1),
        None,
    );
    // counter
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_COUNTER);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e2, Some(condition(ctcon)));
    effect::set_operation(d, e2, Some(operation(ctop)));
    card::register_effect(d, c, e2, false);
    // atk def
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_condition(d, e3, Some(lua_fn(adcon)));
    effect::set_target(d, e3, Some(effect_card_fn(adtg)));
    effect::set_value_fn(d, e3, effect_card_value(adval));
    card::register_effect(d, c, e3, false);
    let e4 = effect::clone(d, e3);
    effect::set_code(d, e4, EFFECT_UPDATE_DEFENSE);
    card::register_effect(d, c, e4, false);
    //
    let e5 = effect::create_effect(d, c);
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e5, EFFECT_SPSUMMON_COST);
    effect::set_property(d, e5, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_cost(d, e5, Some(lua_fn(spcost)));
    effect::set_operation(d, e5, Some(operation(spcop)));
    card::register_effect(d, c, e5, false);
}
fn spcost(d: &mut Duel, a: Args<'_>) -> bool {
    let tp = a.p(2);
    let sumtype = a.int(3);
    sumtype != (SUMMON_TYPE_SPECIAL + 1) as i64 || duel::get_activity_count(d, tp, &[ACTIVITY_NORMALSUMMON])[0] == 0
}
fn spcop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CANNOT_SUMMON);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    effect::set_target_range(d, e1, 1, 0);
    duel::register_effect(d, e1, a.tp);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_CANNOT_MSET);
    duel::register_effect(d, e2, a.tp);
}
fn ctcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::get_summon_type(d, c) == SUMMON_TYPE_SPECIAL + 1
}
fn ctop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_faceup(d, c) {
        card::add_counter(d, c, (COUNTER_NEED_ENABLE + COUNTER_A) as u16, 1, false);
    }
}
fn adcon(d: &mut Duel, _a: Args<'_>) -> bool {
    duel::is_phase(d, PHASE_DAMAGE_CAL) && duel::get_attack_target(d).is_some()
}
fn adtg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    match card::get_battle_target(d, c) {
        Some(bc) => card::get_counter(d, c, COUNTER_A as u16) != 0 && card::is_set_card(d, bc, &[SET_ALIEN as u16]),
        None => false,
    }
}
fn adval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    card::get_counter(d, c, COUNTER_A as u16) as i64 * -300
}
