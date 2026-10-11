//! Storm Cipher
use super::prelude::*;

pub const ID: u32 = 89571015;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot attack directly
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
    effect::set_code(d, e1, EFFECT_CANNOT_DIRECT_ATTACK);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // cannot attack emz
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e2:SetValue(s.atlimit)
    effect::set_value_fn(d, e2, effect_card_fn(atlimit));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // immune
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e3, EFFECT_IMMUNE_EFFECT);
    // e3:SetValue(s.immval)
    effect::set_value_fn(d, e3, lua_fn(immval));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // indes
    // local e4=e2:Clone()
    let e4 = effect::clone(d, e2);
    // e4:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e4, EFFECT_INDESTRUCTABLE_BATTLE);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// function s.atlimit(e,c)
fn atlimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:GetSequence()>4
    card::get_sequence(d, c) > 4
}
// function s.immval(e,te)
fn immval(d: &mut Duel, a: Args) -> Value {
    // local tc=te:GetHandler()
    let te = a.e(1);
    let tc = effect::get_handler(d, te).nil();
    // if not (tc:IsLocation(LOCATION_MZONE) or tc:IsPreviousLocation(LOCATION_MZONE)) then return end
    if !(card::is_location(d, tc, LOCATION_MZONE) || card::is_previous_location(d, tc, LOCATION_MZONE)) {
        return Value::Nil;
    }
    // if tc:IsLocation(LOCATION_MZONE) then
    if card::is_location(d, tc, LOCATION_MZONE) {
        // return te:IsMonsterEffect() and te:IsActivated() and tc:GetSequence()>4
        return Value::Bool(effect::is_monster_effect(d, te) && effect::is_activated(d, te) && card::get_sequence(d, tc) > 4);
    } else if card::is_previous_location(d, tc, LOCATION_MZONE) {
        // return te:IsMonsterEffect() and te:IsActivated() and tc:GetPreviousSequence()>4
        return Value::Bool(effect::is_monster_effect(d, te) && effect::is_activated(d, te) && card::get_previous_sequence(d, tc) > 4);
    }
    Value::Nil
}
