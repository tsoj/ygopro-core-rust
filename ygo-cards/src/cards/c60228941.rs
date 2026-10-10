//! Snoww, Unlight of Dark World
use super::prelude::*;

pub const ID: u32 = 60228941;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DARK_WORLD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // search
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let prev = card::get_previous_controler(d, c);
    effect::set_label(d, a.e, &[prev as i64]);
    card::is_previous_location(d, c, LOCATION_HAND) && (a.r & (REASON_DISCARD | REASON_EFFECT)) == (REASON_DISCARD | REASON_EFFECT)
}
fn filter1(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_DARK_WORLD as u16]) && card::is_able_to_hand(d, c, None)
}
fn filter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_GRAVE) && filter2(d, chkc, e, tp);
    }
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    let label = effect::get_label(d, e);
    if a.tp != a.rp
        && tp as i64 == label
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_target(d, Some(&|d, c| filter2(d, c, e, tp)), tp, 0, LOCATION_GRAVE, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
    {
        effect::set_property(d, e, EFFECT_FLAG_CARD_TARGET, 0);
        effect::set_category(d, e, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON);
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let g = duel::select_target(d, tp, Some(&|d, c| filter2(d, c, e, tp)), tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX);
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    } else {
        effect::set_category(d, e, CATEGORY_TOHAND + CATEGORY_SEARCH);
    }
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&filter1), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, &g);
        let tc = duel::first_target(d);
        if let Some(tc) = tc {
            if card::is_relate_to_effect(d, tc, a.e) {
                duel::break_effect(d);
                duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
            }
        }
    }
}
