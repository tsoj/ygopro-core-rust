//! Magic Reflector
use super::prelude::*;

pub const ID: u32 = 61844784;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // destroy rep
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_COUNTER);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_target(d, e1, Some(cost_target(addct)));
    effect::set_operation(d, e1, Some(operation(addc)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_spell(d, c)
}
fn addct(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_SZONE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc) && chkc != h;
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_SZONE, 0, 1, h.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 0));
    duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_SZONE, 0, 1, 1, false, h.into());
    true
}
fn addc(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        card::add_counter(d, tc, 0x102a, 1, false);
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, e1, EFFECT_DESTROY_REPLACE);
        effect::set_target(d, e1, Some(cost_target(reptg)));
        effect::set_operation(d, e1, Some(operation(repop)));
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e1, false);
    }
}
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return (a.r & (REASON_REPLACE + REASON_RULE)) == 0 && card::get_counter(d, h, 0x102a) > 0;
    }
    true
}
fn repop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    card::remove_counter(d, h, a.tp, 0x102a, 1, REASON_EFFECT);
}
