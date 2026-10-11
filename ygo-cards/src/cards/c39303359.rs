//! Ancient Gear Knight
//! 古代の機械騎士
use super::prelude::*;

pub const ID: u32 = 39303359;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Gemini.AddProcedure(c)
    gemini::add_procedure(d, c);
    // Opponent cannot activate Spell/Trap cards
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // e1:SetCondition(function(e) return Gemini.EffectStatusCondition(e) and Duel.GetAttacker()==e:GetHandler() end)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetValue(function(_,re) return re:IsHasType(EFFECT_TYPE_ACTIVATE) end)
    effect::set_value_fn(d, e1, lua_fn(|d: &mut Duel, a: Args| effect::is_has_type(d, a.e(1), EFFECT_TYPE_ACTIVATE)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `function(e) return Gemini.EffectStatusCondition(e) and Duel.GetAttacker()==e:GetHandler() end`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    gemini::effect_status_condition().call(d, &[Value::Effect(a.e)]).truthy()
        && duel::get_attacker(d) == effect::get_handler(d, a.e)
}
