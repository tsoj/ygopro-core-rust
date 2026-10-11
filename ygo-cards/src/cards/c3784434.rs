//! Rampaging Rhynos
//! 突撃ライノス
use super::prelude::*;

pub const ID: u32 = 3784434;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // move
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(aux.seqmovcon)
    effect::set_condition(d, e1, Some(aux::seqmovcon()));
    // e1:SetTarget(aux.seqmovtg)
    effect::set_target(d, e1, Some(aux::seqmovtg()));
    // e1:SetOperation(aux.seqmovtgop)
    effect::set_operation(d, e1, Some(aux::seqmovtgop()));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(lua_fn(atkcon)));
    // e2:SetValue(500)
    effect::set_value(d, e2, 500);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.atkcon(e)`
fn atkcon(d: &mut Duel, a: Args) -> bool {
    // local ph=Duel.GetCurrentPhase()
    let ph = duel::get_current_phase(d);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e(0)).nil();
    // local at=Duel.GetAttackTarget()
    let at = duel::get_attack_target(d);
    // if (ph==PHASE_DAMAGE or ph==PHASE_DAMAGE_CAL) and Duel.GetAttacker()==c and at then
    if (ph == PHASE_DAMAGE || ph == PHASE_DAMAGE_CAL) && duel::get_attacker(d) == Some(c) {
        if let Some(at) = at {
            // return c:GetColumnGroup():IsContains(at)
            let cg = card::get_column_group(d, c, 0, 0);
            return group::is_contains(&cg, at);
        }
    }
    // else return false end
    false
}
