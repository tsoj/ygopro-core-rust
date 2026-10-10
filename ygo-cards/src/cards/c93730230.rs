//! Chronomaly Crystal Chrononaut
use super::prelude::*;

pub const ID: u32 = 93730230;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 3 monsters
    xyz::add_procedure(d, c, None, Some(3), 2, None, None, None, None, false, None);
    // When targeted for attack, make itself unable to be destroyed by battle or card effect
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_BE_BATTLE_TARGET);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    card::register_effect(d, c, e1, false);
}
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // Cannot be destroyed by battle or card effect
        let e1 = effect::create_effect(d, c);
        effect::set_description(d, e1, 3008);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
        effect::set_value(d, e1, 1);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, c, e1, false);
        let e2 = effect::clone(d, e1);
        effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
        card::register_effect(d, c, e2, false);
        // Opponent takes the battle damage instead
        let e3 = effect::clone(d, e1);
        effect::set_code(d, e3, EFFECT_REFLECT_BATTLE_DAMAGE);
        card::register_effect(d, c, e3, false);
    }
}
