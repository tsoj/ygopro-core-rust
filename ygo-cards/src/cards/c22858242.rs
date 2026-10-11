//! Zeman the Ape King
use super::prelude::*;

pub const ID: u32 = 22858242;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro summon
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_attribute_ctx(d, c, ATTRIBUTE_DARK, sc, st, Some(tp)))),
        1,
        1,
        Some(synchro::non_tuner_ex(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_BEAST, sc, st, Some(tp)))),
        1,
        1,
        None,
        None,
        None,
        None,
        None,
    );
    card::enable_revive_limit(d, c);
    // actlimit
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, 0, 1);
    effect::set_value_fn(d, e1, lua_fn(aclimit));
    effect::set_condition(d, e1, Some(condition(actcon)));
    card::register_effect(d, c, e1, false);
    // negate attack
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_condition(d, e2, Some(condition(condition_fn)));
    effect::set_cost(d, e2, Some(cost_target(cost_fn)));
    effect::set_target(d, e2, Some(cost_target(target)));
    effect::set_operation(d, e2, Some(operation(activate)));
    card::register_effect(d, c, e2, false);
}
fn aclimit(d: &mut Duel, a: Args) -> bool {
    effect::is_has_type(d, a.e(1), EFFECT_TYPE_ACTIVATE)
}
fn actcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_attacker(d) == effect::get_handler(d, a.e)
}
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    let first = group::get_first(d, &eg).nil();
    card::is_controler(d, first, 1 - a.tp)
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_monster(d, c) && card::is_able_to_grave_as_cost(d, c)
}
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE | LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&cfilter), a.tp, LOCATION_MZONE | LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    duel::send_to_grave(d, &g, REASON_COST, None, None);
    true
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tg = duel::get_attacker(d);
    if let Some(chkc) = a.chkc() {
        return Some(chkc) == tg;
    }
    if a.chk == Some(0) {
        let tg = tg.nil();
        return card::is_on_field(d, tg) && card::is_can_be_effect_target(d, tg, Some(a.e));
    }
    duel::set_target_card(d, tg.nil());
    true
}
fn activate(d: &mut Duel, _a: &EffArgs) {
    duel::negate_attack(d);
}
