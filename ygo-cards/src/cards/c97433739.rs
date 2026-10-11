//! Message in a Bottle
use super::prelude::*;

pub const ID: u32 = 97433739;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SPSUMMON, lua_fn(counterfilter));
}
fn counterfilter(d: &mut Duel, a: Args) -> bool {
    card::is_xyz_summoned(d, a.c(0))
}
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_custom_activity_count(d, ID, a.tp, ACTIVITY_SPSUMMON) == 0;
    }
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET | EFFECT_FLAG_OATH, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_label_object(d, e1, Value::Effect(a.e));
    effect::set_target(d, e1, Some(lua_fn(sumlimit)));
    duel::register_effect(d, e1, a.tp);
    let e2 = effect::create_effect(d, h);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET | EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    effect::set_target_range(d, e2, 1, 0);
    duel::register_effect(d, e2, a.tp);
    true
}
fn sumlimit(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let sumtype = a.int(3);
    sumtype != SUMMON_TYPE_XYZ as i64 && effect::get_label_object(d, e).as_effect() != a.v(6).as_effect()
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_level_above(d, c, 1)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && card::is_can_be_effect_target(d, c, Some(e))
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chkc().is_some() {
        return false;
    }
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, NO_EX);
    if a.chk == Some(0) {
        return duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 2
            && group::get_class_count(d, &g, &|d, c| card::get_level(d, c) as i64) >= 3;
    }
    let tg = aux::select_unselect_group(
        d,
        &g,
        Some(a.e),
        a.tp,
        Some(3),
        Some(3),
        Some(&aux::dpcheck(|d: &mut Duel, c: CardId| card::get_level(d, c) as i64)),
        1,
        a.tp,
        Some(HINTMSG_SPSUMMON as u64),
        None,
        None,
        false,
    )
    .unwrap_or_default();
    duel::set_target_card(d, tg.clone());
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(tg), 3, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    let g = duel::get_target_cards(d, a.e).nil();
    if g.len() != 3 {
        return;
    }
    if (duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 as i64) < g.len() as i64 {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    let _ = group::get_first(d, &g);
    for tc in group::iter(d, &g) {
        if duel::special_summon_step(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) {
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
            effect::set_value(d, e1, 0);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, tc, e1, false);
            let e2 = effect::create_effect(d, c);
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e2, EFFECT_DISABLE);
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, tc, e2, false);
            let e3 = effect::create_effect(d, c);
            effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e3, EFFECT_DISABLE_EFFECT);
            effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, tc, e3, false);
        }
    }
    duel::special_summon_complete(d);
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD | EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_operation(d, e1, Some(operation(checkop2)));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD | EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_operation(d, e2, Some(operation(lpop)));
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e2, a.tp);
    effect::set_label_object(d, e1, Value::Effect(e2));
}
fn checkop2(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg.clone().nil();
    let first = group::get_first(d, &eg).nil();
    if card::is_xyz_summoned(d, first) {
        let lo = effect::get_label_object(d, a.e).as_effect().nil();
        effect::reset(d, lo);
        effect::reset(d, a.e);
    }
}
fn lpop(d: &mut Duel, a: &EffArgs) {
    let lp = duel::get_lp(d, a.tp);
    duel::set_lp(d, a.tp, lp - 4000);
}
