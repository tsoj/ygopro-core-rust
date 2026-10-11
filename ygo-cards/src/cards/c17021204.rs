//! Mother Spider
use super::prelude::*;

pub const ID: u32 = 17021204;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon proc
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    effect::set_target(d, e1, Some(lua_fn(sptg)));
    effect::set_operation(d, e1, Some(lua_fn(spop)));
    card::register_effect(d, c, e1, false);
}
fn spfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_position(d, c, POS_FACEUP_DEFENSE) && card::is_able_to_grave_as_cost(d, c)
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::get_race(d, c) != RACE_INSECT
}
fn check(d: &mut Duel, tp: u8) -> bool {
    let g = duel::get_matching_group(d, Some(&card::is_monster), tp, LOCATION_GRAVE, 0, NO_EX);
    !g.is_empty() && !group::is_exists(d, &g, &cfilter, 1, NO_EX)
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    let Some(c) = a.card(1) else {
        return true;
    };
    let tp = card::get_controler(d, c);
    let rg = duel::get_matching_group(d, Some(&spfilter), tp, 0, LOCATION_MZONE, NO_EX);
    matches!(
        aux::select_unselect_group(d, &rg, Some(a.e(0)), tp, Some(2), Some(2), Some(&aux::chkf_mmz(1)), 0, tp, None, None, None, false),
        Err(true)
    ) && check(d, card::get_controler(d, c))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    let rg = duel::get_matching_group(d, Some(&spfilter), tp, 0, LOCATION_MZONE, NO_EX);
    let g = aux::select_unselect_group(
        d,
        &rg,
        Some(a.e(0)),
        tp,
        Some(2),
        Some(2),
        Some(&aux::chkf_mmz(1)),
        1,
        tp,
        Some(HINTMSG_RELEASE as u64),
        None,
        None,
        true,
    )
    .unwrap_or_default();
    if !g.is_empty() {
        effect::set_label_object(d, a.e(0), Value::Group(g));
        return true;
    }
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    let Some(g) = effect::get_label_object(d, a.e(0)).as_group().cloned() else {
        return;
    };
    duel::release(d, g, REASON_COST, None);
}
