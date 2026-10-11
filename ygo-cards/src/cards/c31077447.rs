//! Punch-in-the-Box
use super::prelude::*;

pub const ID: u32 = 31077447;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let attacker = duel::get_attacker(d).nil();
    card::is_controler(d, attacker, 1 - a.tp) && duel::get_field_group_count(d, a.tp, 0, LOCATION_MZONE) > 1
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tc = duel::get_attacker(d);
    if let Some(chkc) = a.chkc() {
        return Some(chkc) == tc;
    }
    if a.chk == Some(0) {
        let tc = tc.nil();
        return card::is_on_field(d, tc) && card::is_can_be_effect_target(d, tc, Some(a.e));
    }
    duel::set_target_card(d, tc.nil());
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d);
    if !duel::negate_attack(d) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    let ex = match tc {
        Some(x) => x.into(),
        None => NO_EX,
    };
    let g = duel::select_matching_card(d, a.tp, None, a.tp, 0, LOCATION_MZONE, 1, 1, false, ex).nil();
    let sc = group::get_first(d, &g);
    if let Some(sc) = sc {
        if duel::send_to_grave(d, sc, REASON_EFFECT, None, None) != 0 && card::is_location(d, sc, LOCATION_GRAVE) {
            duel::break_effect(d);
            let atk = card::get_attack(d, sc);
            let def = card::get_defense(d, sc);
            let val = 0.max(atk).max(def);
            let c = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            effect::set_value(d, e1, -(val as i64));
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, tc.nil(), e1, false);
        }
    }
}
