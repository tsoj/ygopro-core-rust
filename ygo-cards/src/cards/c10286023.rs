//! Adamancipator Crystal - Dragite
use super::prelude::*;

pub const ID: u32 = 10286023;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ADAMANCIPATOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // decktop other
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(dtcon)));
    effect::set_target(d, e1, Some(cost_target(dttg)));
    effect::set_operation(d, e1, Some(operation(dtop)));
    card::register_effect(d, c, e1, false);
    // to extra
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TODECK);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    effect::set_operation(d, e2, Some(operation(tdop)));
    card::register_effect(d, c, e2, false);
}
fn dtcon(d: &mut Duel, a: &EffArgs) -> bool {
    match a.re {
        Some(re) => {
            let h = effect::get_handler(d, re).nil();
            card::is_set_card(d, h, &[SET_ADAMANCIPATOR as u16])
        }
        None => false,
    }
}
fn dttg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1);
    }
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
fn dtop(d: &mut Duel, a: &EffArgs) {
    duel::draw(d, a.tp, 1, REASON_EFFECT);
}
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_GRAVE))
        && card::is_attribute(d, c, ATTRIBUTE_WATER)
        && card::is_type(d, c, TYPE_SYNCHRO)
        && card::is_able_to_extra(d, c)
}
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e);
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE | LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && tdfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tdfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX)
            && card::is_able_to_deck(d, c.nil());
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    let sel = duel::select_target(d, tp, Some(&tdfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    duel::set_operation_info(d, 0, CATEGORY_TODECK, c.map(CardSet::from_card), 1, tp, LOCATION_GRAVE as i32);
    if card::is_location(d, tc.nil(), LOCATION_GRAVE) {
        duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, tc.map(CardSet::from_card), 1, tp, 0);
    }
    true
}
fn tdop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e);
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::send_to_deck(d, tc, None, SEQ_DECKTOP as i32, REASON_EFFECT, None) != 0
        && card::is_location(d, tc, LOCATION_EXTRA)
        && card::is_relate_to_effect(d, c.nil(), a.e)
        && duel::send_to_deck(d, c.nil(), None, SEQ_DECKTOP as i32, REASON_EFFECT, None) != 0
    {
        duel::confirm_decktop(d, a.tp, 1);
    }
}
