//! Morphtronic Telefon
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 38082437;

pub const INFO: ScriptInfo = ScriptInfo { roll_dice: true, listed_series: &[SET_MORPHTRONIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Gain LP then Special Summon 1 "Morphtronic" from the GY
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_RECOVER + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(cond_a)));
    effect::set_target(d, e1, Some(cost_target(target_a)));
    effect::set_operation(d, e1, Some(operation(operation_a)));
    card::register_effect(d, c, e1, false);
    // Exavate and send 1 "Morphtronic" card to the GY
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(cond_d)));
    effect::set_target(d, e2, Some(cost_target(target_d)));
    effect::set_operation(d, e2, Some(operation(operation_d)));
    card::register_effect(d, c, e2, false);
}
fn cond_a(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_attack_pos(d, c)
}
fn cond_d(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_defense_pos(d, c)
}
fn target_a(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_DICE, None, 0, a.tp, 1);
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
fn spfilter(d: &mut Duel, c: CardId, lv: i64, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_MORPHTRONIC as u16]) && card::is_level_below(d, c, lv as u32) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn operation_a(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let res = duel::toss_dice(d, tp, 1, 0)[0] as i64;
    let amount = res * 100;
    if duel::recover(d, tp, amount, REASON_EFFECT, false, None) as i64 != amount || duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 == 0 {
        return;
    }
    let f = aux::necro_valley_filter(|d, c| spfilter(d, c, res, a.e, tp));
    let g = duel::get_matching_group(d, Some(&f), tp, LOCATION_GRAVE, 0, NO_EX);
    if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        if let Some(sc) = group::get_first(d, &sel) {
            duel::break_effect(d);
            duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_MORPHTRONIC as u16]) && card::is_able_to_grave(d, c, None)
}
fn target_d(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, tp, LOCATION_DECK, 0) > 0;
    }
    duel::set_operation_info(d, 0, CATEGORY_DICE, None, 0, tp, 1);
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    true
}
fn operation_d(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::get_field_group_count(d, tp, LOCATION_DECK, 0) == 0 {
        return;
    }
    let res = duel::toss_dice(d, tp, 1, 0)[0] as u32;
    duel::confirm_decktop(d, tp, res);
    let mut dg = duel::get_decktop_group(d, tp, res);
    let mut ct = 0;
    if group::is_exists(d, &dg, &tgfilter, 1, NO_EX) && duel::select_yes_no(d, tp, aux::stringid(ID, 3)) {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        let tg = group::filter_select(d, &dg, tp, &tgfilter, 1, 1, false, NO_EX).nil();
        group::remove_card(&mut dg, &tg);
        duel::disable_shuffle_check(d, true);
        duel::send_to_grave(d, tg, REASON_EFFECT | REASON_EXCAVATE, None, None);
        ct = 1;
    }
    let ac = res as i64 - ct;
    if ac == 0 {
        return;
    }
    if duel::select_option(d, tp, true, &[aux::stringid(ID, 4), aux::stringid(ID, 5)]) == 0 {
        duel::move_to_deck_top(d, dg);
        duel::sort_decktop(d, tp, tp, ac as u16);
    } else {
        duel::move_to_deck_bottom(d, dg);
        duel::sort_deckbottom(d, tp, tp, ac as u16);
    }
}
