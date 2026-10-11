//! Dovelgus, Generaider Boss of Iron
//! 鉄の王 ドヴェルグス
//! Scripted by AlphaKretin and Eerie Code
use super::prelude::*;

pub const ID: u32 = 76382116;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GENERAIDER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:SetUniqueOnField(1,0,id)
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_GENERAIDER) or c:IsRace(RACE_MACHINE)
    card::is_set_card(d, c, &[SET_GENERAIDER as u16]) || card::is_race(d, c, RACE_MACHINE as u64)
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return (c:IsSetCard(SET_GENERAIDER) or c:IsRace(RACE_MACHINE)) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    cfilter(d, c) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.check(g)` returns `function(sg,tp) ... end`
fn check(d: &mut Duel, sg: &CardSet, tp: u8, g: &CardSet) -> (bool, bool) {
    // local codes=sg:GetClass(Card.GetCode)
    let codes = group::get_class(d, sg, &|d, c| card::get_code(d, c)[0] as i64);
    // return aux.ChkfMMZ(#sg)(sg,nil,tp)
    //     and g:Filter(aux.NOT(Card.IsCode),nil,table.unpack(codes)):GetClassCount(Card.GetCode)>=#sg
    let (mmz, _) = aux::chkf_mmz(sg.len() as i32)(d, sg, None, tp, &CardSet::new(), None);
    if !mmz {
        return (false, false);
    }
    let filtered = group::filter(d, g, &|d, c| !codes.iter().any(|&code| card::is_code(d, c, &[code as u32])), NO_EX);
    let res = group::get_class_count(d, &filtered, &|d, c| card::get_code(d, c)[0] as i64) >= sg.len();
    (res, false)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_HAND,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, NO_EX);
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,s.check(g),nil) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&cfilter), 1, 1, false, Some(&|d: &mut Duel, sg: &CardSet, tp: u8, _mg: &CardSet| check(d, sg, tp, &g)), NO_EX);
    }
    // local max=5
    let mut max = 5;
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then max=1 end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        max = 1;
    }
    // local rg=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,max,false,s.check(g),nil)
    let rg = duel::select_release_group_cost(d, a.tp, Some(&cfilter), 1, max, false, Some(&|d: &mut Duel, sg: &CardSet, tp: u8, _mg: &CardSet| check(d, sg, tp, &g)), NO_EX);
    // Duel.Release(rg,REASON_COST)
    duel::release(d, rg, REASON_COST, None);
    // local og=Duel.GetOperatedGroup()
    let og = duel::get_operated_group(d);
    // og:KeepAlive()
    // e:SetLabelObject(og)
    effect::set_label_object(d, a.e, Value::Group(og));
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local rg=e:GetLabelObject()
    let rg = effect::get_label_object(d, a.e).as_group().cloned().nil();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,#rg,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, rg.len() as u8, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local rg=e:GetLabelObject()
    let rg = effect::get_label_object(d, a.e).as_group().cloned().nil();
    // local codes=rg:GetClass(Card.GetCode)
    let codes = group::get_class(d, &rg, &|d, c| card::get_code(d, c)[0] as i64);
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_HAND,0,nil,e,tp):Filter(aux.NOT(Card.IsCode),nil,table.unpack(codes))
    let hand = duel::get_matching_group(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, NO_EX);
    let g = group::filter(d, &hand, &|d, c| !codes.iter().any(|&code| card::is_code(d, c, &[code as u32])), NO_EX);
    // local ct=math.min(#rg,Duel.GetLocationCount(tp,LOCATION_MZONE))
    let mut ct = rg.len().min(duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 as usize);
    // if #g>0 and ct>0 then
    if !g.is_empty() && ct > 0 {
        // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ct=1 end
        if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
            ct = 1;
        }
        // local sg=aux.SelectUnselectGroup(g,e,tp,ct,ct,aux.dncheck,1,tp,HINTMSG_SPSUMMON)
        let sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(ct), Some(ct), Some(&aux::dncheck()), 1, a.tp, Some(HINTMSG_SPSUMMON as u64), None, None, false).unwrap_or_default();
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
            duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
        }
    }
    // rg:DeleteGroup()
}
