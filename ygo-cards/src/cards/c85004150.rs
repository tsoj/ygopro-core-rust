//! Digital Bug Rhinosebus
use super::prelude::*;

pub const ID: u32 = 85004150;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Xyz.AddProcedure(c,s.mfilter,7,2,s.ovfilter,aux.Stringid(id,0),Xyz.InfiniteMats,s.xyzop)
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(mfilter)),
        Some(7),
        2,
        Some(xyz::alter_filter(ovfilter)),
        Some(aux::stringid(ID, 0)),
        Some(xyz::INFINITE_MATS),
        Some(lua_fn(xyzop)),
        false,
        None,
    );
    // destroy
    let e2 = effect::create_effect(d, c);
    effect::set_category(d, e2, CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER_E as u32));
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    effect::set_target(d, e2, Some(cost_target(destg)));
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
    //
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e3, EFFECT_PIERCE);
    card::register_effect(d, c, e3, false);
}
fn ovfilter(d: &mut Duel, c: CardId, tp: u8, xyzc: CardId) -> bool {
    card::is_faceup(d, c)
        && card::is_type_ctx(d, c, TYPE_XYZ, Some(xyzc), SUMMON_TYPE_XYZ as u64, Some(tp))
        && (card::get_rank(d, c) == 5 || card::get_rank(d, c) == 6)
        && card::is_race_ctx(d, c, RACE_INSECT, Some(xyzc), SUMMON_TYPE_XYZ as u64, Some(tp))
}
fn xyzop(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    let chk = a.int(2);
    let mc = a.c(3);
    if chk == 0 {
        return card::check_remove_overlay_card(d, mc, tp, 2, REASON_COST);
    }
    card::remove_overlay_card(d, mc, tp, 2, 2, REASON_COST);
    true
}
fn mfilter(d: &mut Duel, c: CardId, xyz: Option<CardId>, sumtype: u64, tp: u8) -> bool {
    card::is_race_ctx(d, c, RACE_INSECT, xyz, sumtype, Some(tp)) && card::is_attribute_ctx(d, c, ATTRIBUTE_LIGHT, xyz, sumtype, Some(tp))
}
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_defense_above(d, c, 0)
}
fn max_defense_group(d: &mut Duel, g: &CardSet) -> CardSet {
    group::get_max_group(d, g, &|d, c| card::get_defense(d, c) as i64).map_or_else(CardSet::new, |(g, _)| g)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&desfilter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    let g = duel::get_matching_group(d, Some(&desfilter), a.tp, 0, LOCATION_MZONE, NO_EX);
    let dg = max_defense_group(d, &g);
    let n = dg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), n, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_matching_group(d, Some(&desfilter), a.tp, 0, LOCATION_MZONE, NO_EX);
    if !g.is_empty() {
        let dg = max_defense_group(d, &g);
        duel::destroy(d, dg, REASON_EFFECT, None, None);
    }
}
