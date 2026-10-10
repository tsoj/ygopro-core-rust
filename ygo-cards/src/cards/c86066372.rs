//! Accesscode Talker
use super::prelude::*;

pub const ID: u32 = 86066372;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Link Summon
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_EFFECT, sc, st, Some(tp)))),
        2,
        None,
        None,
        None,
        None,
    );
    card::enable_revive_limit(d, c);
    // Check materials for a Link Monster
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e0, EFFECT_MATERIAL_CHECK);
    effect::set_value_fn(d, e0, effect_card_value(valcheck));
    card::register_effect(d, c, e0, false);
    // Increase ATK
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e1, Some(condition(atkcon)));
    effect::set_target(d, e1, Some(cost_target(atktg)));
    effect::set_operation(d, e1, Some(operation(atkop)));
    effect::set_label_object(d, e1, Value::Effect(e0));
    card::register_effect(d, c, e1, false);
    // Destroy 1 card the opponent controls
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_cost(d, e2, Some(cost_target(descost)));
    effect::set_target(d, e2, Some(cost_target(destg)));
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
    // Register Attributes used
    aux::global_check(d, ID, move |d| {
        set_s_var(d, ID, 0, Value::Int(0));
        set_s_var(d, ID, 1, Value::Int(0));
        let ge1 = effect::create_effect(d, c);
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, ge1, EVENT_PHASE + PHASE_END);
        effect::set_count_limit(d, ge1, 1, 0, 0, 0);
        effect::set_condition(d, ge1, Some(condition(resetop)));
        duel::register_effect(d, ge1, 0);
    });
}
/// `s.valcheck(e,c)`: label 1 when a material is a Link Monster
fn valcheck(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    let g = card::get_material(d, c);
    effect::set_label(d, e, &[0]);
    if group::is_exists(d, &g, &|d, tc| card::is_link_monster(d, tc), 1, NO_EX) {
        effect::set_label(d, e, &[1]);
    }
    // the function returns nothing: the core reads nil as 0
    0
}
/// `s.resetop(e,tp,eg,ep,ev,re,r,rp)`
fn resetop(d: &mut Duel, _a: &EffArgs) -> bool {
    set_s_var(d, ID, 0, Value::Int(0));
    set_s_var(d, ID, 1, Value::Int(0));
    false
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_link_summoned(d, c) {
        return false;
    }
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    effect::get_label(d, lo) == 1
}
fn atkfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    card::is_type(d, c, TYPE_LINK)
        && (card::is_location(d, c, LOCATION_GRAVE) || (card::is_location(d, c, LOCATION_REMOVED) && card::is_faceup(d, c)))
        && card::is_can_be_effect_target(d, c, Some(e))
}
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    let c = effect::get_handler(d, e).nil();
    let mat = card::get_material(d, c);
    let g = group::filter(d, &mat, &|d, tc| atkfilter(d, tc, e), NO_EX);
    if let Some(chkc) = a.chkc() {
        let mat = card::get_material(d, c);
        return group::is_contains(&mat, chkc) && atkfilter(d, chkc, e);
    }
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    let mat = card::get_material(d, c);
    let tg = group::filter_select(d, &mat, tp, &|d, tc| atkfilter(d, tc, e), 1, 1, false, NO_EX).nil();
    duel::set_target_card(d, tg);
    duel::set_chain_limit(d, lua_fn(chlimit));
    true
}
/// `s.chlimit(e,ep,tp)`
fn chlimit(_d: &mut Duel, a: Args) -> bool {
    a.p(2) == a.p(1)
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let c = effect::get_handler(d, e).nil();
    let tc = duel::first_target(d);
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, e) && card::is_faceup(d, tc.nil()) && card::is_relate_to_effect(d, tc.nil(), e) {
        // Increase ATK
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        let link = card::get_link(d, tc.nil()) as i64;
        effect::set_value(d, e1, link * 1000);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        card::register_effect(d, c, e1, false);
    }
}
/// `s.costfilter(c,e,tp)`
fn costfilter(d: &mut Duel, c: CardId, _e: EffectId, tp: u8) -> bool {
    let attr = card::get_attribute(d, c);
    card::is_link_monster(d, c)
        && card::is_able_to_remove_as_cost(d, c, None)
        && (s_var(d, ID, tp as u64).int() & attr as i64) == 0
}
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| costfilter(d, c, e, tp)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| costfilter(d, c, e, tp)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let first = group::get_first(d, &g).nil();
    let attr = card::get_attribute(d, first);
    effect::set_label(d, e, &[attr as i64]);
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, None, a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    let g = duel::get_matching_group(d, None, a.tp, 0, LOCATION_ONFIELD, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    duel::set_chain_limit(d, lua_fn(chlimit));
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    let att = effect::get_label(d, e);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    let g = duel::select_matching_card(d, tp, None, tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::hint_selection(d, g.clone(), true);
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
    let cur = s_var(d, ID, tp as u64).int();
    set_s_var(d, ID, tp as u64, Value::Int(cur | att));
    for s in aux::get_attribute_strings(att as u64) {
        let h = effect::get_handler(d, e).nil();
        card::register_flag_effect(d, h, 0, RESETS_STANDARD_PHASE_END, EFFECT_FLAG_CLIENT_HINT, 1, 0, s);
    }
}
