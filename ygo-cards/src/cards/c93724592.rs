//! Karakuri Ninja mdl 339 "Sazank"
use super::prelude::*;

pub const ID: u32 = 93724592;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // must attack
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_MUST_ATTACK);
    card::register_effect(d, c, e1, false);
    // to defense
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_POSITION);
    effect::set_type(d, e3, EFFECT_TYPE_TRIGGER_F + EFFECT_TYPE_SINGLE);
    effect::set_code(d, e3, EVENT_BE_BATTLE_TARGET);
    effect::set_condition(d, e3, Some(condition(poscon)));
    effect::set_operation(d, e3, Some(operation(posop)));
    card::register_effect(d, c, e3, false);
    // Destroy
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 1));
    effect::set_type(d, e4, EFFECT_TYPE_TRIGGER_F + EFFECT_TYPE_SINGLE);
    effect::set_category(d, e4, CATEGORY_TOGRAVE);
    effect::set_code(d, e4, EVENT_FLIP);
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_target(d, e4, Some(cost_target(tgtg)));
    effect::set_operation(d, e4, Some(operation(tgop)));
    card::register_effect(d, c, e4, false);
    // direct attack
    let e5 = effect::create_effect(d, c);
    effect::set_type(d, e5, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    effect::set_code(d, e5, EVENT_FLIP);
    effect::set_operation(d, e5, Some(operation(dirop)));
    card::register_effect(d, c, e5, false);
}
fn poscon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_attack_pos(d, c)
}
fn posop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        duel::change_position(d, CardSet::from_card(c), POS_FACEUP_DEFENSE as u8, None, None, None, false);
    }
}
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc);
    }
    if a.chk == Some(0) {
        return true;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g.clone()), g.len() as u8, 0, 0);
    true
}
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d);
    if let Some(tc) = tc {
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
            duel::send_to_grave(d, CardSet::from_card(tc), REASON_EFFECT, None, None);
        }
    }
}
fn dirop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    card::register_effect(d, c, e1, false);
}
