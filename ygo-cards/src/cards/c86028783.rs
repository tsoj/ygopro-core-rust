//! Rookie Warrior Lady
use super::prelude::*;

pub const ID: u32 = 86028783;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Reduce ATK
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(atkcon)));
    effect::set_cost(d, e1, Some(cost::self_to_grave()));
    effect::set_operation(d, e1, Some(operation(atkop)));
    card::register_effect(d, c, e1, false);
    // Add to hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let (mut at, mut bt) = (duel::get_attacker(d), duel::get_attack_target(d));
    if card::is_controler(d, at.nil(), 1 - a.tp) {
        std::mem::swap(&mut at, &mut bt);
    }
    match (at, bt) {
        (Some(at), Some(bt)) => {
            let h = effect::get_handler(d, a.e).nil();
            at != h
                && card::is_controler(d, at, a.tp)
                && card::is_faceup(d, at)
                && card::is_race(d, at, RACE_WARRIOR as u64)
                && card::is_faceup(d, bt)
                && card::is_controler(d, bt, 1 - a.tp)
        }
        _ => false,
    }
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    let (mut at, mut bt) = (duel::get_attacker(d), duel::get_attack_target(d));
    if card::is_controler(d, at.nil(), 1 - a.tp) {
        std::mem::swap(&mut at, &mut bt);
    }
    if let (Some(_), Some(bt)) = (at, bt) {
        if card::is_relate_to_battle(d, bt) && card::is_faceup(d, bt) && card::is_controler(d, bt, 1 - a.tp) {
            // Update ATK
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            let base = card::get_base_attack(d, bt) as i64;
            effect::set_value(d, e1, -base);
            card::register_effect(d, bt, e1, false);
        }
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_race(d, c, RACE_WARRIOR as u64) && card::is_attribute(d, c, ATTRIBUTE_EARTH) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && thfilter(d, chkc);
    }
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_target(d, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, h.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_target(d, tp, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let n = g.len() as i32;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), n as u8, 0, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
    }
}
