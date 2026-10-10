//! Gruesome Grave Squirmer
use super::prelude::*;

pub const ID: u32 = 24215921;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_YUBEL, ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card from your hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(selfcond)));
    effect::set_target(d, e1, Some(cost_target(selfsptg)));
    effect::set_operation(d, e1, Some(operation(selfspop)));
    card::register_effect(d, c, e1, false);
    // Special Summon 1 Fiend monster with 0 ATK/DEF from your hand or GY
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(hgysptg)));
    effect::set_operation(d, e2, Some(operation(hgyspop)));
    card::register_effect(d, c, e2, false);
}
fn selfcond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_FIEND)), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, a.tp, LOCATION_ONFIELD as i32);
    true
}
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    (card::is_code(d, c, &[CARD_YUBEL]) || card::lists_code(d, c, &[CARD_YUBEL])) && card::is_faceup(d, c)
}
fn selfspop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tp = a.tp;
    if card::is_relate_to_effect(d, c, a.e)
        && duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0
        && duel::is_existing_matching_card(d, Some(&desfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
    {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        let g = duel::select_matching_card(d, tp, Some(&desfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
        if !g.is_empty() {
            duel::hint_selection(d, g.clone(), true);
            duel::break_effect(d);
            duel::destroy(d, g, REASON_EFFECT, None, None);
        }
    }
}
fn hgyspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_race(d, c, RACE_FIEND)
        && card::is_attack(d, c, &[0])
        && card::is_defense(d, c, &[0])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && !card::is_code(d, c, &[ID])
}
fn hgysptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        let h = effect::get_handler(d, e).nil();
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| hgyspfilter(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, h.into());
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
fn hgyspop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 == 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(|d, c| hgyspfilter(d, c, e, tp))), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
