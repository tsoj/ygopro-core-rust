//! Rank-Up-Magic Barian's Force
use super::prelude::*;

pub const ID: u32 = 47660516;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NUMBER_C, SET_CXYZ], ..ScriptInfo::DEFAULT };

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
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `Group.FromCards(c)` (nil is skipped)
fn from_opt(c: Option<CardId>) -> CardSet {
    c.map_or_else(CardSet::new, CardSet::from_card)
}
/// `s.filter1(c,e,tp)`
fn filter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // local rk=c:GetRank()
    let rk = card::get_rank(d, c);
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(c)), tp, None, None, REASON_XYZ);
    // return (#pg<=0 or (#pg==1 and pg:IsContains(c))) and c:IsFaceup() and (rk>0 or c:IsStatus(STATUS_NO_LEVEL)) and Duel.IsExistingMatchingCard(...)
    (pg.is_empty() || (pg.len() == 1 && pg.contains(c)))
        && card::is_faceup(d, c)
        && (rk > 0 || card::is_status(d, c, STATUS_NO_LEVEL))
        && {
            let race = card::get_race(d, c);
            duel::is_existing_matching_card(d, Some(&|d, x| filter2(d, x, e, tp, c, rk + 1, race, &pg)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
        }
}
/// `s.filter2(c,e,tp,mc,rk,rc,pg)`
fn filter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId, rk: u32, rc: u64, _pg: &CardSet) -> bool {
    // if c.rum_limit and not c.rum_limit(mc,e) then return false end
    if let Some(rum_limit) = crate::card_info(d, c).rum_limit {
        if !rum_limit(d, mc, e) {
            return false;
        }
    }
    // return mc:IsType(TYPE_XYZ,c,SUMMON_TYPE_XYZ,tp) and c:IsRank(rk) and c:IsRace(rc) and (c:IsSetCard({SET_NUMBER_C,SET_CXYZ}))
    card::is_type_ctx(d, mc, TYPE_XYZ, Some(c), SUMMON_TYPE_XYZ as u64, Some(tp))
        && card::is_rank(d, c, &[rk])
        && card::is_race(d, c, rc)
        && card::is_set_card(d, c, &[SET_NUMBER_C as u16, SET_CXYZ as u16])
        // and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0 and mc:IsCanBeXyzMaterial(c,tp) and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false)
        && duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), None)
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ as u32, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && filter1(d, chkc, a.e, a.tp);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| filter1(d, x, a.e, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.filter1,tp,LOCATION_MZONE,0,1,1,nil,e,tp)
    duel::select_target(d, a.tp, Some(&|d, x| filter1(d, x, a.e, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(tc),tp,nil,nil,REASON_XYZ)
    let pg = aux::get_must_be_material_group_opt(d, a.tp, Some(&from_opt(tc)), a.tp, None, None, REASON_XYZ);
    // if not tc or tc:IsFacedown() or not tc:IsRelateToEffect(e) or tc:IsControler(1-tp) or tc:IsImmuneToEffect(e) or #pg>1 or (#pg==1 and not pg:IsContains(tc)) then return end
    let Some(tc) = tc else { return };
    if card::is_facedown(d, tc)
        || !card::is_relate_to_effect(d, tc, a.e)
        || card::is_controler(d, tc, 1 - a.tp)
        || card::is_immune_to_effect(d, tc, a.e)
        || pg.len() > 1
        || (pg.len() == 1 && !pg.contains(tc))
    {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter2,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,tc,tc:GetRank()+1,tc:GetRace(),pg)
    let rk = card::get_rank(d, tc);
    let rc = card::get_race(d, tc);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| filter2(d, x, a.e, a.tp, tc, rk + 1, rc, &pg)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // local sc=g:GetFirst()
    let sc = group::get_first(d, &g);
    if let Some(sc) = sc {
        // sc:SetMaterial(tc)
        card::set_material(d, sc, CardSet::from_card(tc));
        // Duel.Overlay(sc,tc)
        duel::overlay(d, sc, CardSet::from_card(tc), false);
        // Duel.SpecialSummon(sc,SUMMON_TYPE_XYZ,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sc, SUMMON_TYPE_XYZ, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        // sc:CompleteProcedure()
        card::complete_procedure(d, sc);
        // local g1=Duel.GetOverlayGroup(tp,0,1)
        let g1 = duel::get_overlay_group(d, a.tp, 0, 1, None);
        // if #g1>0 then
        if !g1.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,0))
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 0));
            // local mg2=g1:Select(tp,1,1,nil)
            let mg2 = group::select(d, &g1, a.tp, 1, 1, false, NO_EX).nil();
            // local oc=mg2:GetFirst():GetOverlayTarget()
            let first = group::get_first(d, &mg2).nil();
            let oc = card::get_overlay_target(d, first).nil();
            // Duel.Overlay(sc,mg2)
            duel::overlay(d, sc, mg2, false);
            // Duel.RaiseSingleEvent(oc,EVENT_DETACH_MATERIAL,e,0,0,0,0)
            duel::raise_single_event(d, oc, EVENT_DETACH_MATERIAL, Some(a.e), 0, 0, 0, 0);
        }
    }
}
