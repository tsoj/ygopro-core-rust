//! Lightning Rod Lord
//! 避雷神
use super::prelude::*;

pub const ID: u32 = 40672993;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // actlimit
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(1,1)
    effect::set_target_range(d, e1, 1, 1);
    // e1:SetValue(s.actlimit)
    effect::set_value_fn(d, e1, lua_fn(actlimit));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.actlimit(e,te,tp)
fn actlimit(d: &mut Duel, a: Args) -> bool {
    let te = a.e(1);
    // return Duel.IsPhase(PHASE_MAIN1)
    //     and te:IsHasType(EFFECT_TYPE_ACTIVATE) and te:IsSpellEffect()
    duel::is_phase(d, PHASE_MAIN1) && effect::is_has_type(d, te, EFFECT_TYPE_ACTIVATE) && effect::is_spell_effect(d, te)
}
