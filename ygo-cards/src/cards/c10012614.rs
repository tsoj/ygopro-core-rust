//! Banner of Courage
use super::prelude::*;

pub const ID: u32 = 10012614;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Atk up
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_condition(d, e2, Some(lua_fn(con)));
    effect::set_value(d, e2, 200);
    card::register_effect(d, c, e2, false);
}
// function s.con(e)
fn con(d: &mut Duel, a: Args) -> bool {
    // return Duel.IsTurnPlayer(e:GetHandlerPlayer()) and Duel.IsBattlePhase()
    let p = effect::get_handler_player(d, a.e(0));
    duel::is_turn_player(d, p) && duel::is_battle_phase(d, None)
}
