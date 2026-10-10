//! Dark Artist
use super::prelude::*;

pub const ID: u32 = 72520073;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // defdown
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e1, EFFECT_SET_DEFENSE_FINAL);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_condition(d, e1, Some(condition(defcon)));
    effect::set_value_fn(d, e1, effect_card_value(defval));
    card::register_effect(d, c, e1, false);
}
fn defcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, c);
    duel::is_phase(d, PHASE_DAMAGE_CAL) && duel::get_attack_target(d) == Some(c) && card::is_attribute(d, bc.nil(), ATTRIBUTE_LIGHT)
}
fn defval(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    // math.ceil(e:GetHandler():GetDefense()/2)
    let h = effect::get_handler(d, e).nil();
    let def = card::get_defense(d, h) as f64;
    (def / 2.0).ceil() as i64
}
