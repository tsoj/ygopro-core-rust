//! Exosister Asophiel
use super::prelude::*;

pub const ID: u32 = 41524885;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_EXOSISTER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // 2 Level 4 monsters
    xyz::add_procedure(d, c, None, Some(4), 2, None, None, None, None, false, None);
    // Check materials on Xyz Summon
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e0, EFFECT_MATERIAL_CHECK);
    effect::set_value_fn(d, e0, effect_card_value(valcheck));
    card::register_effect(d, c, e0, false);
    // Effect destruction immunity
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    effect::set_value_fn(d, e1, lua_fn(|d, a: Args| {
        let re = a.e(1);
        effect::is_activated(d, re) && effect::is_monster_effect(d, re) && chain::effect_is_card(d, re, chain::Prop::SummonType, &[SUMMON_TYPE_SPECIAL as i64]).truthy() && chain::effect_is_card(d, re, chain::Prop::SummonLocation, &[LOCATION_GRAVE as i64]).truthy()
    }));
    card::register_effect(d, c, e1, false);
    // Limit effects from GY
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(limcon)));
    effect::set_operation(d, e2, Some(operation(limop)));
    card::register_effect(d, c, e2, false);
    // Bounce
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_TOHAND);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_cost(d, e3, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    effect::set_target(d, e3, Some(cost_target(thtg)));
    effect::set_operation(d, e3, Some(operation(thop)));
    card::register_effect(d, c, e3, false);
}
fn valcheck(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    let g = card::get_material(d, c);
    if group::is_exists(d, &g, &|d, x| card::is_set_card(d, x, &[SET_EXOSISTER as u16]), 1, NO_EX) {
        card::register_flag_effect(d, c, ID, RESET_EVENT | (RESETS_STANDARD_PHASE_END & !RESET_TOFIELD), 0, 1, 0, 0);
    }
    0
}
fn limcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_xyz_summoned(d, c) && card::get_flag_effect(d, c, ID) > 0
}
fn limop(d: &mut Duel, a: &EffArgs) {
    // Neither player can activate effects from the GY
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_description(d, e1, aux::stringid(ID, 2));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    effect::set_target_range(d, e1, 1, 1);
    effect::set_value_fn(d, e1, lua_fn(|d, a: Args| effect::get_activate_location(d, a.e(1)) == LOCATION_GRAVE as u32));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && card::is_able_to_hand(d, chkc, None);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
    }
}
