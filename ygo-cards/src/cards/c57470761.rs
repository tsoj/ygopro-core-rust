//! Tyrant Wing
use super::prelude::*;

pub const ID: u32 = 57470761;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_EQUIP);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    effect::set_cost(d, e1, Some(cost::remain_field_cost()));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_race(d, c, RACE_DRAGON)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) && duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_location(d, c, LOCATION_SZONE) || !card::is_relate_to_effect(d, c, a.e) || card::is_status(d, c, STATUS_LEAVE_CONFIRMED) {
        return;
    }
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        duel::equip(d, a.tp, c, tc, true, false);
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_value_fn(d, e1, effect_card_fn(eqlimit));
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, c, e1, false);
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
        effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
        effect::set_value(d, e2, 400);
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, c, e2, false);
        let e3 = effect::clone(d, e2);
        effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
        card::register_effect(d, c, e3, false);
        let e4 = effect::create_effect(d, c);
        effect::set_type(d, e4, EFFECT_TYPE_EQUIP);
        effect::set_code(d, e4, EFFECT_EXTRA_ATTACK_MONSTER);
        effect::set_value(d, e4, 1);
        effect::set_reset(d, e4, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, c, e4, false);
        let e7 = effect::create_effect(d, c);
        effect::set_type(d, e7, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, e7, EVENT_BATTLE_START);
        effect::set_range(d, e7, LOCATION_SZONE);
        effect::set_operation(d, e7, Some(operation(regop)));
        effect::set_reset(d, e7, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, c, e7, false);
        let e8 = effect::create_effect(d, c);
        effect::set_type(d, e8, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, e8, EVENT_EQUIP);
        effect::set_range(d, e8, LOCATION_SZONE);
        effect::set_operation(d, e8, Some(operation(resetop)));
        effect::set_reset(d, e8, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, c, e8, false);
        let e9 = effect::create_effect(d, c);
        effect::set_category(d, e9, CATEGORY_DESTROY);
        effect::set_type(d, e9, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
        effect::set_code(d, e9, EVENT_PHASE + PHASE_END);
        effect::set_range(d, e9, LOCATION_SZONE);
        effect::set_count_limit(d, e9, 1, 0, 0, 0);
        effect::set_condition(d, e9, Some(condition(descon)));
        effect::set_target(d, e9, Some(cost_target(destg)));
        effect::set_operation(d, e9, Some(operation(desop)));
        effect::set_reset(d, e9, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, c, e9, false);
    } else {
        card::cancel_to_grave(d, c, false);
    }
}
fn eqlimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_race(d, c, RACE_DRAGON)
}
fn regop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let ec = card::get_equip_target(d, c).nil();
    if !card::is_relate_to_battle(d, ec) {
        return;
    }
    let bc = card::get_battle_target(d, ec);
    if let Some(bc) = bc {
        if card::is_controler(d, bc, 1 - a.tp) && duel::get_attacker(d) == Some(ec) {
            card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
        }
    }
}
fn resetop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if group::is_contains(&a.eg.clone().nil(), c) {
        card::reset_flag_effect(d, c, ID);
    }
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, c, ID) != 0
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::destroy(d, c, REASON_EFFECT, None, None);
    }
}
