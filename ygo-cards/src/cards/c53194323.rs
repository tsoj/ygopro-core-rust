//! Dark Element
use super::prelude::*;

pub const ID: u32 = 53194323;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GATE_GUARDIAN], listed_names: CARDS_SANGA_KAZEJIN_SUIJIN, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Add 1 "Sanga of the Thunder", "Kazejin", or "Suijin" to the hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_GATE_GUARDIAN as u16]) && card::is_monster(d, c)
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let lp = duel::get_lp(d, a.tp).div_euclid(2);
    duel::pay_lp_cost(d, a.tp, lp as u32);
    true
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32) -> bool {
    if !(card::is_set_card(d, c, &[SET_GATE_GUARDIAN as u16])
        && card::is_monster(d, c)
        && card::is_level_above(d, c, 11)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None))
    {
        return false;
    }
    if card::is_location(d, c, LOCATION_HAND | LOCATION_DECK) {
        ft > 0
    } else {
        duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
    }
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp, ft)), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_EXTRA, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_EXTRA) as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, ft)), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let Some(sc) = group::get_first(d, &sel) else { return };
    let h = effect::get_handler(d, e).nil();
    let code_chk = card::is_code(d, sc, &[26746975]) && card::is_code(d, h, &[ID]);
    if duel::special_summon(d, sc, 0, tp, tp, true, false, POS_FACEUP as u8, None) > 0 && code_chk {
        card::complete_procedure(d, sc);
    }
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, CARDS_SANGA_KAZEJIN_SUIJIN) && card::is_able_to_hand(d, c, None) && (card::is_location(d, c, LOCATION_DECK) || card::is_faceup(d, c))
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_REMOVED) as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK | LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, g);
    }
}
