//! CXyz Battleship Cherry Blossom
use super::prelude::*;

pub const ID: u32 = 76419637;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[40424929], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    xyz::add_procedure(d, c, None, Some(4), 4, None, None, None, None, false, None);
    card::enable_revive_limit(d, c);
    // discard
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_HANDES);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(dccon)));
    effect::set_target(d, e1, Some(cost_target(dctg)));
    effect::set_operation(d, e1, Some(operation(dcop)));
    card::register_effect(d, c, e1, false);
    // damage
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(damcon)));
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    effect::set_target(d, e2, Some(cost_target(damtg)));
    effect::set_operation(d, e2, Some(operation(damop)));
    card::register_effect(d, c, e2, false);
}
fn dccon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    duel::is_turn_player(d, 1 - tp) && duel::get_field_group_count(d, tp, 0, LOCATION_HAND) > duel::get_field_group_count(d, tp, LOCATION_HAND, 0)
}
fn dctg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return true;
    }
    duel::set_target_player(d, 1 - tp);
    duel::set_target_param(d, 1);
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, 1 - tp, 1);
    true
}
fn dcop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let _ = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::discard_hand(d, 1 - tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
}
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    group::is_exists(d, &card::get_overlay_group(d, c), &|d, x| card::is_code(d, x, &[40424929]), 1, NO_EX)
}
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return true;
    }
    let ct = duel::get_field_group_count(d, tp, LOCATION_ONFIELD, LOCATION_ONFIELD);
    duel::set_target_player(d, 1 - tp);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, ct * 300);
    true
}
fn damop(d: &mut Duel, _a: &EffArgs) {
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    let ct = duel::get_field_group_count(d, p, LOCATION_ONFIELD, LOCATION_ONFIELD);
    duel::damage(d, p, (ct * 300) as i64, REASON_EFFECT, false, None);
}
