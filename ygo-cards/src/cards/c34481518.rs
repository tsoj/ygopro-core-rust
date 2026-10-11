//! Grenosaurus Giga-Cannon
use super::prelude::*;

pub const ID: u32 = 34481518;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure
    xyz::add_procedure(d, c, None, Some(4), 2, None, None, None, None, false, None);
    // Inflict 1000 damage to your opp
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DAMAGE + CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYED);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_condition(d, e1, Some(condition(damcon)));
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    effect::set_target(d, e1, Some(cost_target(damtg)));
    effect::set_operation(d, e1, Some(operation(damop)));
    card::register_effect(d, c, e1, false);
    // Destroy 1 card on the field
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_DESTROYED);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(descon)));
    effect::set_target(d, e2, Some(cost_target(destg)));
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
}
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    if !(card::is_previous_controler(d, c, 1 - tp) && card::is_location(d, c, LOCATION_GRAVE)) {
        return false;
    }
    let bc = card::get_reason_card(d, c).nil();
    if card::is_relate_to_battle(d, bc) {
        card::is_controler(d, bc, tp) && card::is_race(d, bc, RACE_DINOSAUR)
    } else {
        card::is_previous_controler(d, bc, tp) && (card::get_previous_race_on_field(d, c) & RACE_DINOSAUR) > 0
    }
}
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, tp), 1, NO_EX)
}
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_target_player(d, 1 - a.tp);
    duel::set_target_param(d, 1000);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 1000);
    true
}
fn damop(d: &mut Duel, a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let amount = info[1].int();
    if duel::damage(d, p, amount, REASON_EFFECT, false, None) > 0 {
        let c = effect::get_handler(d, a.e).nil();
        if !(card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c)) {
            return;
        }
        duel::break_effect(d);
        // Gains 1000 ATK
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_COPY_INHERIT, 0);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_value(d, e1, 1000);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        card::register_effect(d, c, e1, false);
    }
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, c, LOCATION_MZONE) && card::is_xyz_summoned(d, c)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    let g = duel::select_target(d, tp, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, tp, 0);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, 1000);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && duel::destroy(d, tc, REASON_EFFECT, None, None) > 0 {
        duel::damage(d, 1 - tp, 1000, REASON_EFFECT, false, None);
    }
}
