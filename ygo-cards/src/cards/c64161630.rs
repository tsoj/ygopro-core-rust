//! Purge Ray
//! パージ・レイ
use super::prelude::*;

pub const ID: u32 = 64161630;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NUMBER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c,e,tp)`
fn cfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // local rk=c:GetRank()
    let rk = card::get_rank(d, c);
    // return rk>1 and c:IsType(TYPE_XYZ)
    //   and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_EXTRA,0,1,nil,rk-1,c:GetRace(),e,tp,c)
    if !(rk > 1 && card::is_type(d, c, TYPE_XYZ)) {
        return false;
    }
    let rc = card::get_race(d, c);
    duel::is_existing_matching_card(d, Some(&|d, cc| filter(d, cc, rk - 1, rc, e, tp, Some(c))), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.filter(c,rk,rc,e,tp,mc)`
fn filter(d: &mut Duel, c: CardId, rk: u32, rc: u64, e: EffectId, tp: u8, mc: Option<CardId>) -> bool {
    // return c:IsType(TYPE_XYZ) and c:IsSetCard(SET_NUMBER) and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0
    //   and c:GetRank()==rk and c:IsRace(rc) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    if !(card::is_type(d, c, TYPE_XYZ) && card::is_set_card(d, c, &[SET_NUMBER as u16])) {
        return false;
    }
    let ex = match mc {
        Some(mc) => mc.into(),
        None => NO_EX,
    };
    duel::get_location_count_from_ex(d, tp, Some(tp), ex, duel::ExtraCard::Card(c), None).0 > 0
        && card::get_rank(d, c) == rk
        && card::is_race(d, c, rc)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(100)
    effect::set_label(d, a.e, &[100]);
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,nil,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&|d, c| cfilter(d, c, e, tp)), 1, 1, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,1,false,nil,nil,e,tp)
    let g = duel::select_release_group_cost(d, tp, Some(&|d, c| cfilter(d, c, e, tp)), 1, 1, false, None, NO_EX);
    // e:SetLabel(g:GetFirst():GetRank())
    let tc = group::get_first(d, &g).nil();
    let rank = card::get_rank(d, tc) as i64;
    effect::set_label(d, e, &[rank]);
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // e1:SetLabel(g:GetFirst():GetRace())
    let race = card::get_race(d, tc) as i64;
    effect::set_label(d, e1, &[race]);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // e:SetLabelObject(e1)
    effect::set_label_object(d, e, Value::Effect(e1));
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()~=100 then return false end
        if effect::get_label(d, a.e) != 100 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return true
        return true;
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // e1:SetLabel(e:GetLabel())
    let label = effect::get_label(d, a.e);
    effect::set_label(d, e1, &[label]);
    // e1:SetLabelObject(e:GetLabelObject())
    let lo = effect::get_label_object(d, a.e);
    effect::set_label_object(d, e1, lo);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // local rk=e:GetLabel()
    let rk = effect::get_label(d, a.e);
    // local rc=e:GetLabelObject():GetLabel()
    let rc_effect = effect::get_label_object(d, a.e).as_effect().nil();
    let rc = effect::get_label(d, rc_effect);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_EXTRA,0,1,1,nil,rk-1,rc,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| filter(d, c, (rk - 1) as u32, rc as u64, e, tp, None)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
}
