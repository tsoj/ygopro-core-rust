//! Sinful Spoils of Doom - Rciela
use super::prelude::*;

pub const ID: u32 = 16240772;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Apply effects
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_TOGRAVE + CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, Some(TIMINGS_CHECK_MONSTER_E | TIMING_DAMAGE_STEP));
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_level_above(d, c, 7) && card::is_race(d, c, RACE_SPELLCASTER)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && tgfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tgfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    let g = duel::select_target(d, a.tp, Some(&tgfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, g, 1, a.tp, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, 1 - a.tp, LOCATION_MZONE as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if !(card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc)) {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_immune_to_effect(d, tc, a.e) {
        // Unaffected by other monsters' effects
        let e1 = effect::create_effect(d, c);
        effect::set_description(d, e1, 3101);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
        effect::set_range(d, e1, LOCATION_MZONE);
        effect::set_value_fn(d, e1, lua_fn(|d, a: Args| {
            let te = a.e(1);
            let owner = effect::get_owner(d, te).nil();
            let h = effect::get_handler(d, a.e(0)).nil();
            effect::is_monster_effect(d, te) && owner != h
        }));
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e1, false);
        let resetcount: u16 = if duel::get_current_phase(d) <= PHASE_STANDBY { 2 } else { 1 };
        let prevturn = duel::get_turn_count(d, None);
        // Send it to the GY during the Standby Phase of the next turn
        aux::delayed_operation(
            d,
            tc,
            PHASE_STANDBY,
            ID,
            a.e,
            a.tp,
            Some(std::sync::Arc::new(|d: &mut Duel, ag: &CardSet, _a: &EffArgs| {
                duel::send_to_grave(d, ag.clone(), REASON_EFFECT, None, None);
                true
            })),
            Some(std::sync::Arc::new(move |d: &mut Duel, _ag: &CardSet, _a: &EffArgs| duel::get_turn_count(d, None) != prevturn)),
            None,
            Some(resetcount),
            Some(aux::stringid(ID, 1)),
            None,
        );
    }
    let tc_atk = card::get_attack(d, tc);
    if tc_atk == 0 {
        return;
    }
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, NO_EX);
    if g.is_empty() {
        return;
    }
    duel::break_effect(d);
    let mut dg = CardSet::new();
    for oc in group::iter(d, &g) {
        let preatk = card::get_attack(d, oc);
        // Decrease ATK
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
        effect::set_value(d, e2, -(tc_atk as i64));
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, oc, e2, false);
        if preatk != 0 && card::get_attack(d, oc) == 0 {
            group::add_card(&mut dg, &CardSet::from_card(oc));
        }
    }
    if dg.is_empty() {
        return;
    }
    duel::break_effect(d);
    duel::destroy(d, dg, REASON_EFFECT, None, None);
}
