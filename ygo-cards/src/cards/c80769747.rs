//! Iron Chain Snake
use super::prelude::*;

pub const ID: u32 = 80769747;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // equip
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_EQUIP);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    effect::set_operation(d, e1, Some(operation(eqop)));
    card::register_effect(d, c, e1, false);
    // spsummon
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DECKDES);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_LEAVE_FIELD);
    effect::set_condition(d, e2, Some(condition(ddescon)));
    effect::set_target(d, e2, Some(cost_target(ddestg)));
    effect::set_operation(d, e2, Some(operation(ddesop)));
    card::register_effect(d, c, e2, false);
}
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && card::is_faceup(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    let h = effect::get_handler(d, a.e);
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, h.map(CardSet::from_card), 1, 0, 0);
    true
}
fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    effect::get_owner(d, e) == Some(c)
}
fn eqop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d);
    if let Some(tc) = tc {
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) && duel::equip(d, a.tp, c, tc, true, false) {
            // Add Equip limit
            let e1 = effect::create_effect(d, tc);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            effect::set_reset(d, e1, RESET_EVENT + RESETS_STANDARD, 1);
            effect::set_value_fn(d, e1, effect_card_fn(eqlimit));
            card::register_effect(d, c, e1, false);
            let e2 = effect::create_effect(d, c);
            effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
            effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
            effect::set_value(d, e2, -800);
            effect::set_reset(d, e2, RESET_EVENT + RESETS_STANDARD, 1);
            card::register_effect(d, c, e2, false);
            let e3 = effect::create_effect(d, c);
            effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
            effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
            effect::set_value(d, e3, -800);
            effect::set_reset(d, e3, RESET_EVENT + RESETS_STANDARD, 1);
            card::register_effect(d, c, e3, false);
        }
    }
}
fn ddescon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let ec = card::get_previous_equip_target(d, c);
    if card::is_reason(d, c, REASON_LOST_TARGET) {
        let ec = ec.nil();
        if card::is_reason(d, ec, REASON_BATTLE) && card::is_location(d, ec, LOCATION_GRAVE) {
            effect::set_label_object(d, a.e, Value::Card(ec));
            return true;
        }
    }
    false
}
fn ddestg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let ec = effect::get_label_object(d, a.e).as_card().nil();
    let desp = card::get_previous_controler(d, ec);
    let desc = card::get_level(d, ec);
    duel::set_target_player(d, desp);
    duel::set_target_param(d, desc);
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, desp, desc as i32);
    true
}
fn ddesop(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::discard_deck(d, info[0].int() as u8, info[1].int() as u16, REASON_EFFECT);
}
