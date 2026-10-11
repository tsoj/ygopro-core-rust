//! The End of Anubis
use super::prelude::*;

pub const ID: u32 = 65403020;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // negate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EVENT_CHAIN_SOLVING);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
    aux::double_snare_validity(d, Some(c), LOCATION_MZONE, None);
}
fn gfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_location(d, c, LOCATION_GRAVE)
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local loc=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION)
    let loc = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION])[0].int();
    if loc == LOCATION_GRAVE as i64 {
        return true;
    }
    let re = a.re.nil();
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    // local g=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    // return g and g:IsExists(s.gfilter,1,nil)
    match duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]).first().and_then(|v| v.as_group().cloned()) {
        Some(g) => group::is_exists(d, &g, &gfilter, 1, NO_EX),
        None => false,
    }
}
fn op(d: &mut Duel, a: &EffArgs) {
    duel::negate_effect(d, a.ev as u8);
}
