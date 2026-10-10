//! Tindangle Jhrelth
use super::prelude::*;

pub const ID: u32 = 24158464;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TINDANGLE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOGRAVE);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // search
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_TOGRAVE + CATEGORY_SET);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
}
fn discardable(d: &mut Duel, c: CardId) -> bool {
    card::is_discardable(d, c, None, None)
}
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&discardable), tp, LOCATION_HAND, 0, 1, c.into());
    }
    duel::discard_hand(d, tp, Some(&discardable), 1, 1, REASON_COST | REASON_DISCARD, c.into());
    true
}
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_TINDANGLE as u16]) && card::is_able_to_grave(d, c, None) && !card::is_code(d, c, &[ID])
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, NO_EX)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if g.len() > 0
        && duel::send_to_grave(d, g, REASON_EFFECT, None, None) != 0
        && card::is_relate_to_effect(d, c, a.e)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
    {
        duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEDOWN_DEFENSE as u8, None);
    }
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_type(d, c, TYPE_FLIP) && (card::is_able_to_hand(d, c, None) || card::is_able_to_grave(d, c, None)) && !card::is_code(d, c, &[ID])
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
    let sel = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel).nil();
    aux::to_hand_or_else(d, tc, tp, None, None, None);
}
