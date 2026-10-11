//! Legend of Heart
use super::prelude::*;

pub const ID: u32 = 89397517;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_series={SET_LEGENDARY_DRAGON,SET_LEGENDARY_KNIGHT}
    listed_series: &[SET_LEGENDARY_DRAGON as u32, SET_LEGENDARY_KNIGHT as u32],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsRace(RACE_WARRIOR) and Duel.GetMZoneCount(tp,c)>0
    card::is_race(d, c, RACE_WARRIOR) && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // if chk==0 then return Duel.CheckLPCost(tp,2000) and Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,nil,nil,tp) end
    if a.chk == Some(0) {
        return duel::check_lp_cost(d, a.tp, 2000)
            && duel::check_release_group_cost(d, a.tp, Some(&|d, c| cfilter(d, c, a.tp)), 1, 1, false, None, NO_EX);
    }
    // Duel.PayLPCost(tp,2000)
    duel::pay_lp_cost(d, a.tp, 2000);
    // local sg=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,1,false,nil,nil,tp)
    let sg = duel::select_release_group_cost(d, a.tp, Some(&|d, c| cfilter(d, c, a.tp)), 1, 1, false, None, NO_EX);
    // Duel.Release(sg,REASON_COST)
    duel::release(d, sg, REASON_COST, None);
    true
}
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_LEGENDARY_DRAGON) and c:IsSpell() and c:IsAbleToRemove()
    card::is_set_card(d, c, &[SET_LEGENDARY_DRAGON as u16]) && card::is_spell(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_LEGENDARY_KNIGHT) and c:IsMonster() and c:IsCanBeSpecialSummoned(e,0,tp,true,true)
    card::is_set_card(d, c, &[SET_LEGENDARY_KNIGHT as u16])
        && card::is_monster(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, true, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()==0 and Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return false end
        if effect::get_label(d, a.e) == 0 && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return Duel.IsExistingMatchingCard(s.rmfilter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil)
        //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_DECK,0,1,nil,e,tp)
        return duel::is_existing_matching_card(d, Some(&rmfilter), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_GRAVE | LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_GRAVE | LOCATION_DECK) as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft<=0 then return end
    if ft <= 0 {
        return;
    }
    // local rmg=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.rmfilter),tp,LOCATION_HAND|LOCATION_GRAVE,0,nil)
    let rm_filter = aux::necro_valley_filter(rmfilter);
    let rmg = duel::get_matching_group(d, Some(&rm_filter), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, NO_EX);
    // if #rmg==0 then return end
    if rmg.is_empty() {
        return;
    }
    // local rmct=rmg:GetClassCount(Card.GetCode)
    let rmct = group::get_class_count(d, &rmg, &|d, c| card::get_code(d, c)[0] as i64) as i32;
    // local spg=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_DECK,0,nil,e,tp)
    let sp_filter = aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp));
    let spg = duel::get_matching_group(d, Some(&sp_filter), a.tp, LOCATION_HAND | LOCATION_GRAVE | LOCATION_DECK, 0, NO_EX);
    // if #spg==0 then return end
    if spg.is_empty() {
        return;
    }
    // local spct=spg:GetClassCount(Card.GetCode)
    let spct = group::get_class_count(d, &spg, &|d, c| card::get_code(d, c)[0] as i64) as i32;
    // local ct=math.min(3,ft,spct,rmct)
    let ct = 3.min(ft).min(spct).min(rmct);
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // local g=aux.SelectUnselectGroup(rmg,e,tp,1,ct,aux.dncheck,1,tp,HINTMSG_REMOVE)
    let dn = aux::dncheck();
    let g = aux::select_unselect_group(d, &rmg, Some(a.e), a.tp, Some(1), Some(ct as usize), Some(&dn), 1, a.tp, Some(HINTMSG_REMOVE as u64), None, None, false)
        .unwrap_or_default();
    // if #g==0 or Duel.Remove(g,POS_FACEUP,REASON_EFFECT)==0 then return end
    if g.is_empty() || duel::remove(d, g.clone(), POS_FACEUP as u8, REASON_EFFECT, None, None) == 0 {
        return;
    }
    // ct=Duel.GetOperatedGroup():FilterCount(Card.IsLocation,nil,LOCATION_REMOVED)
    let operated = duel::get_operated_group(d);
    let ct = group::filter_count(d, &operated, &|d, c| card::is_location(d, c, LOCATION_REMOVED), NO_EX) as i32;
    // local sg=aux.SelectUnselectGroup(spg,e,tp,ct,ct,aux.dncheck,1,tp,HINTMSG_SPSUMMON)
    let sg = aux::select_unselect_group(d, &spg, Some(a.e), a.tp, Some(ct as usize), Some(ct as usize), Some(&dn), 1, a.tp, Some(HINTMSG_SPSUMMON as u64), None, None, false)
        .unwrap_or_default();
    // if #sg==0 then return end
    if sg.is_empty() {
        return;
    }
    // for tc in sg:Iter() do
    for tc in group::iter(d, &sg) {
        // Duel.SpecialSummonStep(tc,0,tp,tp,true,true,POS_FACEUP)
        duel::special_summon_step(d, tc, 0, a.tp, a.tp, true, true, POS_FACEUP as u8, None);
        // tc:CompleteProcedure()
        card::complete_procedure(d, tc);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
