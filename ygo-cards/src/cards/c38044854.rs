//! Rise Rank-Up-Magic Raidraptor's Force
//! ЯＲＵＭ－レイド・ラプターズ・フォース
use super::prelude::*;

pub const ID: u32 = 38044854;

// s.listed_series={SET_RAIDRAPTOR}
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RAIDRAPTOR], ..ScriptInfo::DEFAULT };

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
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END as u32 | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(function(e,tp) return Duel.IsMainPhase() or (Duel.IsTurnPlayer(1-tp) and Duel.IsBattlePhase()) end)
    effect::set_condition(d, e1, Some(condition(cond1)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `function(e,tp) return Duel.IsMainPhase() or (Duel.IsTurnPlayer(1-tp) and Duel.IsBattlePhase()) end`
fn cond1(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    duel::is_main_phase(d, None) || (duel::is_turn_player(d, 1 - tp) && duel::is_battle_phase(d, None))
}
/// `s.tgfilter(c,e,tp)`
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsType(TYPE_XYZ) and c:IsSetCard(SET_RAIDRAPTOR) and c:IsFaceup()
    //     and c:IsCanBeXyzMaterial(nil,tp,REASON_EFFECT) and c:IsCanBeEffectTarget(e)
    card::is_type(d, c, TYPE_XYZ)
        && card::is_set_card(d, c, &[SET_RAIDRAPTOR as u16])
        && card::is_faceup(d, c)
        && card::is_can_be_xyz_material(d, c, None, Some(tp), Some(REASON_EFFECT))
        && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.spfilter(c,e,tp,rk)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, rk: u32) -> bool {
    // return c:IsType(TYPE_XYZ) and c:IsSetCard(SET_RAIDRAPTOR) and c:IsRank(rk) and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false)
    //     and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0 and (not c.rum_limit or g:IsExists(function(mc) return c.rum_limit(mc,e) end,1,nil))
    if !(card::is_type(d, c, TYPE_XYZ)
        && card::is_set_card(d, c, &[SET_RAIDRAPTOR as u16])
        && card::is_rank(d, c, &[rk])
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ, tp, false, false, None, None, None))
    {
        return false;
    }
    if duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 <= 0 {
        return false;
    }
    // `g` is a global that is nil here: a monster with a rum_limit makes the Lua index it and fail
    if crate::card_info(d, c).rum_limit.is_some() {
        crate::lua_error("attempt to index a nil value (global 'g')");
    }
    true
}
/// `s.rescon(sg,e,tp,mg)` as a `rescon` of `aux.SelectUnselectGroup`: `(res, false)`.
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8) -> (bool, bool) {
    // return sg:IsExists(Card.IsLocation,1,nil,LOCATION_MZONE) and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,sg:GetSum(Card.GetRank))
    let e = e.nil();
    let has_mzone = group::is_exists(d, sg, &|d, c| card::is_location(d, c, LOCATION_MZONE), 1, NO_EX);
    if !has_mzone {
        return (false, false);
    }
    let rk = group::get_sum(d, sg, &|d, c| card::get_rank(d, c) as i64) as u32;
    let res = duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp, rk)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    (res, false)
}
/// `s.rescon` as the `rescon` argument (`aux.SelectUnselectGroup` passes `e` and `tp` in).
fn rescon_fn(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    rescon(d, sg, e, tp)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| tgfilter(d, c, e, tp)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // if chk==0 then return #g>=2 and aux.SelectUnselectGroup(g,e,tp,2,#g,s.rescon,0) end
    if a.chk == Some(0) {
        return g.len() >= 2
            && matches!(
                aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(g.len()), Some(&rescon_fn), 0, tp, None, None, None, false),
                Err(true)
            );
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,2,#g,s.rescon,1,tp,HINTMSG_XMATERIAL,s.rescon)
    let tg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(g.len()), Some(&rescon_fn), 1, tp, Some(HINTMSG_XMATERIAL as u64), Some(&rescon_fn), None, false)
        .unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    // local gyg=tg:Filter(Card.IsLocation,nil,LOCATION_GRAVE)
    let gyg = group::filter(d, &tg, &|d, c| card::is_location(d, c, LOCATION_GRAVE), NO_EX);
    // if #gyg>0 then
    if !gyg.is_empty() {
        // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,gyg,#gyg,tp,0)
        let n = gyg.len() as u8;
        duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(gyg), n, tp, 0);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, e).nil();
    // local futg=tg:Filter(Card.IsFaceup,nil)
    let futg = group::filter(d, &tg, &|d, c| card::is_faceup(d, c), NO_EX);
    // if #futg<2 then return end
    if futg.len() < 2 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,futg:GetSum(Card.GetRank)):GetFirst()
    let rk = group::get_sum(d, &futg, &|d, c| card::get_rank(d, c) as i64) as u32;
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, rk)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel);
    // if sc and Duel.SpecialSummon(sc,SUMMON_TYPE_XYZ,tp,tp,false,false,POS_FACEUP)>0 then
    if let Some(sc) = sc {
        if duel::special_summon(d, sc, SUMMON_TYPE_XYZ, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
            // sc:CompleteProcedure()
            card::complete_procedure(d, sc);
            // tg=tg:Match(aux.NOT(Card.IsImmuneToEffect),nil,e)
            let mut tg = tg.clone();
            group::match_(d, &mut tg, &|d, c| !card::is_immune_to_effect(d, c, e), NO_EX);
            // if #tg==0 then return end
            if tg.is_empty() {
                return;
            }
            // Duel.Overlay(sc,tg)
            duel::overlay(d, sc, tg, false);
        }
    }
}
