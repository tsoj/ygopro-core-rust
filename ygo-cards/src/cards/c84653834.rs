//! ESP Amplifier
use super::prelude::*;

pub const ID: u32 = 84653834;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,...)`
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    // if Duel.IsPhase(PHASE_DAMAGE) and Duel.IsDamageCalculated() then return false end
    if duel::is_phase(d, PHASE_DAMAGE) && duel::is_damage_calculated(d) {
        return false;
    }
    // return true
    true
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_PSYCHIC),tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        let f = aux::faceup_filter(|d, c| card::is_race(d, c, RACE_PSYCHIC));
        return duel::is_existing_matching_card(d, Some(&f), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local sg=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsRace,RACE_PSYCHIC),tp,LOCATION_MZONE,0,nil)
    let f = aux::faceup_filter(|d, c| card::is_race(d, c, RACE_PSYCHIC));
    let sg = duel::get_matching_group(d, Some(&f), tp, LOCATION_MZONE, 0, NO_EX);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=sg:GetFirst()
    let _tc = group::get_first(d, &sg);
    // local atk=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsRace,RACE_PSYCHIC),tp,LOCATION_REMOVED,0,nil)*300
    let atk = duel::get_matching_group_count(d, Some(&f), tp, LOCATION_REMOVED, 0, NO_EX) as i64 * 300;
    // for tc in aux.Next(sg) do
    for tc in group::iter(d, &sg) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // e1:SetValue(atk)
        effect::set_value(d, e1, atk);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
        effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        // e2:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
        // e2:SetRange(LOCATION_MZONE)
        effect::set_range(d, e2, LOCATION_MZONE);
        // e2:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        // e2:SetCountLimit(1)
        effect::set_count_limit(d, e2, 1, 0, 0, 0);
        // e2:SetOperation(s.rmop)
        effect::set_operation(d, e2, Some(operation(rmop)));
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
}
/// `s.rmop(e,tp,...)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // Duel.Remove(e:GetHandler(),POS_FACEUP,REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    duel::remove(d, CardSet::from_card(h), POS_FACEUP as u8, REASON_EFFECT, None, None);
}
