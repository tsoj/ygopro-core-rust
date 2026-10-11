//! Genex Ally Powercell
use super::prelude::*;

pub const ID: u32 = 94622638;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atkup
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    effect::set_target(d, e1, Some(effect_card_fn(atktg)));
    effect::set_value(d, e1, 500);
    card::register_effect(d, c, e1, false);
}
/// `s.atktg(e,c)`
fn atktg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c~=e:GetHandler() and e:GetHandler():IsAttribute(c:GetAttribute())
    let h = effect::get_handler(d, e).nil();
    if c == h {
        return false;
    }
    let att = card::get_attribute(d, c);
    card::is_attribute(d, h, att)
}
