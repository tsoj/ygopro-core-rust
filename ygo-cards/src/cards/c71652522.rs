//! Battle Teleportation
use super::prelude::*;

pub const ID: u32 = 71652522;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
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
    // return Duel.GetCurrentPhase()<=PHASE_BATTLE
    duel::get_current_phase(d) <= PHASE_BATTLE
}
/// `s.target(e,tp,...,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    let tp = a.tp;
    // if chk==0 then return Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsRace,RACE_PSYCHIC),tp,LOCATION_MZONE,0,nil)==1 end
    let f = aux::faceup_filter(|d, c| card::is_race(d, c, RACE_PSYCHIC));
    if a.chk == Some(0) {
        return duel::get_matching_group_count(d, Some(&f), tp, LOCATION_MZONE, 0, NO_EX) == 1;
    }
    // local tc=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsRace,RACE_PSYCHIC),tp,LOCATION_MZONE,0,nil):GetFirst()
    let g = duel::get_matching_group(d, Some(&f), tp, LOCATION_MZONE, 0, NO_EX);
    let tc = group::get_first(d, &g);
    // Duel.SetTargetCard(tc)
    if let Some(tc) = tc {
        duel::set_target_card(d, tc);
    }
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) and tc:IsFaceup() then
    let Some(tc) = duel::first_target(d) else { return };
    if !(card::is_relate_to_effect(d, tc, e) && card::is_faceup(d, tc)) {
        return;
    }
    let h = effect::get_handler(d, e).nil();
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
    // local e2=Effect.CreateEffect(e:GetHandler())
    let e2 = effect::create_effect(d, h);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_PHASE|PHASE_BATTLE)
    effect::set_code(d, e2, EVENT_PHASE | PHASE_BATTLE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetReset(RESET_PHASE|PHASE_BATTLE)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_BATTLE, 1);
    // e2:SetOperation(s.ctop)
    effect::set_operation(d, e2, Some(operation(ctop)));
    // e2:SetLabelObject(tc)
    effect::set_label_object(d, e2, Value::Card(tc));
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
    // tc:RegisterFlagEffect(id,RESET_EVENT|(RESETS_STANDARD&~RESET_TURN_SET)|RESET_PHASE|PHASE_BATTLE,0,1)
    card::register_flag_effect(d, tc, ID, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET) | RESET_PHASE | PHASE_BATTLE, 0, 1, 0, 0);
}
/// `s.ctop(e,tp,...)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // if tc:GetFlagEffect(id)~=0 then
    if card::get_flag_effect(d, tc, ID) != 0 {
        // Duel.GetControl(tc,1-tp)
        duel::get_control(d, CardSet::from_card(tc), 1 - a.tp, 0, 0, Some(0xff), None);
    }
}
