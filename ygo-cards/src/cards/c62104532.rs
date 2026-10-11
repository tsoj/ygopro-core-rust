//! The Phantom Knights' Rank-Up-Magic Requiem
//! ＲＵＭ－幻影騎士団レクイエム
use super::prelude::*;

pub const ID: u32 = 62104532;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_THE_PHANTOM_KNIGHTS, SET_XYZ_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.fieldxyzfilter(c,e,tp)`
fn fieldxyzfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(c)), tp, None, None, REASON_XYZ);
    // if not (#pg==0 or (#pg==1 and pg:IsContains(c))) then return false end
    if !(pg.is_empty() || (pg.len() == 1 && group::is_contains(&pg, c))) {
        return false;
    }
    // return c:IsAttribute(ATTRIBUTE_DARK) and c:IsXyzMonster() and c:IsFaceup()
    //     and Duel.IsExistingMatchingCard(s.xyzspfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c,c:GetRank()+1)
    if !(card::is_attribute(d, c, ATTRIBUTE_DARK) && card::is_xyz_monster(d, c) && card::is_faceup(d, c)) {
        return false;
    }
    let rank = card::get_rank(d, c) + 1;
    duel::is_existing_matching_card(d, Some(&|d, sc| xyzspfilter(d, sc, e, tp, c, rank)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.xyzspfilter(c,e,tp,mc,rank)`
fn xyzspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId, rank: u32) -> bool {
    // if c.rum_limit and not c.rum_limit(mc,e) then return false end
    if let Some(rum_limit) = crate::card_info(d, c).rum_limit {
        if !rum_limit(d, mc, e) {
            return false;
        }
    }
    // return c:IsSetCard({SET_THE_PHANTOM_KNIGHTS,SET_XYZ_DRAGON}) and c:IsXyzMonster() and c:IsRank(rank)
    //     and mc:IsCanBeXyzMaterial(c,tp) and mc:IsType(TYPE_XYZ,c,SUMMON_TYPE_XYZ,tp)
    //     and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0 and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false)
    card::is_set_card(d, c, &[SET_THE_PHANTOM_KNIGHTS as u16, SET_XYZ_DRAGON as u16])
        && card::is_xyz_monster(d, c)
        && card::is_rank(d, c, &[rank])
        && card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), None)
        && card::is_type_ctx(d, mc, TYPE_XYZ, Some(c), SUMMON_TYPE_XYZ as u64, Some(tp))
        && duel::get_location_count_from_ex(d, tp, Some(tp), Exclude::Card(mc), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ, tp, false, false, None, None, None)
}
/// `s.gybanspfilter(c,e,tp,field_xyz_chk)`
fn gybanspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, field_xyz_chk: bool) -> bool {
    // return c:IsSetCard({SET_THE_PHANTOM_KNIGHTS,SET_XYZ_DRAGON}) and c:IsFaceup() and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and (field_xyz_chk or s.fieldxyzfilter(c,e,tp))
    card::is_set_card(d, c, &[SET_THE_PHANTOM_KNIGHTS as u16, SET_XYZ_DRAGON as u16])
        && card::is_faceup(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && (field_xyz_chk || fieldxyzfilter(d, c, e, tp))
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local field_xyz_chk=Duel.IsExistingMatchingCard(s.fieldxyzfilter,tp,LOCATION_MZONE,0,1,nil,e,tp)
    let field_xyz_chk = duel::is_existing_matching_card(d, Some(&|d, c| fieldxyzfilter(d, c, a.e, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE|LOCATION_REMOVED) and s.gybanspfilter(chkc,e,tp,field_xyz_chk) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp)
            && card::is_location(d, chkc, LOCATION_GRAVE | LOCATION_REMOVED)
            && gybanspfilter(d, chkc, a.e, tp, field_xyz_chk);
    }
    // if chk==0 then return Duel.IsPlayerCanSpecialSummonCount(tp,2) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.gybanspfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil,e,tp,field_xyz_chk) end
    if a.chk == Some(0) {
        return duel::is_player_can_special_summon_count(d, tp, 2)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| gybanspfilter(d, c, a.e, tp, field_xyz_chk)), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.gybanspfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,1,nil,e,tp,field_xyz_chk)
    let g = duel::select_target(d, tp, Some(&|d, c| gybanspfilter(d, c, a.e, tp, field_xyz_chk)), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,2,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 2, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP) then
    if card::is_relate_to_effect(d, tc, a.e) && duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
        // Negate its effects
        // tc:NegateEffects(e:GetHandler())
        let handler = effect::get_handler(d, a.e).nil();
        card::negate_effects(d, tc, handler, None, false, None);
        // if Duel.SpecialSummonComplete()==0 then return end
        if duel::special_summon_complete(d) == 0 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_XMATERIAL)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_XMATERIAL as u64);
        // local mc=Duel.SelectMatchingCard(tp,s.fieldxyzfilter,tp,LOCATION_MZONE,0,1,1,nil,e,tp):GetFirst()
        let g = duel::select_matching_card(d, tp, Some(&|d, c| fieldxyzfilter(d, c, a.e, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
        let Some(mc) = group::get_first(d, &g) else { return };
        // Duel.HintSelection(mc)
        duel::hint_selection(d, mc, true);
        // if mc:IsImmuneToEffect(e) then return end
        if card::is_immune_to_effect(d, mc, a.e) {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local xyz=Duel.SelectMatchingCard(tp,s.xyzspfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,mc,mc:GetRank()+1):GetFirst()
        let rank = card::get_rank(d, mc) + 1;
        let g = duel::select_matching_card(d, tp, Some(&|d, c| xyzspfilter(d, c, a.e, tp, mc, rank)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        let xyz = group::get_first(d, &g);
        // if xyz then
        if let Some(xyz) = xyz {
            // xyz:SetMaterial(mc)
            card::set_material(d, xyz, CardSet::from_card(mc));
            // Duel.Overlay(xyz,mc)
            duel::overlay(d, xyz, mc, false);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // if Duel.SpecialSummon(xyz,SUMMON_TYPE_XYZ,tp,tp,false,false,POS_FACEUP)>0 then
            if duel::special_summon(d, xyz, SUMMON_TYPE_XYZ, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
                // xyz:CompleteProcedure()
                card::complete_procedure(d, xyz);
            }
        }
    }
}
