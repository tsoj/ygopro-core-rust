//! Infinite Impermanence
use super::prelude::*;

pub const ID: u32 = 10045474;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate the effects of 1 monster the opponent controls
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DISABLE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Can be activated from the hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_TRAP_ACT_IN_HAND);
    effect::set_condition(d, e2, Some(condition(handcon)));
    card::register_effect(d, c, e2, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && card::is_negatable_monster(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&card::is_negatable_monster), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    let c = effect::get_handler(d, a.e).nil();
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_NEGATE as u64);
    let g = duel::select_target(d, a.tp, Some(&card::is_negatable_monster), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(g), 1, 0, 0);
    let pos = if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) && !card::is_status(d, c, STATUS_ACT_FROM_HAND) && card::is_previous_position(d, c, POS_FACEDOWN) {
        POS_FACEDOWN
    } else {
        0
    };
    duel::set_target_param(d, pos);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d).nil();
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) && card::is_can_be_disabled_by_effect(d, tc, a.e, true) {
        duel::negate_related_chain(d, tc, RESET_TURN_SET);
        // Negate its effects
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_DISABLE);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e1, false);
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        effect::set_value(d, e2, RESET_TURN_SET as i64);
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e2, false);
        let pos = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
        if card::is_relate_to_effect(d, c, a.e) && (pos & POS_FACEDOWN as i64) > 0 {
            duel::break_effect(d);
            card::register_flag_effect(d, c, ID, RESET_CHAIN, 0, 0, 0, 0);
            // Negate Spell/Trap effects in the same column
            let e3 = effect::create_effect(d, c);
            effect::set_type(d, e3, EFFECT_TYPE_FIELD);
            effect::set_code(d, e3, EFFECT_DISABLE);
            effect::set_target_range(d, e3, LOCATION_ONFIELD, LOCATION_ONFIELD);
            effect::set_target(d, e3, Some(effect_card_fn(distg)));
            effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 1);
            let seq = card::get_sequence(d, c) as i64;
            effect::set_label(d, e3, &[seq]);
            duel::register_effect(d, e3, a.tp);
            let e4 = effect::clone(d, e3);
            effect::set_code(d, e4, EFFECT_DISABLE_TRAPMONSTER);
            duel::register_effect(d, e4, a.tp);
            let e5 = effect::create_effect(d, c);
            effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            effect::set_code(d, e5, EVENT_CHAIN_SOLVING);
            effect::set_operation(d, e5, Some(operation(disop)));
            effect::set_reset(d, e5, RESET_PHASE | PHASE_END, 1);
            let seq = card::get_sequence(d, c) as i64;
            effect::set_label(d, e5, &[seq]);
            duel::register_effect(d, e5, a.tp);
            let zone: u64 = 1 << (card::get_sequence(d, c) + 8);
            duel::hint(d, HINT_ZONE as u8, a.tp, zone);
        }
    }
}
fn distg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let mut seq = effect::get_label(d, e);
    let p = 1 - effect::get_handler_player(d, e);
    if card::is_controler(d, c, p) {
        seq = 4 - seq;
    }
    card::is_spell_trap(d, c) && seq == card::get_sequence(d, c) as i64 && card::get_flag_effect(d, c, ID) == 0
}
fn disop(d: &mut Duel, a: &EffArgs) {
    let mut cseq = effect::get_label(d, a.e);
    let re = a.re.nil();
    if !effect::is_spell_trap_effect(d, re) {
        return;
    }
    let rc = effect::get_handler(d, re).nil();
    if card::has_flag_effect(d, rc, ID, None) {
        return;
    }
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_CONTROLER, CHAININFO_TRIGGERING_LOCATION, CHAININFO_TRIGGERING_SEQUENCE]);
    let p = info[0].int() as u8;
    let mut loc = info[1].int() as u32;
    let mut seq = info[2].int();
    if effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) {
        if (loc & LOCATION_SZONE) == 0 || card::is_controler(d, rc, 1 - p) {
            if card::is_location(d, rc, LOCATION_SZONE) && card::is_controler(d, rc, p) {
                seq = card::get_sequence(d, rc) as i64;
            } else {
                seq = card::get_previous_sequence(d, rc) as i64;
            }
        }
        if (loc & LOCATION_SZONE) == 0 {
            let val = effect::get_value(d, re);
            if matches!(val, Value::Nil) || matches!(val, Value::Int(x) if x == LOCATION_SZONE as i64 || x == LOCATION_FZONE as i64 || x == LOCATION_PZONE as i64) {
                loc = LOCATION_SZONE;
            }
        }
    }
    if a.ep != effect::get_handler_player(d, a.e) {
        cseq = 4 - cseq;
    }
    if (loc & LOCATION_SZONE) != 0 && cseq == seq {
        duel::negate_effect(d, a.ev as u8);
    }
}
fn handcon(d: &mut Duel, a: &EffArgs) -> bool {
    let p = effect::get_handler_player(d, a.e);
    duel::get_field_group_count(d, p, LOCATION_ONFIELD, 0) == 0
}
