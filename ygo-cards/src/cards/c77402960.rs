//! Numeron Calling
use super::prelude::*;

pub const ID: u32 = 77402960;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_NUMERON_NETWORK], listed_series: &[SET_NUMERON_GATE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
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
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local fc=Duel.GetFieldCard(tp,LOCATION_FZONE,0)
    let fc = duel::get_field_card(d, a.tp, LOCATION_FZONE, 0);
    // return fc and fc:IsFaceup() and fc:IsCode(CARD_NUMERON_NETWORK) and Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)==0
    match fc {
        Some(fc) => {
            card::is_faceup(d, fc) && card::is_code(d, fc, &[CARD_NUMERON_NETWORK]) && duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0) == 0
        }
        None => false,
    }
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_NUMERON_GATE) and c:IsType(TYPE_XYZ) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_NUMERON_GATE as u16])
        && card::is_type(d, c, TYPE_XYZ)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCountFromEx(tp,tp,nil,TYPE_XYZ)>0
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_EXTRA,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count_from_ex(d, a.tp, Some(a.tp), NO_EX, duel::ExtraCard::Type(TYPE_XYZ), None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.check(sg,e,tp)`
fn check(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return Duel.GetLocationCountFromEx(tp,tp,nil,TYPE_XYZ)>=#sg and aux.dncheck(sg,e,tp)
    let ok = duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Type(TYPE_XYZ), None).0 >= sg.len() as i32
        && (aux::dncheck())(d, sg, e, tp, mg, None).0;
    (ok, false)
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_EXTRA,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, x| filter(d, x, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, NO_EX);
    // local ft=Duel.GetLocationCountFromEx(tp,tp,nil,TYPE_XYZ)
    let ft = duel::get_location_count_from_ex(d, a.tp, Some(a.tp), NO_EX, duel::ExtraCard::Type(TYPE_XYZ), None).0;
    // if ft==0 then return end
    if ft == 0 {
        return;
    }
    // local ct=4
    let mut ct = 4;
    // if ft<ct then ct=ft end
    if ft < ct {
        ct = ft;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ct=1 end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ct = 1;
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,1,ct,s.check,1,tp,HINTMSG_SPSUMMON)
    let sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(1), Some(ct as usize), Some(&check), 1, a.tp, Some(HINTMSG_SPSUMMON as u64), None, None, false)
        .unwrap_or_default();
    // if #sg>0 then
    if !sg.is_empty() {
        // local fid=c:GetFieldID()
        let fid = card::get_field_id(d, c);
        // for tc in aux.Next(sg) do
        for tc in group::iter(d, &sg) {
            // Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon_step(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
            // tc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1,fid)
            card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, 0, 1, fid as i64, 0);
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
        // sg:KeepAlive()
        // local e0=Effect.CreateEffect(c)
        let e0 = effect::create_effect(d, c);
        // e0:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e0, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e0:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e0, EVENT_PHASE + PHASE_END);
        // e0:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
        effect::set_property(d, e0, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        // e0:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e0, RESET_PHASE | PHASE_END, 1);
        // e0:SetCountLimit(1)
        effect::set_count_limit(d, e0, 1, 0, 0, 0);
        // e0:SetLabel(fid)
        effect::set_label(d, e0, &[fid as i64]);
        // e0:SetLabelObject(sg)
        effect::set_label_object(d, e0, Value::Group(sg.clone()));
        // e0:SetCondition(s.rmcon)
        effect::set_condition(d, e0, Some(condition(rmcon)));
        // e0:SetOperation(s.rmop)
        effect::set_operation(d, e0, Some(operation(rmop)));
        // Duel.RegisterEffect(e0,tp)
        duel::register_effect(d, e0, a.tp);
    }
    // local spc=Duel.GetActivityCount(tp,ACTIVITY_SPSUMMON)+Duel.GetActivityCount(tp,ACTIVITY_SUMMON)
    let spc = activity_sum(d, a.tp);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.limittg)
    effect::set_target(d, e1, Some(lua_fn(limittg)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // e1:SetLabel(spc)
    effect::set_label(d, e1, &[spc]);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_CANNOT_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_SUMMON);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetCode(EFFECT_LEFT_SPSUMMON_COUNT)
    effect::set_code(d, e3, EFFECT_LEFT_SPSUMMON_COUNT);
    // e3:SetValue(s.countval)
    effect::set_value_fn(d, e3, lua_fn(countval));
    // Duel.RegisterEffect(e3,tp)
    duel::register_effect(d, e3, a.tp);
    // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,1),nil)
    aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
}
/// `Duel.GetActivityCount(tp,ACTIVITY_SPSUMMON)+Duel.GetActivityCount(tp,ACTIVITY_SUMMON)`
fn activity_sum(d: &mut Duel, tp: u8) -> i64 {
    duel::get_activity_count(d, tp, &[ACTIVITY_SPSUMMON])[0] + duel::get_activity_count(d, tp, &[ACTIVITY_SUMMON])[0]
}
/// `s.rmfilter(c,fid)`
fn rmfilter(d: &mut Duel, c: CardId, fid: i64) -> bool {
    // return c:GetFlagEffectLabel(id)==fid
    card::get_flag_effect_label(d, c, ID).first().copied() == Some(fid)
}
/// `s.rmcon(e,tp,...)`
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned().unwrap_or_default();
    // if not g:IsExists(s.rmfilter,1,nil,e:GetLabel()) then
    let label = effect::get_label(d, a.e);
    if !group::is_exists(d, &g, &|d, c| rmfilter(d, c, label), 1, NO_EX) {
        // g:DeleteGroup()
        // e:Reset()
        effect::reset(d, a.e);
        // return false
        false
    } else {
        // return true
        true
    }
}
/// `s.rmop(e,tp,...)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned().unwrap_or_default();
    // local tg=g:Filter(s.rmfilter,nil,e:GetLabel())
    let label = effect::get_label(d, a.e);
    let tg = group::filter(d, &g, &|d, c| rmfilter(d, c, label), NO_EX);
    // g:DeleteGroup()
    // Duel.Remove(tg,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, tg, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
/// `s.limittg(e,c,tp)`
fn limittg(d: &mut Duel, a: Args) -> bool {
    // local sp=Duel.GetActivityCount(tp,ACTIVITY_SPSUMMON)+Duel.GetActivityCount(tp,ACTIVITY_SUMMON)
    let sp = activity_sum(d, a.p(2));
    // return sp-e:GetLabel()>=1
    sp - effect::get_label(d, a.e(0)) >= 1
}
/// `s.countval(e,re,tp)`
fn countval(d: &mut Duel, a: Args) -> i64 {
    // local sp=Duel.GetActivityCount(tp,ACTIVITY_SPSUMMON)+Duel.GetActivityCount(tp,ACTIVITY_SUMMON)
    let sp = activity_sum(d, a.p(2));
    // if sp-e:GetLabel()>=1 then return 0 else return 1-sp+e:GetLabel() end
    let label = effect::get_label(d, a.e(0));
    if sp - label >= 1 { 0 } else { 1 - sp + label }
}
