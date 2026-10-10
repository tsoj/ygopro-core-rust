//! Windmill Genex
use super::prelude::*;

pub const ID: u32 = 43925870;

pub const INFO: ScriptInfo = ScriptInfo { material: &[68505803], listed_names: &[68505803], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro summon
    synchro::add_procedure(
        d,
        c,
        Some(aux::filter_summon_code(&[68505803])),
        1,
        1,
        Some(synchro::non_tuner_ex(|d, x, sc, st, tp| card::is_attribute_ctx(d, x, ATTRIBUTE_WIND, sc, st, Some(tp)))),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    card::enable_revive_limit(d, c);
    // atkup
    let e1 = effect::create_effect(d, c);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_value_fn(d, e1, effect_card_value(val));
    card::register_effect(d, c, e1, false);
    // destroy
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_DESTROY);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_cost(d, e2, Some(cost_target(descost)));
    effect::set_target(d, e2, Some(cost_target(destg)));
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
}
fn val(d: &mut Duel, _e: EffectId, _c: CardId) -> i64 {
    duel::get_matching_group_count(d, Some(&|d, x| card::is_facedown(d, x)), 0, LOCATION_SZONE, LOCATION_SZONE, NO_EX) as i64 * 300
}
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| card::is_able_to_grave_as_cost(d, x)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| card::is_able_to_grave_as_cost(d, x)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_facedown(d, c)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_SZONE) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_SZONE, LOCATION_SZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_SZONE, LOCATION_SZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_facedown(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
