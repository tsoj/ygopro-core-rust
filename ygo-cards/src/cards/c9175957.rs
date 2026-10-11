//! Tally-ho! Springans
use super::prelude::*;

pub const ID: u32 = 9175957;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SPRINGANS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Search 1 "Springans" monster
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E | TIMING_MAIN_END));
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(thcost)));
    effect::set_target(d, e1, Some(cost_target(thtg)));
    effect::set_operation(d, e1, Some(operation(thop)));
    card::register_effect(d, c, e1, false);
    // Add this card to your hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_target(d, e2, Some(cost_target(gthtg)));
    effect::set_operation(d, e2, Some(operation(gthop)));
    card::register_effect(d, c, e2, false);
}
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    if duel::check_remove_overlay_card(d, a.tp, 1, 0, 1, REASON_COST, None)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2))
        && duel::remove_overlay_card(d, a.tp, 1, 0, 1, 3, REASON_COST, None) > 0
    {
        let n = duel::get_operated_group(d).len() as u32;
        duel::set_target_param(d, n);
    }
    true
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_SPRINGANS as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    let ct = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].clone();
    if !ct.is_nil() && ct.int() > 0 {
        duel::set_possible_operation_info(
            d,
            0,
            CATEGORY_SPECIAL_SUMMON,
            None,
            ct.int() as u8,
            a.tp,
            (LOCATION_HAND | LOCATION_GRAVE) as i32,
        );
    }
    true
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_SPRINGANS as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if g.is_empty() {
        return;
    }
    if duel::send_to_hand(d, &g, None, REASON_EFFECT, None) == 0 {
        return;
    }
    let tc = group::get_first(d, &g).nil();
    if !card::is_location(d, tc, LOCATION_HAND) {
        return;
    }
    duel::confirm_cards(d, 1 - tp, &g);
    let ct_v = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].clone();
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 as i64;
    if ct_v.is_nil() {
        return;
    }
    let ct = ct_v.int();
    if ct <= 0 || ft < ct || (ct > 1 && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some()) {
        return;
    }
    let sg = duel::get_matching_group(
        d,
        Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, tp))),
        tp,
        LOCATION_HAND | LOCATION_GRAVE,
        0,
        NO_EX,
    );
    if (sg.len() as i64) < ct || !duel::select_yes_no(d, tp, aux::stringid(ID, 3)) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let ssg = group::select(d, &sg, tp, ct as u16, ct as u16, false, NO_EX).nil();
    if ssg.len() as i64 == ct {
        duel::break_effect(d);
        duel::special_summon(d, &ssg, 1, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
fn tgfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ) && card::check_remove_overlay_card(d, c, tp, 1, REASON_EFFECT)
}
fn gthtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && tgfilter(d, chkc, tp);
    }
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None)
            && duel::is_existing_target(d, Some(&|d, x| tgfilter(d, x, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DEATTACHFROM as u64);
    let _ = duel::select_target(d, tp, Some(&|d, x| tgfilter(d, x, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(c.into()), 1, tp, LOCATION_GRAVE as i32);
    true
}
fn gthop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let tc = duel::first_target(d).nil();
    if !card::is_relate_to_effect(d, tc, a.e) || card::remove_overlay_card(d, tc, tp, 1, 1, REASON_EFFECT) < 1 {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, c);
    }
}
