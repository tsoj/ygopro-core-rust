//! Sacred Beasts Combined Assault
use super::prelude::*;

pub const ID: u32 = 50147815;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SACRED_BEAST, SET_PHANTASM], ..ScriptInfo::DEFAULT };

/// `local fusion_params={fusfilter=aux.FilterBoolFunction(Card.IsSetCard,SET_PHANTASM)}`
fn fusion_params() -> fusion::SummonEff {
    fusion::SummonEff { fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_PHANTASM as u16]))), ..Default::default() }
}

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Sacred Beast" monster from your hand or GY in Defense Position, then if you control 2 or more "Sacred Beast" monsters whose original Levels are 10, you can negate the effects of 1 face-up card your opponent controls, and if you do, destroy it
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DISABLE + CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    card::register_effect(d, c, e1, false);
    // You can banish this card from your GY; Fusion Summon 1 "Phantasm" Fusion Monster from your Extra Deck, using monsters from your hand or field
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_cost(d, e2, Some(cost::self_banish()));
    let tg = fusion::summon_eff_tg(d, fusion_params());
    effect::set_target(d, e2, Some(tg));
    let op = fusion::summon_eff_op(d, fusion_params());
    effect::set_operation(d, e2, Some(op));
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    card::register_effect(d, c, e2, false);
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_SACRED_BEAST as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, true, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
fn sacredbeastfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_SACRED_BEAST as u16]) && card::is_original_level(d, c, 10) && card::is_faceup(d, c)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    let dg = duel::get_matching_group(d, Some(&card::is_negatable), tp, 0, LOCATION_ONFIELD, NO_EX);
    duel::set_possible_operation_info(d, 0, CATEGORY_DISABLE, Some(dg.clone()), 1, tp, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), 1, tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 == 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, e, tp))),
        tp,
        LOCATION_HAND | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    if !g.is_empty()
        && duel::special_summon(d, &g, 0, tp, tp, false, true, POS_FACEUP_DEFENSE as u8, None) > 0
        && duel::is_existing_matching_card(d, Some(&sacredbeastfilter), tp, LOCATION_MZONE, 0, 2, NO_EX)
        && duel::is_existing_matching_card(d, Some(&card::is_negatable), tp, 0, LOCATION_ONFIELD, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
    {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NEGATE as u64);
        let sc = duel::select_matching_card(d, tp, Some(&card::is_negatable), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
        let Some(sc) = group::get_first(d, &sc) else { return };
        duel::hint_selection(d, sc, true);
        if card::is_can_be_disabled_by_effect(d, sc, e, true) {
            duel::break_effect(d);
            // Negate the effects of 1 face-up card your opponent controls, and if you do, destroy it
            let h = effect::get_handler(d, e).nil();
            card::negate_effects(d, sc, h, Some(RESET_EVENT | RESETS_STANDARD), true, None);
            duel::adjust_instantly(d, Some(sc));
            if card::is_disabled(d, sc) {
                duel::destroy(d, sc, REASON_EFFECT, None, None);
            }
        }
    }
}
