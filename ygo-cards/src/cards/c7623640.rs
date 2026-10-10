//! Ceruli, Guru of Dark World
use super::prelude::*;

pub const ID: u32 = 7623640;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // spsummon success
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_HANDES);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e2, Some(condition(hdcon)));
    effect::set_target(d, e2, Some(cost_target(hdtg)));
    effect::set_operation(d, e2, Some(operation(hdop)));
    card::register_effect(d, c, e2, false);
}
/// `s.spcon(e,tp,...)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, h, LOCATION_HAND) && (a.r & (REASON_DISCARD | REASON_EFFECT)) == (REASON_DISCARD | REASON_EFFECT)
}
/// `s.sptg(e,tp,...,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,...)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) {
        duel::special_summon(d, h, 0, a.tp, 1 - a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
/// `s.hdcon(e,tp,...)`
fn hdcon(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    card::is_set_card(d, rh, &[SET_DARK_WORLD as u16])
}
/// `s.hdtg(e,tp,...,chk)`
fn hdtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, 1 - a.tp, 1);
    true
}
/// `s.hdop(e,tp,...)`
fn hdop(d: &mut Duel, a: &EffArgs) {
    duel::discard_hand(d, 1 - a.tp, Some(&|_, _| true), 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
}
