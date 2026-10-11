//! Purrely Sharely!?
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 10780049;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_PURRELY],
    ..ScriptInfo::DEFAULT
};

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
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.tgfilter(c,e,tp)
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_XYZ) and c:IsSetCard(SET_PURRELY)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp,c)
    card::is_faceup(d, c)
        && card::is_type(d, c, TYPE_XYZ)
        && card::is_set_card(d, c, &[SET_PURRELY as u16])
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, c)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
// function s.spfilter(c,e,tp,xc)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, xc: CardId) -> bool {
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)
    let eg = CardSet::from_card(c);
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&eg), tp, None, None, REASON_XYZ);
    // if not (#pg<=0 or (#pg==1 and pg:IsContains(c))) then return false end
    if !(pg.len() == 0 || (pg.len() == 1 && group::is_contains(&pg, c))) {
        return false;
    }
    // return c:IsLevel(1) and c:IsSetCard(SET_PURRELY) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.IsExistingMatchingCard(s.xyzfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c,xc)
    card::is_level(d, c, &[1])
        && card::is_set_card(d, c, &[SET_PURRELY as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && duel::is_existing_matching_card(d, Some(&|d, x| xyzfilter(d, x, e, tp, c, xc)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
// function s.xyzfilter(c,e,tp,mc,xc)
fn xyzfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId, xc: CardId) -> bool {
    // if c.rum_limit and not c.rum_limit(mc,e) then return false end
    if let Some(rum_limit) = crate::card_info(d, c).rum_limit {
        if !rum_limit(d, mc, e) {
            return false;
        }
    }
    // return c:IsType(TYPE_XYZ) and c:IsSetCard(SET_PURRELY) and c:IsRank(xc:GetRank())
    //     and c:IsAttributeExcept(xc:GetAttribute()) and mc:IsCanBeXyzMaterial(c,tp)
    //     and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0 and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false)
    let xrank = card::get_rank(d, xc);
    let xattr = card::get_attribute(d, xc);
    card::is_type(d, c, TYPE_XYZ)
        && card::is_set_card(d, c, &[SET_PURRELY as u16])
        && card::is_rank(d, c, &[xrank])
        && card::is_attribute_except(d, c, xattr, None, 0, None)
        && card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ, tp, false, false, None, None, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.tgfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && tgfilter(d, chkc, e, tp);
    }
    // if chk==0 then return Duel.IsPlayerCanSpecialSummonCount(tp,2)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_player_can_special_summon_count(d, tp, 2)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| tgfilter(d, c, e, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,0,1,1,nil,e,tp)
    duel::select_target(d, tp, Some(&|d, c| tgfilter(d, c, e, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_EXTRA|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, tp, (LOCATION_EXTRA | LOCATION_DECK) as i32);
    true
}
// function s.attachfilter(c,xc)
fn attachfilter(d: &mut Duel, c: CardId, xc: CardId) -> bool {
    // return xc:GetOverlayGroup():IsExists(s.tgmatfilter,1,nil,c:GetCode()) and not c:IsForbidden()
    let code = card::get_code(d, c)[0];
    let og = card::get_overlay_group(d, xc);
    group::is_exists(d, &og, &|d, x| tgmatfilter(d, x, code), 1, NO_EX) && !card::is_forbidden(d, c)
}
// function s.tgmatfilter(c,code)
fn tgmatfilter(d: &mut Duel, c: CardId, code: u32) -> bool {
    // return c:IsQuickPlaySpell() and c:IsSetCard(SET_PURRELY) and c:IsCode(code)
    card::is_quick_play_spell(d, c) && card::is_set_card(d, c, &[SET_PURRELY as u16]) && card::is_code(d, c, &[code])
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) or tc:IsFacedown() then return end
    if !card::is_relate_to_effect(d, tc, e) || card::is_facedown(d, tc) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local mc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,tc):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, tc)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let mc = group::get_first(d, &sel);
    // if mc and Duel.SpecialSummonStep(mc,0,tp,tp,false,false,POS_FACEUP) then
    if let Some(mc) = mc {
        if duel::special_summon_step(d, mc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
            // mc:NegateEffects(e:GetHandler())
            let h = effect::get_handler(d, e).nil();
            card::negate_effects(d, mc, h, None, false, None);
        }
    }
    // if Duel.SpecialSummonComplete()==0 then return end
    if duel::special_summon_complete(d) == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local xc=Duel.SelectMatchingCard(tp,s.xyzfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,mc,tc):GetFirst()
    let mc = mc.nil();
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| xyzfilter(d, c, e, tp, mc, tc)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let xc = group::get_first(d, &sel);
    // if xc then
    if let Some(xc) = xc {
        // xc:SetMaterial(Group.FromCards(mc))
        card::set_material(d, xc, CardSet::from_card(mc));
        // Duel.Overlay(xc,mc)
        duel::overlay(d, xc, mc, false);
        // Duel.SpecialSummon(xc,SUMMON_TYPE_XYZ,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, xc, SUMMON_TYPE_XYZ, tp, tp, false, false, POS_FACEUP as u8, None);
        // xc:CompleteProcedure()
        card::complete_procedure(d, xc);
    }
    // local tgmatg=Duel.GetMatchingGroup(s.attachfilter,tp,LOCATION_DECK,0,nil,tc)
    let tgmatg = duel::get_matching_group(d, Some(&|d, c| attachfilter(d, c, tc)), tp, LOCATION_DECK, 0, NO_EX);
    // if #tgmatg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if !tgmatg.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_XMATERIAL)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_XMATERIAL as u64);
        // local qc=tgmatg:Select(tp,1,1,nil)
        let qc = group::select(d, &tgmatg, tp, 1, 1, false, NO_EX).nil();
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Overlay(xc,qc)
        duel::overlay(d, xc.nil(), qc, false);
    }
}
