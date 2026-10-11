//! Red-Eyes Darkness Metal Dragon (Pre-Errata)
use super::prelude::*;

pub const ID: u32 = 88264988;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon procedure
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_condition(d, e1, Some(lua_fn(spcon1)));
    effect::set_target(d, e1, Some(lua_fn(sptg1)));
    effect::set_operation(d, e1, Some(operation(spop1)));
    card::register_effect(d, c, e1, false);
    // special summon a dragon
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}
fn spfilter(d: &mut Duel, c: CardId, ft: i32) -> bool {
    card::is_faceup(d, c) && card::is_race(d, c, RACE_DRAGON) && card::is_able_to_remove_as_cost(d, c, None) && (ft > 0 || card::get_sequence(d, c) < 5)
}
fn spcon1(d: &mut Duel, a: Args) -> bool {
    let Some(c) = a.card(1) else {
        return true;
    };
    let tp = card::get_controler(d, c);
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    let rg = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, ft)), tp, LOCATION_MZONE, 0, NO_EX);
    ft > -1 && !rg.is_empty() && matches!(aux::select_unselect_group(d, &rg, Some(a.e(0)), tp, Some(1), Some(1), None, 0, tp, None, None, None, false), Err(true))
}
fn sptg1(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    let rg = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, ft)), tp, LOCATION_MZONE, 0, NO_EX);
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(1), Some(1), None, 1, tp, Some(HINTMSG_REMOVE as u64), None, None, true).unwrap_or_default();
    if !g.is_empty() {
        effect::set_label_object(d, e, Value::Group(g));
        return true;
    }
    false
}
fn spop1(d: &mut Duel, a: &EffArgs) {
    if let Some(g) = effect::get_label_object(d, a.e).as_group().cloned() {
        duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    }
}
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_race(d, c, RACE_DRAGON) && !card::is_code(d, c, &[88264978]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE | LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_GRAVE | LOCATION_HAND) as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let f = aux::necro_valley_filter(|d, c| filter(d, c, a.e, a.tp));
    let g = duel::select_matching_card(d, a.tp, Some(&f), a.tp, LOCATION_GRAVE | LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
