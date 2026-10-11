//! Rank-Up-Magic Revolution Force
use super::prelude::*;

pub const ID: u32 = 43476205;

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
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `Group.FromCards(c)` where `c` may be nil
fn group_of(c: Option<CardId>) -> CardSet {
    c.map_or_else(CardSet::new, CardSet::from_card)
}
/// `s.filter1(c,e,tp)`
fn filter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // local rk=c:GetRank()
    let rk = card::get_rank(d, c);
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(c)), tp, None, None, REASON_XYZ as u32);
    // return (#pg<=0 or (#pg==1 and pg:IsContains(c))) and rk>0 and c:IsFaceup() and c:IsSetCard(SET_RAIDRAPTOR)
    //   and Duel.IsExistingMatchingCard(s.filter3,tp,LOCATION_EXTRA,0,1,nil,e,tp,c,rk+1,pg)
    (pg.is_empty() || (pg.len() == 1 && group::is_contains(&pg, c)))
        && rk > 0
        && card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_RAIDRAPTOR as u16])
        && duel::is_existing_matching_card(d, Some(&|d, x| filter3(d, x, e, tp, c, rk + 1, &pg)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.filter2(c,e,tp)`
fn filter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // local rk=c:GetRank()
    let rk = card::get_rank(d, c);
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(c)), tp, None, None, REASON_XYZ as u32);
    // return (#pg<=0 or (#pg==1 and pg:IsContains(c))) and rk>0 and c:IsFaceup() and c:GetOverlayCount()==0
    //   and c:IsControlerCanBeChanged() and Duel.IsExistingMatchingCard(s.filter3,tp,LOCATION_EXTRA,0,1,nil,e,tp,c,rk+1,pg)
    (pg.is_empty() || (pg.len() == 1 && group::is_contains(&pg, c)))
        && rk > 0
        && card::is_faceup(d, c)
        && card::get_overlay_count(d, c) == 0
        && card::is_controler_can_be_changed(d, c, false, None)
        && duel::is_existing_matching_card(d, Some(&|d, x| filter3(d, x, e, tp, c, rk + 1, &pg)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.filter3(c,e,tp,mc,rk,pg)`
fn filter3(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId, rk: u32, _pg: &CardSet) -> bool {
    // if c.rum_limit and not c.rum_limit(mc,e) then return false end
    if let Some(rum_limit) = crate::card_info(d, c).rum_limit {
        if !rum_limit(d, mc, e) {
            return false;
        }
    }
    // return mc:IsType(TYPE_XYZ,c,SUMMON_TYPE_XYZ,tp) and c:IsRank(rk)
    //   and c:IsSetCard(SET_RAIDRAPTOR) and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0
    //   and mc:IsCanBeXyzMaterial(c,tp) and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false)
    card::is_type_ctx(d, mc, TYPE_XYZ, Some(c), SUMMON_TYPE_XYZ as u64, Some(tp))
        && card::is_rank(d, c, &[rk])
        && card::is_set_card(d, c, &[SET_RAIDRAPTOR as u16])
        && duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), None)
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if Duel.IsTurnPlayer(tp) then
    if duel::is_turn_player(d, a.tp) {
        // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.filter1(chkc,e,tp) end
        if let Some(chkc) = a.chkc() {
            return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && filter1(d, chkc, a.e, a.tp);
        }
        // if chk==0 then return Duel.IsExistingTarget(s.filter1,tp,LOCATION_MZONE,0,1,nil,e,tp) end
        if a.chk == Some(0) {
            return duel::is_existing_target(d, Some(&|d, c| filter1(d, c, a.e, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
        }
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
        // Duel.SelectTarget(tp,s.filter1,tp,LOCATION_MZONE,0,1,1,nil,e,tp)
        let _ = duel::select_target(d, a.tp, Some(&|d, c| filter1(d, c, a.e, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // else
    } else {
        // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE) and s.filter2(chkc,e,tp) end
        if let Some(chkc) = a.chkc() {
            return card::is_controler(d, chkc, 1 - a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && filter2(d, chkc, a.e, a.tp);
        }
        // if chk==0 then return Duel.IsExistingTarget(s.filter2,tp,0,LOCATION_MZONE,1,nil,e,tp) end
        if a.chk == Some(0) {
            return duel::is_existing_target(d, Some(&|d, c| filter2(d, c, a.e, a.tp)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
        }
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_CONTROL)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON + CATEGORY_CONTROL);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
        // local g=Duel.SelectTarget(tp,s.filter2,tp,0,LOCATION_MZONE,1,1,nil,e,tp)
        let g = duel::select_target(d, a.tp, Some(&|d, c| filter2(d, c, a.e, a.tp)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_CONTROL,g,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_CONTROL, g, 1, 0, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if Duel.IsTurnPlayer(1-tp) then
    if duel::is_turn_player(d, 1 - a.tp) {
        // if not tc or not tc:IsRelateToEffect(e) or not tc:IsFaceup() or Duel.GetControl(tc,tp)==0 then return end
        let Some(t) = tc else {
            return;
        };
        if !card::is_relate_to_effect(d, t, a.e) || !card::is_faceup(d, t) || duel::get_control(d, t, a.tp, 0, 0, None, None) == 0 {
            return;
        }
        // Duel.BreakEffect()
        duel::break_effect(d);
    }
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(tc),tp,nil,nil,REASON_XYZ)
    let pg = aux::get_must_be_material_group_opt(d, a.tp, Some(&group_of(tc)), a.tp, None, None, REASON_XYZ as u32);
    // if not tc or tc:IsFacedown() or not tc:IsRelateToEffect(e) or tc:IsControler(1-tp) or tc:IsImmuneToEffect(e)
    //   or #pg>1 or (#pg==1 and not pg:IsContains(tc)) then return end
    let Some(t) = tc else {
        return;
    };
    if card::is_facedown(d, t)
        || !card::is_relate_to_effect(d, t, a.e)
        || card::is_controler(d, t, 1 - a.tp)
        || card::is_immune_to_effect(d, t, a.e)
        || pg.len() > 1
        || (pg.len() == 1 && !group::is_contains(&pg, t))
    {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter3,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,tc,tc:GetRank()+1,pg)
    let rk = card::get_rank(d, t) + 1;
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| filter3(d, x, a.e, a.tp, t, rk, &pg)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // local sc=g:GetFirst()
    let sc = group::get_first(d, &g);
    // if sc then
    if let Some(sc) = sc {
        // sc:SetMaterial(Group.FromCards(tc))
        card::set_material(d, sc, CardSet::from_card(t));
        // Duel.Overlay(sc,tc)
        duel::overlay(d, sc, t, false);
        // Duel.SpecialSummon(sc,SUMMON_TYPE_XYZ,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sc, SUMMON_TYPE_XYZ, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        // sc:CompleteProcedure()
        card::complete_procedure(d, sc);
    }
}
