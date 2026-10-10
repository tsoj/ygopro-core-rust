//! Sky Striker Mecha - Adil Saber
use super::prelude::*;

pub const ID: u32 = 61151074;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SKY_STRIKER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Make 1 "Sky Striker" monster on the field gain 1500 ATK
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(e1cond)));
    effect::set_cost(d, e1, Some(cost::self_discard()));
    effect::set_target(d, e1, Some(cost_target(discardatktg)));
    effect::set_operation(d, e1, Some(operation(discardatkop)));
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    card::register_effect(d, c, e1, false);
    // Equip this card to a "Sky Striker" Link Monster as an Equip Card that gives it 1500 ATK
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_EQUIP + CATEGORY_ATKCHANGE);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_target(d, e2, Some(cost_target(eqatktg)));
    effect::set_operation(d, e2, Some(operation(eqatkop)));
    card::register_effect(d, c, e2, false);
}
/// `Duel.IsBattlePhase() and aux.StatChangeDamageStepCondition()`
fn e1cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_battle_phase(d, None) && aux::stat_change_damage_step_condition(d)
}
fn sky_striker_set(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_SKY_STRIKER as u16])
}
fn discardatktg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_set_card(d, chkc, &[SET_SKY_STRIKER as u16]) && card::is_faceup(d, chkc);
    }
    let f = aux::faceup_filter(sky_striker_set);
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&f), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATKDEF as u64);
    let g = duel::select_target(d, a.tp, Some(&f), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, g, 1, a.tp, 1500);
    true
}
fn discardatkop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // It gains 1500 ATK until the end of this turn
        let c = effect::get_handler(d, a.e).nil();
        card::update_attack(d, tc, 1500, Some(RESETS_STANDARD_PHASE_END), Some(c), None);
    }
}
fn eqatkfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_SKY_STRIKER as u16]) && card::is_link_monster(d, c)
}
fn eqatktg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && eqatkfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&eqatkfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // The Lua passes the undefined global s.eqpatkfilter (nil) as the filter
    duel::select_target(d, a.tp, None, a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
fn eqatkop(d: &mut Duel, a: &EffArgs) {
    if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, c, a.e) && card::is_relate_to_effect(d, tc, a.e) && duel::equip(d, a.tp, c, tc, true, false) {
        // Equip limit
        let e0 = effect::create_effect(d, c);
        effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_code(d, e0, EFFECT_EQUIP_LIMIT);
        // function(e,c) return c==tc end
        effect::set_value_fn(d, e0, lua_fn(move |_d, a: Args| a.c(1) == tc));
        effect::set_reset(d, e0, RESET_EVENT + RESETS_STANDARD, 1);
        card::register_effect(d, c, e0, false);
        // The equipped monster gains 1500 ATK
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_value(d, e1, 1500);
        effect::set_reset(d, e1, RESET_EVENT + RESETS_STANDARD, 1);
        card::register_effect(d, c, e1, false);
    }
}
