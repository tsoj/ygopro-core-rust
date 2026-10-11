//! A・O・J ルドラ (Ally of Justice Rudra)
use super::prelude::*;

pub const ID: u32 = 52265835;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetCondition(s.condtion)
    effect::set_condition(d, e1, Some(lua_fn(condtion)));
    // e1:SetValue(700)
    effect::set_value(d, e1, 700);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.condtion(e)
fn condtion(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // local ph=Duel.GetCurrentPhase()
    let ph = duel::get_current_phase(d);
    // if not (ph==PHASE_DAMAGE or ph==PHASE_DAMAGE_CAL) then return false end
    if !(ph == PHASE_DAMAGE || ph == PHASE_DAMAGE_CAL) {
        return false;
    }
    // local a=Duel.GetAttacker()
    let attacker = duel::get_attacker(d);
    // local d=Duel.GetAttackTarget()
    let defender = duel::get_attack_target(d);
    // return (a==e:GetHandler() and d and d:IsFaceup() and d:IsAttribute(ATTRIBUTE_LIGHT))
    let handler = effect::get_handler(d, e).nil();
    if attacker == Some(handler) {
        if let Some(dc) = defender {
            if card::is_faceup(d, dc) && card::is_attribute(d, dc, ATTRIBUTE_LIGHT) {
                return true;
            }
        }
    }
    // or (d==e:GetHandler() and a:IsAttribute(ATTRIBUTE_LIGHT))
    if defender == Some(handler) && card::is_attribute(d, attacker.nil(), ATTRIBUTE_LIGHT) {
        return true;
    }
    false
}
