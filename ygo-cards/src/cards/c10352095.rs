//! Scroll of Bewitchment
use super::prelude::*;

pub const ID: u32 = 10352095;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,nil,nil,nil,s.target)
    aux::add_equip_procedure(d, c, None, None, None, None, Some(std::sync::Arc::new(target)), None, None, None);
    // ATTRIBUTE
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e2, EFFECT_CHANGE_ATTRIBUTE);
    effect::set_condition(d, e2, Some(condition(attcon)));
    effect::set_value_fn(d, e2, effect_card_value(value));
    card::register_effect(d, c, e2, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,tc)`: `aux.EquipTarget` calls it with the chosen monster and no `chk`.
fn target(d: &mut Duel, a: &EffArgs, tc: Option<CardId>) {
    let rc = card::announce_another_attribute(d, tc.nil(), a.tp);
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, rc as i64, 0);
    card::set_hint(d, h, CHINT_ATTRIBUTE as u8, rc as u64);
}
fn attcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, h, ID) != 0
}
fn value(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    let h = effect::get_handler(d, e).nil();
    card::get_flag_effect_label(d, h, ID).first().copied().unwrap_or(0)
}
