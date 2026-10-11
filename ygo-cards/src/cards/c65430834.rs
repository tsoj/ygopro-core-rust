//! Jurassic Impact
use super::prelude::*;

pub const ID: u32 = 65430834;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(Card.IsRace,tp,LOCATION_MZONE,0,2,nil,RACE_DINOSAUR) and Duel.GetLP(tp)<Duel.GetLP(1-tp)
    duel::is_existing_matching_card(
        d,
        Some(&|d, c| card::is_race(d, c, RACE_DINOSAUR)),
        a.tp,
        LOCATION_MZONE,
        0,
        2,
        NO_EX,
    ) && duel::get_lp(d, a.tp) < duel::get_lp(d, 1 - a.tp)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_MZONE,LOCATION_MZONE)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, LOCATION_MZONE, LOCATION_MZONE) > 0;
    }
    // local g=Duel.GetFieldGroup(tp,LOCATION_MZONE,LOCATION_MZONE)
    let g = duel::get_field_group(d, a.tp, LOCATION_MZONE, LOCATION_MZONE);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n as u8, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,PLAYER_ALL,#g*1000)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, PLAYER_ALL as u8, (n * 1000) as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetFieldGroup(tp,LOCATION_MZONE,LOCATION_MZONE)
    let g = duel::get_field_group(d, a.tp, LOCATION_MZONE, LOCATION_MZONE);
    // local ct=Duel.Destroy(g,REASON_EFFECT)
    let ct = duel::destroy(d, g, REASON_EFFECT, None, None);
    // if ct>0 then
    if ct > 0 {
        // local val=Duel.Damage(tp,ct*1000,REASON_EFFECT)
        let val = duel::damage(d, a.tp, ct as i64 * 1000, REASON_EFFECT, false, None);
        // if val>0 and Duel.GetLP(tp)>0 then
        if val > 0 && duel::get_lp(d, a.tp) > 0 {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Damage(1-tp,val,REASON_EFFECT)
            duel::damage(d, 1 - a.tp, val as i64, REASON_EFFECT, false, None);
        }
    }
    // if e:IsHasType(EFFECT_TYPE_ACTIVATE) then
    if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        let h = effect::get_handler(d, a.e).nil();
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e1:SetCode(EFFECT_CANNOT_SUMMON)
        effect::set_code(d, e1, EFFECT_CANNOT_SUMMON);
        // e1:SetTargetRange(1,1)
        effect::set_target_range(d, e1, 1, 1);
        // e1:SetReset(RESET_PHASE|PHASE_END,2)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 2);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
        effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, a.tp);
        // local e3=Effect.CreateEffect(e:GetHandler())
        let e3 = effect::create_effect(d, h);
        // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
        // e3:SetDescription(aux.Stringid(id,1))
        effect::set_description(d, e3, aux::stringid(ID, 1));
        // e3:SetReset(RESET_PHASE|PHASE_END,2)
        effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 2);
        // e3:SetTargetRange(1,1)
        effect::set_target_range(d, e3, 1, 1);
        // Duel.RegisterEffect(e3,tp)
        duel::register_effect(d, e3, a.tp);
    }
}
