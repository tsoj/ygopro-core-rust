//! Recardination
use super::prelude::*;

pub const ID: u32 = 73271204;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FLOWER_CARDIAN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // mill
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_condition(d, e2, Some(condition(milcon)));
    effect::set_target(d, e2, Some(cost_target(miltg)));
    effect::set_operation(d, e2, Some(operation(milop)));
    card::register_effect(d, c, e2, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_FLOWER_CARDIAN as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), 1, 0, 0);
    true
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_FLOWER_CARDIAN as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
fn activate(d: &mut Duel, a: &EffArgs) {
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) && duel::send_to_hand(d, tc, None, REASON_EFFECT, None) != 0 && card::is_location(d, tc, LOCATION_HAND) {
            let tg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, NO_EX);
            if !tg.is_empty() && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
                duel::break_effect(d);
                duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
                let sc = group::select(d, &tg, a.tp, 1, 1, false, NO_EX).nil();
                duel::special_summon(d, sc, 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None);
            }
        }
    }
}
fn milcon(d: &mut Duel, a: &EffArgs) -> bool {
    let Some(re) = a.re else {
        return false;
    };
    let rc = effect::get_handler(d, re).nil();
    let h = effect::get_handler(d, a.e).nil();
    card::is_reason(d, h, REASON_EFFECT) && card::is_set_card(d, rc, &[SET_FLOWER_CARDIAN as u16]) && card::is_monster(d, rc)
}
fn miltg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) > 4;
    }
    true
}
fn milfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
fn milop(d: &mut Duel, a: &EffArgs) {
    if duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) < 5 {
        return;
    }
    duel::confirm_decktop(d, a.tp, 5);
    let mut g = duel::get_decktop_group(d, a.tp, 5);
    if !g.is_empty() {
        duel::disable_shuffle_check(d, true);
        if group::is_exists(d, &g, &milfilter, 1, NO_EX) && duel::select_yes_no(d, a.tp, aux::stringid(ID, 3)) {
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
            let sg = group::filter_select(d, &g, a.tp, &milfilter, 1, 1, false, NO_EX).nil();
            duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
            duel::confirm_cards(d, 1 - a.tp, sg.clone());
            duel::shuffle_hand(d, a.tp);
            group::remove_card(&mut g, &sg);
        }
        duel::sort_decktop(d, a.tp, a.tp, g.len() as u16);
    }
}
