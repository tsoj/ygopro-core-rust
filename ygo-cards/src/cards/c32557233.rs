//! Rikka Tranquility
use super::prelude::*;

pub const ID: u32 = 32557233;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RIKKA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
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
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // e1:SetLabel(0)
    effect::set_label(d, e1, &[0]);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spfilter1(c,e,tp,check)`
fn spfilter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8, check: i64) -> bool {
    // return c:IsSetCard(SET_RIKKA) and c:IsMonster() and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    //   and (check==0 or Duel.IsExistingMatchingCard(s.spfilter2,tp,LOCATION_GRAVE,0,1,c,e,tp))
    card::is_set_card(d, c, &[SET_RIKKA as u16])
        && card::is_monster(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
        && (check == 0 || duel::is_existing_matching_card(d, Some(&|d, x| spfilter2(d, x, e, tp)), tp, LOCATION_GRAVE, 0, 1, c.into()))
}
/// `s.spfilter2(c,e,tp)`
fn spfilter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_PLANT) and c:IsMonster() and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_race(d, c, RACE_PLANT)
        && card::is_monster(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsRikkaReleasable(tp) and Duel.GetMZoneCount(tp,c)>1
    aux::is_rikka_releasable(d, c, tp) && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 1
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // local a=Duel.IsExistingMatchingCard(s.spfilter1,tp,LOCATION_GRAVE,0,1,nil,e,tp,0)
    let ex_a = duel::is_existing_matching_card(d, Some(&|d, x| spfilter1(d, x, a.e, a.tp, 0)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    // local b=Duel.IsExistingMatchingCard(s.spfilter1,tp,LOCATION_GRAVE,0,1,nil,e,tp,1)
    let b = duel::is_existing_matching_card(d, Some(&|d, x| spfilter1(d, x, a.e, a.tp, 1)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and (a or b) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && (ex_a || b);
    }
    // if b and Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,nil,nil,tp)
    //   and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if b
        && duel::check_release_group_cost(d, a.tp, Some(&|d, x| cfilter(d, x, a.tp)), 1, 1, false, None, NO_EX)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1))
    {
        // local g=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,1,false,nil,nil,tp)
        let g = duel::select_release_group_cost(d, a.tp, Some(&|d, x| cfilter(d, x, a.tp)), 1, 1, false, None, NO_EX);
        // Duel.Release(g,REASON_COST)
        duel::release(d, g, REASON_COST, None);
        // e:SetLabel(1)
        effect::set_label(d, a.e, &[1]);
    } else {
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
    }
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(s.spfilter1,tp,LOCATION_GRAVE,0,1,nil,e,tp,0) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter1(d, x, a.e, a.tp, 0)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,e:GetLabel()+1,tp,LOCATION_GRAVE)
    let ct = (effect::get_label(d, a.e) + 1) as u8;
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, ct, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let label = effect::get_label(d, a.e);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter1,tp,LOCATION_GRAVE,0,1,1,nil,e,tp,e:GetLabel())
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| spfilter1(d, x, a.e, a.tp, label)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP_DEFENSE)~=0 then
    if !g.is_empty() && duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None) != 0 {
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 or e:GetLabel()==0 then return end
        if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 || effect::get_label(d, a.e) == 0 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local g2=Duel.SelectMatchingCard(tp,s.spfilter2,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
        let g2 = duel::select_matching_card(d, a.tp, Some(&|d, x| spfilter2(d, x, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // if #g2>0 then
        if !g2.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SpecialSummon(g2,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
            duel::special_summon(d, g2, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
        }
    }
}
