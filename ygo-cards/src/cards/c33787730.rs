//! Ryzeal Plasma Hole
use super::prelude::*;

pub const ID: u32 = 33787730;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RYZEAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy face-up cards your opponent controls up to the number of "Ryzeal" Xyz monsters you control
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_LEAVE_GRAVE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    effect::set_target(d, e1, Some(cost_target(destg)));
    effect::set_operation(d, e1, Some(operation(desop)));
    card::register_effect(d, c, e1, false);
    // Xyz Summon using monsters you control, including a "Ryzeal" monster
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(xyztg)));
    effect::set_operation(d, e2, Some(operation(xyzop)));
    card::register_effect(d, c, e2, false);
}
fn desconfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_RYZEAL as u16]) && card::is_type(d, c, TYPE_XYZ) && card::is_faceup(d, c)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e);
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_faceup(d, chkc) && card::is_controler(d, chkc, 1 - a.tp);
    }
    let ct = duel::get_matching_group_count(d, Some(&desconfilter), a.tp, LOCATION_MZONE, 0, NO_EX);
    if a.chk == Some(0) {
        return ct > 0 && duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    let ex: Exclude = match c {
        Some(c) => c.into(),
        None => NO_EX,
    };
    let dg = duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_ONFIELD, 1, ct as u16, false, ex).nil();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg.clone()), dg.len() as u8, a.tp, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, a.tp, 0);
    true
}
fn attachfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_RYZEAL as u16]) && duel::is_existing_matching_card(d, Some(&|d, x| xyzfilter(d, x, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
fn xyzfilter(d: &mut Duel, c: CardId, mc: CardId, tp: u8) -> bool {
    card::is_type(d, c, TYPE_XYZ) && card::is_rank(d, c, &[4]) && card::is_faceup(d, c) && card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), Some(REASON_EFFECT))
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let g = duel::get_target_cards(d, a.e).nil();
    if g.len() > 0 && duel::destroy(d, g, REASON_EFFECT, None, None) > 0 {
        let nv = aux::necro_valley_filter(|d, c| attachfilter(d, c, tp));
        if duel::is_existing_matching_card(d, Some(&nv), tp, LOCATION_GRAVE, 0, 1, NO_EX) && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATTACH as u64);
            let mg = duel::select_matching_card(d, tp, Some(&nv), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
            let Some(mc) = group::get_first(d, &mg) else { return };
            duel::hint_selection(d, CardSet::from_card(mc), true);
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
            let xg = duel::select_matching_card(d, tp, Some(&|d, x| xyzfilter(d, x, mc, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
            let xyzc = group::get_first(d, &xg).nil();
            if !card::is_immune_to_effect(d, xyzc, a.e) {
                duel::break_effect(d);
                duel::overlay(d, xyzc, CardSet::from_card(mc), false);
            }
        }
    }
}
fn raizeolmatfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_RYZEAL as u16])
        && card::is_faceup(d, c)
        && duel::is_existing_matching_card(d, Some(&|d, x| card::is_xyz_summonable(d, x, Some(CardSet::from_card(c)), None, 0, 0)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
fn xyztg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| raizeolmatfilter(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
fn xyzop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 3));
    let g = duel::select_matching_card(d, tp, Some(&|d, c| raizeolmatfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    if g.is_empty() {
        return;
    }
    duel::hint_selection(d, g.clone(), true);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let xg = duel::select_matching_card(d, tp, Some(&|d, c| card::is_xyz_summonable(d, c, Some(g.clone()), None, 0, 0)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    if let Some(xyzc) = group::get_first(d, &xg) {
        duel::xyz_summon(d, tp, xyzc, Some(g), None, 0, 0);
    }
}
