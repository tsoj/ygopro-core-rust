//! Gora Turtle of Illusion
use super::prelude::*;

pub const ID: u32 = 42868711;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // disable
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_DISABLE);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, LOCATION_SZONE, LOCATION_SZONE);
    effect::set_target(d, e1, Some(effect_card_fn(distg)));
    card::register_effect(d, c, e1, false);
    // disable effect
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_CHAIN_SOLVING);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_operation(d, e2, Some(operation(disop)));
    card::register_effect(d, c, e2, false);
    // self destroy
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_code(d, e3, EFFECT_SELF_DESTROY);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_target_range(d, e3, LOCATION_SZONE, LOCATION_SZONE);
    effect::set_target(d, e3, Some(effect_card_fn(distg)));
    card::register_effect(d, c, e3, false);
    // Double Snare
    aux::double_snare_validity(d, Some(c), LOCATION_MZONE, None);
}
fn distg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let h = effect::get_handler(d, e).nil();
    card::is_controler(d, c, 1 - effect::get_handler_player(d, e)) && card::is_has_card_target(d, c, h)
}
fn disop(d: &mut Duel, a: &EffArgs) {
    if a.rp == a.tp || !effect::is_spell_trap_effect(d, a.re.nil()) {
        return;
    }
    let re = a.re.nil();
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return;
    }
    let g = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]).first().and_then(|v| v.as_group().cloned());
    let Some(g) = g else { return };
    if g.is_empty() {
        return;
    }
    let h = effect::get_handler(d, a.e).nil();
    if group::is_contains(&g, h) && duel::negate_effect(d, a.ev as u8) {
        let rc = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rc, re) {
            duel::destroy(d, rc, REASON_EFFECT, None, None);
        }
    }
}
