//! Revenge Rally
use super::prelude::*;

pub const ID: u32 = 51686645;

pub const INFO: ScriptInfo = ScriptInfo { self_equip_trap: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Target 1 face-up monster your opponent controls; equip this card to that target. It gains 500 ATK/DEF
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_EQUIP);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
    // If this card is sent to the GY because the equipped monster has left the field: Draw 1 card, or if you activated this card this turn, draw 2 cards then discard 1 card, instead
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DRAW + CATEGORY_HANDES);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(drcon)));
    effect::set_target(d, e2, Some(cost_target(drtg)));
    effect::set_operation(d, e2, Some(operation(drop_op)));
    card::register_effect(d, c, e2, false);
}
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let c = effect::get_handler(d, a.e).nil();
    let cid = duel::get_chain_info(d, 0, &[CHAININFO_CHAIN_ID])[0].int();
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_REMAIN_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_OATH, 0);
    effect::set_reset(d, e1, RESET_CHAIN, 1);
    card::register_effect(d, c, e1, false);
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_CHAIN_DISABLED);
    effect::set_operation(d, e2, Some(operation(tgop)));
    effect::set_label(d, e2, &[cid]);
    effect::set_reset(d, e2, RESET_CHAIN, 1);
    duel::register_effect(d, e2, a.tp);
    true
}
fn tgop(d: &mut Duel, a: &EffArgs) {
    let cid = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_CHAIN_ID])[0].int();
    if cid != effect::get_label(d, a.e) {
        return;
    }
    let owner = effect::get_owner(d, a.e).nil();
    if card::is_relate_to_chain(d, owner, a.ev as u8) {
        card::cancel_to_grave(d, owner, false);
    }
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && card::is_faceup(d, chkc);
    }
    if a.chk == Some(0) {
        return effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) && duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(h)), 1, 0, 0);
    card::register_flag_effect(d, h, ID, RESET_PHASE | PHASE_END, EFFECT_FLAG_OATH, 1, 0, 0);
    true
}
// Equip to opponent's monster, it gains 500 ATK/DEF
fn op(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_location(d, c, LOCATION_SZONE) {
        return;
    }
    if !card::is_relate_to_effect(d, c, a.e) || card::is_status(d, c, STATUS_LEAVE_CONFIRMED) {
        return;
    }
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && card::is_controler(d, tc, 1 - a.tp) {
        duel::equip(d, a.tp, c, tc, true, false);
        // Increase the ATK by 500
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_value(d, e1, 500);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, c, e1, false);
        // Increase the DEF by 500
        let e2 = effect::clone(d, e1);
        effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
        card::register_effect(d, c, e2, false);
        // Equip limit
        let e3 = effect::create_effect(d, c);
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e3, EFFECT_EQUIP_LIMIT);
        effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_value(d, e3, 1);
        effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, c, e3, false);
    } else {
        card::cancel_to_grave(d, c, false);
    }
}
// If this card was sent to the GY because the equipped monster left the field
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    let ec = card::get_previous_equip_target(d, h);
    card::is_reason(d, h, REASON_LOST_TARGET) && !card::is_location(d, ec.nil(), LOCATION_ONFIELD | LOCATION_OVERLAY)
}
// Activation legality
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let h = effect::get_handler(d, a.e).nil();
    if card::has_flag_effect(d, h, ID, None) {
        effect::set_label(d, a.e, &[1]);
        card::reset_flag_effect(d, h, ID);
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 2);
        duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, a.tp, 1);
    } else {
        effect::set_label(d, a.e, &[0]);
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    }
    true
}
// Draw 1 card, or 2 cards and discard 1 if activated this turn
fn drop_op(d: &mut Duel, a: &EffArgs) {
    if effect::get_label(d, a.e) == 1 {
        if duel::draw(d, a.tp, 2, REASON_EFFECT) != 0 {
            duel::shuffle_hand(d, a.tp);
            duel::break_effect(d);
            duel::discard_hand(d, a.tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
        }
    } else {
        duel::draw(d, a.tp, 1, REASON_EFFECT);
    }
}
