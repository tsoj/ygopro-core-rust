//! Rank-Up-Magic Admiration of the Thousands
use super::prelude::*;

pub const ID: u32 = 96142517;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NUMBER_C, SET_CXYZ], ..ScriptInfo::DEFAULT };

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
    // if chk==0 then return Duel.GetActivityCount(tp,ACTIVITY_SPSUMMON)==0 end
    if a.chk == Some(0) {
        return duel::get_activity_count(d, a.tp, &[ACTIVITY_SPSUMMON]).first() == Some(&0);
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.sumlimit)
    effect::set_target(d, e1, Some(lua_fn(sumlimit)));
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // local e2=Effect.CreateEffect(e:GetHandler())
    let e2 = effect::create_effect(d, h);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT+EFFECT_FLAG_OATH)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT + EFFECT_FLAG_OATH, 0);
    // e2:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e2, aux::stringid(ID, 2));
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
    true
}
/// `s.sumlimit(e,c,sump,sumtype,sumpos,targetp,se)`
fn sumlimit(d: &mut Duel, a: Args) -> bool {
    // return e:GetHandler()~=se:GetHandler()
    let h = effect::get_handler(d, a.e(0));
    let seh = effect::get_handler(d, a.e(6)).nil();
    h != Some(seh)
}
/// `s.filter1(c,e,tp)`
fn filter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // local rk=c:GetRank()
    let rk = card::get_rank(d, c);
    // return c:IsType(TYPE_XYZ)
    //     and Duel.IsExistingMatchingCard(s.filter2,tp,0,LOCATION_GRAVE,1,nil,e,tp,rk,Group.FromCards(c))
    if !card::is_type(d, c, TYPE_XYZ) {
        return false;
    }
    let g1 = CardSet::from_card(c);
    duel::is_existing_matching_card(d, Some(&|d, x| filter2(d, x, e, tp, rk, &g1)), tp, 0, LOCATION_GRAVE, 1, NO_EX)
}
/// `s.filter2(c,e,tp,rk,g1)`
fn filter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8, rk: u32, g1: &CardSet) -> bool {
    // local g=g1:Clone()
    // g:AddCard(c)
    let mut g = g1.clone();
    group::add_card(&mut g, &CardSet::from_card(c));
    // return c:IsType(TYPE_XYZ) and c:IsRank(rk) and c:IsCanBeEffectTarget(e)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,rk+1,g)
    card::is_type(d, c, TYPE_XYZ)
        && card::is_rank(d, c, &[rk])
        && card::is_can_be_effect_target(d, c, Some(e))
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, rk + 1, &g)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,tp,rk,g)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, rk: u32, g: &CardSet) -> bool {
    // if c:GetRank()~=rk or (not c:IsSetCard(SET_NUMBER_C) and not c:IsSetCard(SET_CXYZ))
    //     or not c:IsCanBeSpecialSummoned(e,0,tp,false,false) then return false end
    if card::get_rank(d, c) != rk
        || (!card::is_set_card(d, c, &[SET_NUMBER_C as u16]) && !card::is_set_card(d, c, &[SET_CXYZ as u16]))
        || !card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
    {
        return false;
    }
    // return not c.rum_limit or g:IsExists(function(mc) return c.rum_limit(mc,e) end,1,nil) and Duel.GetLocationCountFromEx(tp,tp,g,c)>0
    match crate::card_info(d, c).rum_limit {
        None => true,
        Some(rum_limit) => {
            group::is_exists(d, g, &|d, mc| rum_limit(d, mc, e), 1, NO_EX) && duel::get_location_count_from_ex(d, tp, Some(tp), g.into(), duel::ExtraCard::Card(c), None).0 > 0
        }
    }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.filter1,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| filter1(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local g1=Duel.SelectTarget(tp,s.filter1,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let mut g1 = duel::select_target(d, tp, Some(&|d, c| filter1(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // local rk=g1:GetFirst():GetRank()
    let first = group::get_first(d, &g1).nil();
    let rk = card::get_rank(d, first);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local g2=Duel.SelectTarget(tp,s.filter2,tp,0,LOCATION_GRAVE,1,1,nil,e,tp,rk,g1)
    let g1_snapshot = g1.clone();
    let g2 = duel::select_target(d, tp, Some(&|d, x| filter2(d, x, e, tp, rk, &g1_snapshot)), tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // local g=Duel.GetMatchingGroup(s.filter2,tp,LOCATION_GRAVE,LOCATION_GRAVE,nil,e,tp,rk,g1)
    let mut g = duel::get_matching_group(d, Some(&|d, x| filter2(d, x, e, tp, rk, &g1)), tp, LOCATION_GRAVE, LOCATION_GRAVE, NO_EX);
    // g:Sub(g1)
    group::remove_card(&mut g, &g1);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if g.len() > 0 && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
        // local sg=g:Select(tp,1,99,nil)
        let sg = group::select(d, &g, tp, 1, 99, false, NO_EX).nil();
        // Duel.SetTargetCard(sg)
        duel::set_target_card(d, sg.clone());
        // g1:Merge(sg)
        group::add_card(&mut g1, &sg);
    }
    // Duel.SetTargetParam(rk)
    duel::set_target_param(d, rk);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,g1,#g1,0,0)
    let n = g1.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(g1), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local rk=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM]);
    let rk = info[0].int() as u32;
    // local mg0=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]);
    let mg0 = info[0].as_group().cloned().nil();
    // local mg=mg0:Filter(Card.IsRelateToEffect,nil,e)
    let e = a.e;
    let mg = group::filter(d, &mg0, &|d, c| card::is_relate_to_effect(d, c, e), NO_EX);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 or #mg==0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 || mg.len() == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,rk+1,mg0)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, rk + 1, &mg0)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // local sc=g:GetFirst()
    let sc = group::get_first(d, &g);
    // if sc then
    if let Some(sc) = sc {
        // Duel.Overlay(sc,mg)
        duel::overlay(d, sc, mg, false);
        // Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
