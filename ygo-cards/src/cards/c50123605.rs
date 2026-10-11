//! Heavy Cavalry of the Indestructible Insects
use super::prelude::*;

pub const ID: u32 = 50123605;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);

    // Fusion Materials: 1 Level 5 or higher Insect monster + 1 Insect monster
    // Fusion.AddProcMix(c,true,true,s.matfilter,aux.FilterBoolFunctionEx(Card.IsRace,RACE_INSECT))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| matfilter(d, c, fc, st, tp))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_race_ctx(d, c, RACE_INSECT, Some(fc), st, Some(tp)))),
        ],
    );

    // Cannot be destroyed by your opponent's card effects
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_value_fn(d, e1, aux::indoval());
    card::register_effect(d, c, e1, false);

    // Banish 2 monsters on the field, including an Insect monster you control
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_REMOVE);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(rmvcon)));
    effect::set_target(d, e2, Some(cost_target(rmvtg)));
    effect::set_operation(d, e2, Some(operation(rmvop)));
    card::register_effect(d, c, e2, false);
}
/// `s.matfilter(c,fc,sumtype,tp)`
fn matfilter(d: &mut Duel, c: CardId, fc: CardId, st: u64, tp: u8) -> bool {
    card::is_level_above(d, c, 5) && card::is_race_ctx(d, c, RACE_INSECT, Some(fc), st, Some(tp))
}
fn rmvcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, NO_EX);
    g.len() == group::filter_count(d, &g, &|d, c| card::is_race(d, c, RACE_INSECT), NO_EX)
}
fn rmvfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    card::is_able_to_remove(d, c, None, None, None) && card::is_can_be_effect_target(d, c, Some(e))
}
fn insectfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_race(d, c, RACE_INSECT) && card::is_controler(d, c, tp) && card::is_faceup(d, c)
}
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    (group::is_exists(d, sg, &|d, c| insectfilter(d, c, tp), 1, NO_EX), false)
}
fn rmvtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chkc().is_some() {
        return false;
    }
    let e = a.e;
    let g = duel::get_matching_group(d, Some(&|d, c| rmvfilter(d, c, e)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    let rs: aux::Rescon = &rescon;
    if a.chk == Some(0) {
        return g.len() >= 2 && matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(rs), 0, tp, None, None, None, false), Err(true));
    }
    let g = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(rs), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, false).unwrap_or_default();
    duel::set_target_card(d, g.clone());
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 2, tp, 0);
    true
}
fn rmvop(d: &mut Duel, a: &EffArgs) {
    let tg = duel::get_target_cards(d, a.e).nil();
    if !tg.is_empty() {
        duel::remove(d, tg, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
