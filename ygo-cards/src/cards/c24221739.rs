//! Protector of the Sanctuary
use super::prelude::*;

pub const ID: u32 = 24221739;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot draw
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_CANNOT_DRAW)
    effect::set_code(d, e1, EFFECT_CANNOT_DRAW);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // e1:SetCondition(s.con)
    effect::set_condition(d, e1, Some(lua_fn(con)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.con(e)`
fn con(d: &mut Duel, _a: Args) -> bool {
    // return Duel.GetCurrentPhase()~=PHASE_DRAW
    duel::get_current_phase(d) != PHASE_DRAW
}
