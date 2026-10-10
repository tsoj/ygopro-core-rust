//! ＲＵＭ－ファントム・フォース
//! Phantom Knights' Rank-Up-Magic Force
use super::prelude::*;

pub const ID: u32 = 88504133;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_THE_PHANTOM_KNIGHTS, SET_RAIDRAPTOR, SET_XYZ_DRAGON],
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
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END));
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsMainPhase()
    let _ = a;
    duel::is_main_phase(d, None)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(100)
    effect::set_label(d, a.e, &[100]);
    // if chk==0 then return true end
    true
}
/// `s.filter1(c,e,tp,sg,exg,dr)`
fn filter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8, sg: &CardSet, exg: &CardSet, dr: i64) -> bool {
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(c)), tp, None, None, REASON_XYZ);
    // return #pg<=1 and exg:IsExists(s.filter2,1,nil,e,tp,c,c:GetRank()+dr,pg,sg)
    let rk = card::get_rank(d, c) as i64 + dr;
    pg.len() <= 1 && group::is_exists(d, exg, &|d, c2| filter2(d, c2, e, tp, c, rk, &pg, sg), 1, NO_EX)
}
/// `s.filter2(c,e,tp,mc,rk,pg,sg)`
fn filter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId, rk: i64, pg: &CardSet, sg: &CardSet) -> bool {
    // if (c.rum_limit and not c.rum_limit(mc,e)) or Duel.GetLocationCountFromEx(tp,tp,sg+mc,c)<=0 then return false end
    let rum_fail = match crate::card_info(d, c).rum_limit {
        Some(rum_limit) => !rum_limit(d, mc, e),
        None => false,
    };
    if rum_fail {
        return false;
    }
    let mut sg_mc = sg.clone();
    group::add_card(&mut sg_mc, &CardSet::from_card(mc));
    if duel::get_location_count_from_ex(d, tp, Some(tp), (&sg_mc).into(), duel::ExtraCard::Card(c), None).0 <= 0 {
        return false;
    }
    // return c:IsRank(rk) and mc:IsCanBeXyzMaterial(c,tp) and (#pg==0 or pg:IsContains(mc))
    card::is_rank(d, c, &[rk as u32]) && card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), None) && (pg.is_empty() || group::is_contains(pg, mc))
}
/// `s.rmfilter(c)`
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_DARK) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true,false)
    card::is_attribute(d, c, ATTRIBUTE_DARK) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.extrafil(c,e,tp)`
fn extrafil(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false) and (c:IsSetCard(SET_THE_PHANTOM_KNIGHTS) or c:IsSetCard(SET_RAIDRAPTOR) or c:IsSetCard(SET_XYZ_DRAGON))
    card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ as u32, tp, false, false, None, None, None)
        && (card::is_set_card(d, c, &[SET_THE_PHANTOM_KNIGHTS as u16])
            || card::is_set_card(d, c, &[SET_RAIDRAPTOR as u16])
            || card::is_set_card(d, c, &[SET_XYZ_DRAGON as u16]))
}
/// `s.fieldfil(c,e)`
fn fieldfil(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_XYZ) and c:IsAttribute(ATTRIBUTE_DARK) and (c:GetRank()>0 or c:IsStatus(STATUS_NO_LEVEL)) and c:IsCanBeEffectTarget(e)
    card::is_faceup(d, c)
        && card::is_type(d, c, TYPE_XYZ)
        && card::is_attribute(d, c, ATTRIBUTE_DARK)
        && (card::get_rank(d, c) > 0 || card::is_status(d, c, STATUS_NO_LEVEL))
        && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.rescon(exg,fg,maxrel)`: the returned `function(sg,e,tp,mg)`
fn rescon(
    d: &mut Duel,
    sg: &CardSet,
    e: Option<EffectId>,
    tp: u8,
    exg: &CardSet,
    fg: &CardSet,
    maxrel: i64,
) -> (bool, bool) {
    let e = e.nil();
    // return fg:IsExists(s.filter1,1,sg,e,tp,sg,exg,#sg),#sg>maxrel
    let valid = group::is_exists(d, fg, &|d, c| filter1(d, c, e, tp, sg, exg, sg.len() as i64), 1, sg.into());
    (valid, (sg.len() as i64) > maxrel)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    // local exg=Duel.GetMatchingGroup(s.extrafil,tp,LOCATION_EXTRA,0,nil,e,tp)
    let exg = duel::get_matching_group(d, Some(&|d, c| extrafil(d, c, e, tp)), tp, LOCATION_EXTRA, 0, NO_EX);
    if let Some(chkc) = a.chkc() {
        // return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.fieldfil(chkc,e) and s.filter1(chkc,e,tp,Group.FromCards(chkc),exg,e:GetLabel())
        let dr = effect::get_label(d, e);
        return card::is_controler(d, chkc, tp)
            && card::is_location(d, chkc, LOCATION_MZONE)
            && fieldfil(d, chkc, e)
            && filter1(d, chkc, e, tp, &CardSet::from_card(chkc), &exg, dr);
    }
    // local g=Duel.GetMatchingGroup(s.rmfilter,tp,LOCATION_GRAVE|LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&rmfilter), tp, LOCATION_GRAVE | LOCATION_MZONE, 0, NO_EX);
    // local fg=Duel.GetMatchingGroup(s.fieldfil,tp,LOCATION_MZONE,0,nil,e)
    let fg = duel::get_matching_group(d, Some(&|d, c| fieldfil(d, c, e)), tp, LOCATION_MZONE, 0, NO_EX);
    // local _,maxrel1=fg:GetMinGroup(Card.GetRank)
    let maxrel1 = group::get_min_group(d, &fg, &|d, c| card::get_rank(d, c) as i64).map_or(0, |(_, v)| v);
    // local _,maxrel2=exg:GetMaxGroup(Card.GetRank)
    let maxrel2 = group::get_max_group(d, &exg, &|d, c| card::get_rank(d, c) as i64).map_or(0, |(_, v)| v);
    // local maxrel=maxrel2-maxrel1
    let maxrel = maxrel2 - maxrel1;
    if a.chk == Some(0) {
        // if e:GetLabel()~=100 then return false end
        if effect::get_label(d, e) != 100 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, e, &[0]);
        // return #fg>0 and #g>0 and #exg>0 and aux.SelectUnselectGroup(g,e,tp,nil,nil,s.rescon(exg,fg,maxrel),0)
        return !fg.is_empty()
            && !g.is_empty()
            && !exg.is_empty()
            && matches!(
                aux::select_unselect_group(
                    d,
                    &g,
                    Some(e),
                    tp,
                    None,
                    None,
                    Some(&|d, sg, e, tp, _mg, _c| rescon(d, sg, e, tp, &exg, &fg, maxrel)),
                    0,
                    tp,
                    None,
                    None,
                    None,
                    false,
                ),
                Err(true)
            );
    }
    // local rg=aux.SelectUnselectGroup(g,e,tp,nil,nil,s.rescon(exg,fg,maxrel),1,tp,HINTMSG_REMOVE,s.rescon(exg,fg,maxrel))
    let rg = aux::select_unselect_group(
        d,
        &g,
        Some(e),
        tp,
        None,
        None,
        Some(&|d, sg, e, tp, _mg, _c| rescon(d, sg, e, tp, &exg, &fg, maxrel)),
        1,
        tp,
        Some(HINTMSG_REMOVE as u64),
        Some(&|d, sg, e, tp, _mg, _c| rescon(d, sg, e, tp, &exg, &fg, maxrel)),
        None,
        false,
    )
    .unwrap_or_default();
    // local dr=#rg
    let dr = rg.len() as i64;
    // Duel.Remove(rg,POS_FACEUP,REASON_COST)
    duel::remove(d, rg.clone(), POS_FACEUP as u8, REASON_COST, None, None);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,aux.AND(s.fieldfil,s.filter1),tp,LOCATION_MZONE,0,1,1,nil,e,tp,rg,exg,dr)
    let _ = duel::select_target(
        d,
        tp,
        Some(&|d, c| fieldfil(d, c, e) && filter1(d, c, e, tp, &rg, &exg, dr)),
        tp,
        LOCATION_MZONE,
        0,
        1,
        1,
        false,
        NO_EX,
    );
    // e:SetLabel(dr)
    effect::set_label(d, e, &[dr]);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local ge1=Effect.CreateEffect(c)
    let ge1 = effect::create_effect(d, c);
    // ge1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, ge1, EFFECT_TYPE_FIELD);
    // ge1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, ge1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // ge1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, ge1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // ge1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, ge1, aux::stringid(ID, 1));
    // ge1:SetTargetRange(1,0)
    effect::set_target_range(d, ge1, 1, 0);
    // ge1:SetTarget(s.splimit)
    effect::set_target(d, ge1, Some(effect_card_fn(splimit)));
    // ge1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, ge1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(ge1,tp)
    duel::register_effect(d, ge1, tp);
    // --lizard check
    // aux.addTempLizardCheck(e:GetHandler(),tp,s.lizfilter)
    aux::add_temp_lizard_check(d, c, tp, Some(effect_card_fn(lizfilter)), None, None, None, None);
    // local tc=Duel.GetFirstTarget()
    // if not tc or tc:IsFacedown() or not tc:IsRelateToEffect(e) or tc:IsControler(1-tp) or tc:IsImmuneToEffect(e) then return end
    let Some(tc) = duel::first_target(d) else { return };
    if card::is_facedown(d, tc)
        || !card::is_relate_to_effect(d, tc, e)
        || card::is_controler(d, tc, 1 - tp)
        || card::is_immune_to_effect(d, tc, e)
    {
        return;
    }
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(tc),tp,nil,nil,REASON_XYZ)
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(tc)), tp, None, None, REASON_XYZ);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.AND(s.extrafil,s.filter2),tp,LOCATION_EXTRA,0,1,1,nil,e,tp,tc,tc:GetRank()+e:GetLabel(),pg,Group.FromCards(tc))
    let rk = card::get_rank(d, tc) as i64 + effect::get_label(d, e);
    let sg = CardSet::from_card(tc);
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&|d, cc| extrafil(d, cc, e, tp) && filter2(d, cc, e, tp, tc, rk, &pg, &sg)),
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
        // sc:SetMaterial(tc)
        card::set_material(d, sc, CardSet::from_card(tc));
        // Duel.Overlay(sc,tc)
        duel::overlay(d, sc, tc, false);
        // Duel.SpecialSummon(sc,SUMMON_TYPE_XYZ,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sc, SUMMON_TYPE_XYZ as u32, tp, tp, false, false, POS_FACEUP as u8, None);
        // sc:CompleteProcedure()
        card::complete_procedure(d, sc);
    }
}
/// `s.splimit(e,c,sump,sumtype,sumpos,targetp,se)`
fn splimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let _ = e;
    // return not c:IsType(TYPE_XYZ) and c:IsLocation(LOCATION_EXTRA)
    !card::is_type(d, c, TYPE_XYZ) && card::is_location(d, c, LOCATION_EXTRA)
}
/// `s.lizfilter(e,c)`
fn lizfilter(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let _ = e;
    // return not c:IsOriginalType(TYPE_XYZ)
    !card::is_original_type(d, c, TYPE_XYZ)
}
