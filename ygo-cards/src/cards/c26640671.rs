//! Spiders' Lair
use super::prelude::*;

pub const ID: u32 = 26640671;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // target
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_target(d, e2, Some(cost_target(target)));
    effect::set_operation(d, e2, Some(operation(operation_fn)));
    card::register_effect(d, c, e2, false);
    //
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_code(d, e3, EVENT_BATTLED);
    effect::set_operation(d, e3, Some(operation(regop)));
    card::register_effect(d, c, e3, false);
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_range(d, e4, LOCATION_SZONE);
    effect::set_code(d, e4, EVENT_PHASE | PHASE_BATTLE);
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    effect::set_operation(d, e4, Some(operation(posop)));
    card::register_effect(d, c, e4, false);
    // poschange
    let e5 = effect::create_effect(d, c);
    effect::set_type(d, e5, EFFECT_TYPE_FIELD);
    effect::set_range(d, e5, LOCATION_SZONE);
    effect::set_code(d, e5, EFFECT_CANNOT_CHANGE_POSITION);
    effect::set_target_range(d, e5, 0, LOCATION_MZONE);
    effect::set_target(d, e5, Some(effect_card_fn(postg)));
    card::register_effect(d, c, e5, false);
}
// function s.filter(c,ec)
fn filter(d: &mut Duel, c: CardId, ec: CardId) -> bool {
    card::is_faceup(d, c) && !card::is_has_card_target(d, ec, c) && card::is_race(d, c, RACE_INSECT)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc, h);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, h)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    duel::select_target(d, a.tp, Some(&|d, c| filter(d, c, h)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let tc = duel::first_target(d).nil();
    if !card::is_relate_to_effect(d, tc, a.e) || card::is_facedown(d, tc) {
        return;
    }
    card::set_card_target(d, c, tc);
}
// function s.regop(e,tp,eg,ep,ev,re,r,rp)
fn regop(d: &mut Duel, a: &EffArgs) {
    let att = duel::get_attacker(d);
    let def = duel::get_attack_target(d);
    let h = effect::get_handler(d, a.e).nil();
    if let Some(dt) = def {
        if card::is_controler(d, dt, 1 - a.tp) && card::is_has_card_target(d, h, att.nil()) {
            card::register_flag_effect(d, dt, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
            return;
        }
        if card::is_controler(d, att.nil(), 1 - a.tp) && card::is_has_card_target(d, h, dt) {
            card::register_flag_effect(d, att.nil(), ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
        }
    }
}
// function s.pfilter(c)
fn pfilter(d: &mut Duel, c: CardId) -> bool {
    card::get_flag_effect(d, c, ID) != 0 && card::is_attack_pos(d, c)
}
// function s.posop(e,tp,eg,ep,ev,re,r,rp)
fn posop(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_matching_group(d, Some(&pfilter), a.tp, 0, LOCATION_MZONE, NO_EX);
    duel::change_position(d, &g, POS_FACEUP_DEFENSE as u8, None, None, None, false);
    let _ = group::get_first(d, &g);
    for tc in group::iter(d, &g) {
        card::register_flag_effect(d, tc, ID + 1, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
    }
}
// function s.postg(e,c)
fn postg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::get_flag_effect(d, c, ID + 1) != 0
}
