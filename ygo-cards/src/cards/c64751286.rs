//! Penumbral Soldier Lady
use super::prelude::*;

pub const ID: u32 = 64751286;

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
    // e1:SetValue(1000)
    effect::set_value(d, e1, 1000);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condtion(e)`
fn condtion(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ph=Duel.GetCurrentPhase()
    let ph = duel::get_current_phase(d);
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // return (ph==PHASE_DAMAGE or ph==PHASE_DAMAGE_CAL) and c:IsRelateToBattle() and bc and bc:IsFaceup() and bc:IsAttribute(ATTRIBUTE_LIGHT)
    if !((ph == PHASE_DAMAGE || ph == PHASE_DAMAGE_CAL) && card::is_relate_to_battle(d, c)) {
        return false;
    }
    match bc {
        Some(bc) => card::is_faceup(d, bc) && card::is_attribute(d, bc, ATTRIBUTE_LIGHT),
        None => false,
    }
}
