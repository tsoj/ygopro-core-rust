//! Fairy King Truesdale
use super::prelude::*;

pub const ID: u32 = 45425051;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_condition(d, e1, Some(lua_fn(con)));
    effect::set_target(d, e1, Some(effect_card_fn(tg)));
    effect::set_value(d, e1, 500);
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    effect::set_value(d, e2, 500);
    card::register_effect(d, c, e2, false);
}
/// `function(e) return e:GetHandler():IsDefensePos() end`
fn con(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_defense_pos(d, h)
}
/// `function(e,c) return c:IsRace(RACE_PLANT) end`
fn tg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_race(d, c, RACE_PLANT)
}
