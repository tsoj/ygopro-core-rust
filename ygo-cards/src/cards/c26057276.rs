//! Satellarknight Betelgeuse
use super::prelude::*;

pub const ID: u32 = 26057276;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TELLARKNIGHT], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Send this card to the GY, and if you do, add 1 "tellarknight" card from your GY to your hand, except "Satellarknight Betelgeuse"
    let e1a = effect::create_effect(d, c);
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    effect::set_category(d, e1a, CATEGORY_TOGRAVE | CATEGORY_TOHAND);
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1a, EFFECT_FLAG_DELAY | EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1a, EVENT_SUMMON_SUCCESS);
    effect::set_count_limit(d, e1a, 1, ID, 0, 0);
    effect::set_target(d, e1a, Some(cost_target(thtg)));
    effect::set_operation(d, e1a, Some(operation(thop)));
    card::register_effect(d, c, e1a, false);
    let e1b = effect::clone(d, e1a);
    effect::set_code(d, e1b, EVENT_FLIP_SUMMON_SUCCESS);
    card::register_effect(d, c, e1b, false);
    let e1c = effect::clone(d, e1a);
    effect::set_code(d, e1c, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e1c, false);
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_TELLARKNIGHT as u16]) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && thfilter(d, chkc);
    }
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_grave(d, c, None) && duel::is_existing_target(d, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_target(d, tp, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(CardSet::from_card(c)), 1, tp, 0);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, tp, 0);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d);
    if card::is_relate_to_effect(d, c, a.e)
        && duel::send_to_grave(d, c, REASON_EFFECT, None, None) > 0
        && card::is_location(d, c, LOCATION_GRAVE)
        && card::is_relate_to_effect(d, tc.nil(), a.e)
    {
        duel::send_to_hand(d, tc.nil(), None, REASON_EFFECT, None);
    }
}
