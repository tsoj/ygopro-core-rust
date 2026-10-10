//! Heretical Phobos Covos
use super::prelude::*;

pub const ID: u32 = 76078185;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure
    xyz::add_procedure(d, c, None, Some(4), 2, None, None, None, None, false, None);
    // Negate the effects of an opponent's monster
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DISABLE + CATEGORY_CONTROL);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(|d, a| !quickcon(d, a))));
    effect::set_target(d, e1, Some(cost_target(distg)));
    effect::set_operation(d, e1, Some(operation(disop)));
    card::register_effect(d, c, e1, false);
    // This is a Quick Effect if this card has an Illusion monster as material
    let e2 = effect::clone(d, e1);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E | TIMING_BATTLE_END));
    effect::set_condition(d, e2, Some(condition(quickcon)));
    card::register_effect(d, c, e2, false);
    // If this card battles a monster, neither can be destroyed by that battle
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e3, Some(effect_card_fn(indestg)));
    effect::set_value(d, e3, 1);
    card::register_effect(d, c, e3, false);
    // Keep track of any monster that has battled in a given turn
    aux::global_check(d, ID, |d| {
        let ge1 = effect::create_effect(d, c);
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, ge1, EVENT_BATTLED);
        effect::set_operation(d, ge1, Some(operation(checkop)));
        duel::register_effect(d, ge1, 0);
    });
}
fn checkop(d: &mut Duel, _a: &EffArgs) {
    let attacker = duel::get_attacker(d).nil();
    card::register_flag_effect(d, attacker, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    let ac = duel::get_attack_target(d);
    if let Some(ac) = ac {
        card::register_flag_effect(d, ac, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    }
}
fn quickcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let g = card::get_overlay_group(d, c);
    group::is_exists(d, &g, &|d, c| card::is_race(d, c, RACE_ILLUSION), 1, NO_EX)
}
fn disfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_type(d, c, TYPE_EFFECT) && card::is_negatable_monster(d, c)
}
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && disfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&disfilter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_NEGATE as u64);
    let g = duel::select_target(d, a.tp, Some(&disfilter), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, g.clone(), 1, a.tp, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_CONTROL, g, 1, a.tp, 0);
    true
}
fn disop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if !(card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && card::is_can_be_disabled_by_effect(d, tc, a.e, true)) {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    card::negate_effects(d, tc, c, None, false, None);
    duel::adjust_instantly(d, Some(tc));
    if card::has_flag_effect(d, tc, ID, None) && card::is_controler_can_be_changed(d, tc, false, Some(0xff)) && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
        duel::break_effect(d);
        duel::get_control(d, tc, a.tp, PHASE_END as u16, 1, None, None);
    }
}
fn indestg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let handler = effect::get_handler(d, e).nil();
    c == handler || Some(c) == card::get_battle_target(d, handler)
}
