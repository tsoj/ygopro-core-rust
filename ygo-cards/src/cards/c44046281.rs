//! Dimension Gate
use super::prelude::*;

pub const ID: u32 = 44046281;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_REMOVE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
    // send to grave
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_condition(d, e2, Some(condition(tgcon)));
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    effect::set_operation(d, e2, Some(operation(tgop)));
    card::register_effect(d, c, e2, false);
    // spsummon
    let e2b = effect::create_effect(d, c);
    effect::set_description(d, e2b, aux::stringid(ID, 1));
    effect::set_category(d, e2b, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2b, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2b, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2b, EVENT_LEAVE_FIELD);
    effect::set_condition(d, e2b, Some(condition(spcon)));
    effect::set_target(d, e2b, Some(cost_target(sptg)));
    effect::set_operation(d, e2b, Some(operation(spop)));
    card::register_effect(d, c, e2b, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && card::is_able_to_remove(d, chkc, None, None, None);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, 0, 0);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d);
    if card::is_relate_to_effect(d, c, a.e)
        && card::is_relate_to_effect(d, tc.nil(), a.e)
        && duel::remove(d, CardSet::from_card(tc.nil()), POS_FACEUP as u8, REASON_EFFECT, None, None) > 0
        && card::is_location(d, tc.nil(), LOCATION_REMOVED)
    {
        card::set_card_target(d, c, tc.nil());
    }
}
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    a.ep != a.tp && duel::get_attack_target(d).is_none()
}
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn tgop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::send_to_grave(d, CardSet::from_card(c), REASON_EFFECT, None, None);
    }
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if let Some(tc) = card::get_first_card_target(d, c) {
        if card::is_location(d, c, LOCATION_GRAVE) {
            effect::set_label_object(d, a.e, Value::Card(tc));
            return true;
        }
    }
    false
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tc = effect::get_label_object(d, a.e).as_card();
    if a.chk == Some(0) {
        if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return false;
        }
        return match tc {
            Some(tc) => card::is_can_be_special_summoned(d, tc, a.e, 0, a.tp, false, false, None, None, None),
            None => false,
        };
    }
    duel::set_target_card(d, tc.nil());
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(tc.nil())), 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
