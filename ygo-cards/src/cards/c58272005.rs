//! Survival of the Fittest
use super::prelude::*;

pub const ID: u32 = 58272005;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_EQUIP);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    effect::set_cost(d, e1, Some(aux::remain_field_cost()));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_race(d, c, RACE_DINOSAUR)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        let e = a.e;
        return effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE) && duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    let _g = duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_location(d, c, LOCATION_SZONE) || !card::is_relate_to_effect(d, c, a.e) || card::is_status(d, c, STATUS_LEAVE_CONFIRMED) {
        return;
    }
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        duel::equip(d, tp, c, tc, true, false);
        // Atkup
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_value(d, e1, 1000);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, c, e1, false);
        // Equip limit
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e2, EFFECT_EQUIP_LIMIT);
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_value_fn(d, e2, effect_card_fn(eqlimit));
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, c, e2, false);
        // chain attack
        let e3 = effect::create_effect(d, c);
        effect::set_description(d, e3, aux::stringid(ID, 0));
        effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
        effect::set_range(d, e3, LOCATION_SZONE);
        effect::set_code(d, e3, EVENT_BATTLE_DESTROYING);
        effect::set_condition(d, e3, Some(condition(atcon)));
        effect::set_operation(d, e3, Some(operation(atop)));
        effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, c, e3, false);
    } else {
        card::cancel_to_grave(d, c, false);
    }
}
fn eqlimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_race(d, c, RACE_DINOSAUR)
}
fn atcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let ec = card::get_equip_target(d, c);
    let eg = a.eg.clone().nil();
    let Some(ec) = ec else {
        return false;
    };
    if !group::is_contains(&eg, ec) {
        return false;
    }
    let bc = card::get_battle_target(d, ec).nil();
    card::is_location(d, bc, LOCATION_GRAVE)
        && card::is_monster(d, bc)
        && card::can_chain_attack(d, ec, Some(2), true)
        && card::is_status(d, ec, STATUS_OPPO_BATTLE)
}
fn atop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let ec = card::get_equip_target(d, c).nil();
    if !card::is_relate_to_battle(d, ec) {
        return;
    }
    duel::chain_attack(d, None);
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_CANNOT_DIRECT_ATTACK);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE, 1);
    card::register_effect(d, ec, e1, false);
}
