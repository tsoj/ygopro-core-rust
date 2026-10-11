//! D/D/D Contract Change
use super::prelude::*;

pub const ID: u32 = 811734;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DDD, SET_DD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let attacker = duel::get_attacker(d).nil();
    card::is_controler(d, attacker, 1 - a.tp)
}
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_DDD as u16])
        && card::get_attack(d, c) > 0
        && card::is_able_to_remove(d, c, None, None, None)
        && aux::sp_elim_filter(d, c, true, false)
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_level_below(d, c, 4)
        && card::is_set_card(d, c, &[SET_DD as u16])
        && card::is_type(d, c, TYPE_PENDULUM)
        && card::is_able_to_hand(d, c, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let b1 = duel::is_existing_matching_card(d, Some(&atkfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    let b2 = duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    if a.chk == Some(0) {
        return b1 || b2;
    }
    let op: i64 = if b1 && b2 {
        duel::select_option(d, tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1)]) as i64
    } else if b1 {
        duel::select_option(d, tp, true, &[aux::stringid(ID, 0)]) as i64
    } else {
        duel::select_option(d, tp, true, &[aux::stringid(ID, 1)]) as i64 + 1
    };
    effect::set_label(d, a.e, &[op]);
    if op == 0 {
        effect::set_category(d, a.e, CATEGORY_REMOVE);
        if duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_some() {
            duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, tp, LOCATION_MZONE as i32);
        } else {
            duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, tp, LOCATION_GRAVE as i32);
        }
    } else {
        effect::set_category(d, a.e, CATEGORY_TOHAND + CATEGORY_SEARCH);
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    }
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if effect::get_label(d, a.e) == 0 {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        let g = duel::select_matching_card(d, tp, Some(&atkfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        if g.is_empty() {
            return;
        }
        let tc0 = group::get_first(d, &g).nil();
        let atk = card::get_attack(d, tc0);
        let tc = duel::get_attacker(d).nil();
        if duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0 && card::is_relate_to_battle(d, tc) && card::is_faceup(d, tc) {
            let e1 = effect::create_effect(d, effect::get_handler(d, a.e).nil());
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            effect::set_value(d, e1, -(atk as i64));
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, tc, e1, false);
        }
    } else {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        if !g.is_empty() {
            duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
            duel::confirm_cards(d, 1 - tp, &g);
        }
    }
}
