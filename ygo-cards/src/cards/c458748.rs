//! The Book of the Law
use super::prelude::*;

pub const ID: u32 = 458748;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_INVOKED], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_label(d, e1, &[0]);
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(100)
    effect::set_label(d, a.e, &[100]);
    // return true
    true
}
// function s.cfilter(c,e,tp)
fn cfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_INVOKED) and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c:GetOriginalAttribute(),c)
    let att = card::get_original_attribute(d, c);
    card::is_set_card(d, c, &[SET_INVOKED as u16])
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, att, Some(c))), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
// function s.spfilter(c,e,tp,att,mc)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, att: u32, mc: Option<CardId>) -> bool {
    // return Duel.GetLocationCountFromEx(tp,tp,mc,c)>0 and c:IsSetCard(SET_INVOKED) and not c:IsOriginalAttribute(att)
    //   and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_FUSION,tp,false,false) and c:CheckFusionMaterial()
    duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_set_card(d, c, &[SET_INVOKED as u16])
        && !card::is_original_attribute(d, c, att)
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_FUSION, tp, false, false, None, None, None)
        && card::check_fusion_material(d, c, None, None, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        // if e:GetLabel()~=100 then return false end
        if effect::get_label(d, a.e) != 100 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,nil,nil,e,tp)
        return duel::check_release_group_cost(d, tp, Some(&|d, c| cfilter(d, c, a.e, tp)), 1, 1, false, None, NO_EX);
    }
    // local rg=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,1,false,nil,nil,e,tp)
    let rg = duel::select_release_group_cost(d, tp, Some(&|d, c| cfilter(d, c, a.e, tp)), 1, 1, false, None, NO_EX);
    // e:SetLabel(rg:GetFirst():GetOriginalAttribute())
    let first = group::get_first(d, &rg).nil();
    let att = card::get_original_attribute(d, first);
    effect::set_label(d, a.e, &[att as i64]);
    // Duel.Release(rg,REASON_COST)
    duel::release(d, rg, REASON_COST, None);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local att=e:GetLabel()
    let att = effect::get_label(d, a.e) as u32;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,att):GetFirst()
    let sg = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp, att, None)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if not tc then return end
    let Some(tc) = group::get_first(d, &sg) else { return };
    // tc:SetMaterial(nil)
    card::set_material(d, tc, CardSet::new());
    // if Duel.SpecialSummon(tc,SUMMON_TYPE_FUSION,tp,tp,false,false,POS_FACEUP)>0 then
    if duel::special_summon(d, tc, SUMMON_TYPE_FUSION, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
        // tc:CompleteProcedure()
        card::complete_procedure(d, tc);
    }
}
