//! Berserk Gorilla
use super::prelude::*;

pub const ID: u32 = 39168895;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // must attack
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_MUST_ATTACK);
    card::register_effect(d, c, e1, false);
    // destroy
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_code(d, e3, EFFECT_SELF_DESTROY);
    effect::set_condition(d, e3, Some(condition(descon)));
    card::register_effect(d, c, e3, false);
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsDefensePos()
    let h = effect::get_handler(d, a.e).nil();
    card::is_defense_pos(d, h)
}
