//! Magician's Right Hand
use super::prelude::*;

pub const ID: u32 = 87769556;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // negate
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_CHAIN_SOLVING);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(negcon)));
    effect::set_operation(d, e2, Some(operation(negop)));
    card::register_effect(d, c, e2, false);
}
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_SPELLCASTER)), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
        && a.rp != a.tp
        && effect::is_spell_effect(d, a.re.nil())
        && duel::is_chain_disablable(d, a.ev as u8)
}
fn negop(d: &mut Duel, a: &EffArgs) {
    let rc = effect::get_handler(d, a.re.nil()).nil();
    if duel::negate_effect(d, a.ev as u8) && card::is_relate_to_effect(d, rc, a.re.nil()) {
        duel::destroy(d, rc, REASON_EFFECT, None, None);
    }
}
