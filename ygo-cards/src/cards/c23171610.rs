//! Limiter Removal
use super::prelude::*;

pub const ID: u32 = 23171610;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_MACHINE),tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_MACHINE)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    true
}
fn filter2(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_MACHINE) and not c:IsImmuneToEffect(e)
    card::is_faceup(d, c) && card::is_race(d, c, RACE_MACHINE) && !card::is_immune_to_effect(d, c, e)
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local sg=Duel.GetMatchingGroup(s.filter2,tp,LOCATION_MZONE,0,nil,e)
    let sg = duel::get_matching_group(d, Some(&|d, c| filter2(d, c, a.e)), a.tp, LOCATION_MZONE, 0, NO_EX);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local fid=c:GetFieldID()
    let fid = card::get_field_id(d, c);
    // local tc=sg:GetFirst()
    let _ = group::get_first(d, &sg);
    // for tc in aux.Next(sg) do
    for tc in group::iter(d, &sg) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // e1:SetValue(tc:GetAttack()*2)
        let atk = card::get_attack(d, tc) as i64;
        effect::set_value(d, e1, atk * 2);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // tc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1,fid)
        card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, 0, 1, fid as i64, 0);
    }
    // sg:KeepAlive() (groups are values: nothing to do)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetLabel(fid)
    effect::set_label(d, e2, &[fid as i64]);
    // e2:SetLabelObject(sg)
    effect::set_label_object(d, e2, Value::Group(sg.clone()));
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
}
fn desfilter(d: &mut Duel, c: CardId, fid: i64) -> bool {
    // return c:GetFlagEffectLabel(id)==fid
    card::get_flag_effect_label(d, c, ID).first().copied() == Some(fid)
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    // if not g:IsExists(s.desfilter,1,nil,e:GetLabel()) then
    let label = effect::get_label(d, a.e);
    if !group::is_exists(d, &g, &|d, c| desfilter(d, c, label), 1, NO_EX) {
        // g:DeleteGroup()
        // e:Reset()
        effect::reset(d, a.e);
        // return false
        return false;
    } else {
        return true;
    }
}
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    // local dg=g:Filter(s.desfilter,nil,e:GetLabel())
    let label = effect::get_label(d, a.e);
    let dg = group::filter(d, &g, &|d, c| desfilter(d, c, label), NO_EX);
    // g:DeleteGroup()
    // Duel.Destroy(dg,REASON_EFFECT)
    duel::destroy(d, dg, REASON_EFFECT, None, None);
}
