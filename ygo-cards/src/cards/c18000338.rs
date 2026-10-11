//! Re:EX
use super::prelude::*;

pub const ID: u32 = 18000338;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // increase atk/def
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
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetValue(800)
    effect::set_value(d, e1, 800);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e1) (the Lua registers e1 again, not e2)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ph=Duel.GetCurrentPhase()
    let ph = duel::get_current_phase(d);
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // return (ph==PHASE_DAMAGE or ph==PHASE_DAMAGE_CAL) and c:IsRelateToBattle() and bc and bc:GetSequence()>=5
    (ph == PHASE_DAMAGE || ph == PHASE_DAMAGE_CAL)
        && card::is_relate_to_battle(d, c)
        && bc.is_some_and(|bc| card::get_sequence(d, bc) >= 5)
}
