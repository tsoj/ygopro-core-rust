//! Luminous Soldier
use super::prelude::*;

pub const ID: u32 = 57482479;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetCondition(s.condtion)
    effect::set_condition(d, e1, Some(condition(condtion)));
    // e1:SetValue(500)
    effect::set_value(d, e1, 500);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condtion(e)`
fn condtion(d: &mut Duel, a: &EffArgs) -> bool {
    // local ph=Duel.GetCurrentPhase()
    let ph = duel::get_current_phase(d);
    // if not (ph==PHASE_DAMAGE or ph==PHASE_DAMAGE_CAL) or not e:GetHandler():IsRelateToBattle() then return false end
    let h = effect::get_handler(d, a.e).nil();
    if !(ph == PHASE_DAMAGE || ph == PHASE_DAMAGE_CAL) || !card::is_relate_to_battle(d, h) {
        return false;
    }
    // local bc=e:GetHandler():GetBattleTarget()
    let bc = card::get_battle_target(d, h);
    // return bc and bc:IsFaceup() and bc:IsAttribute(ATTRIBUTE_DARK)
    match bc {
        Some(bc) => card::is_faceup(d, bc) && card::is_attribute(d, bc, ATTRIBUTE_DARK),
        None => false,
    }
}
