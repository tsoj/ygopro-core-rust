//! Glowing Crossbow
use super::prelude::*;

pub const ID: u32 = 53860621;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BLACKWING as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_EQUIP);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
    // Equip limit
    let e2 = effect::create_effect(d, c);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_EQUIP_LIMIT);
    effect::set_value_fn(d, e2, effect_card_fn(eqlimit));
    card::register_effect(d, c, e2, false);
    // atk/def up
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    effect::set_value(d, e3, 500);
    card::register_effect(d, c, e3, false);
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e4, EFFECT_UPDATE_DEFENSE);
    effect::set_value(d, e4, 500);
    card::register_effect(d, c, e4, false);
    // discard
    let e5 = effect::create_effect(d, c);
    effect::set_description(d, e5, aux::stringid(ID, 0));
    effect::set_category(d, e5, CATEGORY_HANDES);
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e5, EVENT_BATTLE_DESTROYING);
    effect::set_range(d, e5, LOCATION_SZONE);
    effect::set_count_limit(d, e5, 1, ID, 0, 0);
    effect::set_condition(d, e5, Some(condition(descon)));
    effect::set_target(d, e5, Some(cost_target(destg)));
    effect::set_operation(d, e5, Some(operation(desop)));
    card::register_effect(d, c, e5, false);
    // tohand
    let e6 = effect::create_effect(d, c);
    effect::set_description(d, e6, aux::stringid(ID, 1));
    effect::set_category(d, e6, CATEGORY_TOHAND);
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e6, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e6, EVENT_TO_GRAVE);
    effect::set_count_limit(d, e6, 1, ID, 1, 0);
    effect::set_condition(d, e6, Some(condition(retcon)));
    effect::set_target(d, e6, Some(cost_target(rettg)));
    effect::set_operation(d, e6, Some(operation(retop)));
    card::register_effect(d, c, e6, false);
}
fn eqlimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_BLACKWING as u16])
}
fn eqfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_BLACKWING as u16])
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && eqfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&eqfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    let _ = duel::select_target(d, tp, Some(&eqfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let tc = duel::first_target(d).nil();
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) && card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        duel::equip(d, tp, h, tc, true, false);
    }
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    let first = group::get_first(d, &eg);
    let h = effect::get_handler(d, a.e).nil();
    first == card::get_equip_target(d, h) && card::is_status(d, first.nil(), STATUS_OPPO_BATTLE)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, tp, 0, LOCATION_HAND) > 0;
    }
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, 1 - tp, 1);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    let g = duel::get_field_group(d, tp, 0, LOCATION_HAND);
    if g.is_empty() {
        return;
    }
    let sg = group::random_select(d, &g, 1 - tp, 1);
    duel::send_to_grave(d, sg, REASON_DISCARD | REASON_EFFECT, None, None);
}
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let ec = card::get_previous_equip_target(d, c);
    card::is_reason(d, c, REASON_LOST_TARGET) && ec.is_some_and(|ec| card::is_reason(d, ec, REASON_SYNCHRO))
}
fn rettg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn retop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, c);
    }
}
