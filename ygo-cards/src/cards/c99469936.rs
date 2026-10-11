//! Full Armored Crystalzero Lancer
use super::prelude::*;

pub const ID: u32 = 99469936;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(|d, x, sc, st, tp| card::is_attribute_ctx(d, x, ATTRIBUTE_WATER, sc, st, Some(tp)))),
        Some(6),
        3,
        Some(xyz::alter_filter(ovfilter)),
        Some(aux::stringid(ID, 0)),
        None,
        None,
        false,
        None,
    );
    card::enable_revive_limit(d, c);
    // atk
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_value_fn(d, e1, effect_card_value(atkval));
    card::register_effect(d, c, e1, false);
    // destroy replace
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_target(d, e2, Some(cost_target(reptg)));
    card::register_effect(d, c, e2, false);
    // negate
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_category(d, e3, CATEGORY_DISABLE);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_cost(d, e3, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    effect::set_target(d, e3, Some(cost_target(target)));
    effect::set_operation(d, e3, Some(operation(op)));
    card::register_effect(d, c, e3, false);
}
// function s.ovfilter(c,tp,xyzc)
fn ovfilter(d: &mut Duel, c: CardId, tp: u8, xyzc: CardId) -> bool {
    card::is_faceup(d, c) && card::get_rank(d, c) == 5 && card::is_attribute_ctx(d, c, ATTRIBUTE_WATER, Some(xyzc), SUMMON_TYPE_XYZ as u64, Some(tp))
}
// function s.atkval(e,c) return c:GetOverlayCount()*500 end
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    card::get_overlay_count(d, c) as i64 * 500
}
// function s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return !card::is_reason(d, c, REASON_REPLACE) && card::check_remove_overlay_card(d, c, a.tp, 1, REASON_EFFECT);
    }
    if duel::select_effect_yes_no(d, a.tp, c, Some(96)) {
        card::remove_overlay_card(d, c, a.tp, 1, 1, REASON_EFFECT);
        return true;
    }
    false
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&card::is_negatable_monster), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, Some(&card::is_negatable_monster), a.tp, 0, LOCATION_MZONE, NO_EX);
    let _ = group::get_first(d, &g);
    for tc in group::iter(d, &g) {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_DISABLE);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e1, false);
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e2, false);
    }
}
