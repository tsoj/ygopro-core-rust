//! Giganticastle
use super::prelude::*;

pub const ID: u32 = 63422098;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Gain ATK/DEF per each material used
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_value_fn(d, e1, effect_card_value(val));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    card::register_effect(d, c, e2, false);
    // Register effect when it is Synchro Summoned
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    effect::set_operation(d, e3, Some(operation(regop)));
    card::register_effect(d, c, e3, false);
}
fn val(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    // local ct=e:GetHandler():GetFlagEffectLabel(id)
    // if not ct then return 0 end
    // return ct
    let h = effect::get_handler(d, e).nil();
    card::get_flag_effect_label(d, h, ID).first().copied().unwrap_or(0)
}
fn regop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_synchro_summoned(d, c) {
        let ct = card::get_material_count(d, c) as i64 - 1;
        card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD_DISABLE, 0, 0, ct * 200, 0);
    }
}
