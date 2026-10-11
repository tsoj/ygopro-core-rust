//! Rank-Up-Magic Soul Shave Force
use super::prelude::*;

pub const ID: u32 = 23581825;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RAIDRAPTOR], ..ScriptInfo::DEFAULT };

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
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetHintTiming(TIMING_DESTROY)
    effect::set_hint_timing(d, e1, TIMING_DESTROY, None);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.PayLPCost(tp,math.floor(Duel.GetLP(tp)/2))
    duel::pay_lp_cost(d, tp, duel::get_lp(d, tp).div_euclid(2) as u32);
    true
}
/// `s.filter1(c,e,tp)`
fn filter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(c)), tp, None, None, REASON_XYZ);
    // local rk=c:GetRank()
    let rk = card::get_rank(d, c) as i64;
    // return #pg<=1 and c:IsSetCard(SET_RAIDRAPTOR) and (c:GetRank()>0 or c:IsStatus(STATUS_NO_LEVEL)) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.IsExistingMatchingCard(s.filter2,tp,LOCATION_EXTRA,0,1,nil,e,tp,c,c:GetRank()+2,pg)
    pg.len() <= 1
        && card::is_set_card(d, c, &[SET_RAIDRAPTOR as u16])
        && (card::get_rank(d, c) > 0 || card::is_status(d, c, STATUS_NO_LEVEL))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && duel::is_existing_matching_card(d, Some(&|d, cc| filter2(d, cc, e, tp, c, rk + 2, &pg)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.filter2(c,e,tp,mc,rk,pg)`
fn filter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId, rk: i64, pg: &CardSet) -> bool {
    // if c.rum_limit and not c.rum_limit(mc,e) then return false end
    let rum_fail = match crate::card_info(d, c).rum_limit {
        Some(rum_limit) => !rum_limit(d, mc, e),
        None => false,
    };
    if rum_fail {
        return false;
    }
    // return c:IsType(TYPE_XYZ) and mc:IsType(TYPE_XYZ,c,SUMMON_TYPE_XYZ,tp) and c:IsRank(rk) and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0
    //     and mc:IsCanBeXyzMaterial(c,tp) and (#pg<=0 or pg:IsContains(mc)) and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false)
    card::is_type(d, c, TYPE_XYZ)
        && card::is_type_ctx(d, mc, TYPE_XYZ, Some(c), SUMMON_TYPE_XYZ as u64, Some(tp))
        && card::is_rank(d, c, &[rk as u32])
        && duel::get_location_count_from_ex(d, tp, Some(tp), (&CardSet::from_card(mc)).into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), None)
        && (pg.is_empty() || group::is_contains(pg, mc))
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ as u32, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    if let Some(chkc) = a.chkc() {
        // return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.filter1(chkc,e,tp)
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && filter1(d, chkc, e, tp);
    }
    if a.chk == Some(0) {
        // return Duel.IsPlayerCanSpecialSummonCount(tp,2) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //     and Duel.IsExistingTarget(s.filter1,tp,LOCATION_GRAVE,0,1,nil,e,tp)
        return duel::is_player_can_special_summon_count(d, tp, 2)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| filter1(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.filter1,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| filter1(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,2,0,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 2, 0, LOCATION_EXTRA as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if not tc or not tc:IsRelateToEffect(e) or tc:IsImmuneToEffect(e) then return end
    let Some(tc) = tc else { return };
    if !card::is_relate_to_effect(d, tc, e) || card::is_immune_to_effect(d, tc, e) {
        return;
    }
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(tc),tp,nil,nil,REASON_XYZ)
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(tc)), tp, None, None, REASON_XYZ);
    // if Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)==0 then return end
    if duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter2,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,tc,tc:GetRank()+2,pg)
    let rk = card::get_rank(d, tc) as i64 + 2;
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&|d, cc| filter2(d, cc, e, tp, tc, rk, &pg)),
        tp,
        LOCATION_EXTRA,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // local sc=g:GetFirst()
    // if sc then
    if let Some(sc) = group::get_first(d, &g) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // sc:SetMaterial(Group.FromCards(tc))
        card::set_material(d, sc, CardSet::from_card(tc));
        // Duel.Overlay(sc,tc)
        duel::overlay(d, sc, tc, false);
        // Duel.SpecialSummon(sc,SUMMON_TYPE_XYZ,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sc, SUMMON_TYPE_XYZ as u32, tp, tp, false, false, POS_FACEUP as u8, None);
        // sc:CompleteProcedure()
        card::complete_procedure(d, sc);
    }
}
