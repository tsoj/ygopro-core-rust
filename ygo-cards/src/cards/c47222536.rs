//! Dark Magical Circle
use super::prelude::*;

pub const ID: u32 = 47222536;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_DARK_MAGICIAN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // remove
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_REMOVE);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(rmcon)));
    effect::set_target(d, e2, Some(cost_target(rmtg)));
    effect::set_operation(d, e2, Some(operation(rmop)));
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e3, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) > 2;
    }
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND as u64, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    ((card::lists_code(d, c, &[CARD_DARK_MAGICIAN]) && card::is_spell_trap(d, c)) || card::is_code(d, c, &[CARD_DARK_MAGICIAN]))
        && card::is_able_to_hand(d, c, None)
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    if !card::is_relate_to_effect(d, effect::get_handler(d, e).nil(), e) {
        return;
    }
    if duel::get_field_group_count(d, tp, LOCATION_DECK, 0) < 3 {
        return;
    }
    let g = duel::get_decktop_group(d, tp, 3);
    duel::confirm_cards(d, tp, &g);
    if group::is_exists(d, &g, &filter, 1, NO_EX) && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        let sg = group::filter_select(d, &g, tp, &filter, 1, 1, false, NO_EX).nil();
        duel::disable_shuffle_check(d, true);
        duel::send_to_hand(d, &sg, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, &sg);
        duel::shuffle_hand(d, tp);
        duel::sort_decktop(d, tp, tp, 2);
    } else {
        duel::sort_decktop(d, tp, tp, 3);
    }
}
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_faceup(d, c) && card::is_code(d, c, &[CARD_DARK_MAGICIAN]) && card::is_controler(d, c, tp)
}
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, tp), 1, NO_EX)
}
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - tp) && card::is_able_to_remove(d, chkc, None, None, None);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 1, 0, 0);
    true
}
fn rmop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    if !card::is_relate_to_effect(d, effect::get_handler(d, e).nil(), e) {
        return;
    }
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, e) {
        duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
