//! Super Solar Nutrient
use super::prelude::*;

pub const ID: u32 = 28529976;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
    // Destroy
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EVENT_LEAVE_FIELD);
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
    // Destroy2
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_code(d, e3, EVENT_LEAVE_FIELD);
    effect::set_condition(d, e3, Some(condition(descon2)));
    effect::set_operation(d, e3, Some(operation(desop2)));
    card::register_effect(d, c, e3, false);
}
fn cfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32) -> bool {
    let lv = card::get_level(d, c) as i64;
    lv > 0
        && lv <= 2
        && card::is_race(d, c, RACE_PLANT)
        && (ft > 0 || (card::is_controler(d, c, tp) && card::get_sequence(d, c) < 5))
        && (card::is_controler(d, c, tp) || card::is_faceup(d, c))
        && duel::is_existing_matching_card(d, Some(&|d, x| filter(d, x, lv + 3, e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX)
}
fn filter(d: &mut Duel, c: CardId, lv: i64, e: EffectId, tp: u8) -> bool {
    card::is_level_below(d, c, lv as u32) && card::is_race(d, c, RACE_PLANT) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    if a.chk == Some(0) {
        return ft > -1
            && duel::check_release_group(d, a.tp, Some(&|d, c| cfilter(d, c, a.e, a.tp, ft)), 1, duel::ReleaseArgs::default(), NO_EX, false);
    }
    let rg = duel::select_release_group(d, a.tp, Some(&|d, c| cfilter(d, c, a.e, a.tp, ft)), 1, 1, duel::ReleaseArgs::default(), NO_EX, false).nil();
    let tc = group::get_first(d, &rg).nil();
    let lv = card::get_level(d, tc) as i64 + 3;
    effect::set_label(d, a.e, &[lv]);
    duel::release(d, rg, REASON_COST, None);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let lv = effect::get_label(d, a.e);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| filter(d, x, lv, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if let Some(tc) = group::get_first(d, &g) {
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        card::set_card_target(d, c, tc);
    }
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if let Some(tc) = card::get_first_card_target(d, c) {
        if card::is_location(d, tc, LOCATION_MZONE) {
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
fn descon2(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    match card::get_first_card_target(d, c) {
        Some(tc) => group::is_contains(&a.eg.clone().nil(), tc),
        None => false,
    }
}
fn desop2(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    duel::destroy(d, c, REASON_EFFECT, None, None);
}
