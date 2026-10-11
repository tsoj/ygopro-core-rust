//! Traptrix Genlisea
use super::prelude::*;

pub const ID: u32 = 88078306;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_series={SET_TRAP_HOLE,SET_HOLE}
    listed_series: &[SET_TRAP_HOLE, SET_HOLE],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Unaffected by "Hole" normal trap cards
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    // e1:SetValue(s.efilter)
    effect::set_value_fn(d, e1, lua_fn(efilter));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Set 2 "Hole" Normal Traps (from deck and GY) with different names
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e2, Some(cost::self_tribute()));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.efilter(e,te)`
fn efilter(d: &mut Duel, a: Args) -> bool {
    // local c=te:GetHandler()
    let c = effect::get_handler(d, a.e(1)).nil();
    // return c:IsNormalTrap() and (c:IsSetCard(SET_TRAP_HOLE) or c:IsSetCard(SET_HOLE))
    card::is_normal_trap(d, c) && (card::is_set_card(d, c, &[SET_TRAP_HOLE as u16]) || card::is_set_card(d, c, &[SET_HOLE as u16]))
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsNormalTrap() and c:IsSSetable() and (c:IsSetCard(SET_TRAP_HOLE) or c:IsSetCard(SET_HOLE))
    card::is_normal_trap(d, c)
        && card::is_s_setable(d, c, false, None)
        && (card::is_set_card(d, c, &[SET_TRAP_HOLE as u16]) || card::is_set_card(d, c, &[SET_HOLE as u16]))
}
/// `s.setcheck(sg,e,tp,mg)`
fn setcheck(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetClassCount(Card.GetLocation)>=#sg and sg:GetClassCount(Card.GetCode)>=#sg
    let res = group::get_class_count(d, sg, &|d, c| card::get_location(d, c) as i64) >= sg.len()
        && group::get_class_count(d, sg, &|d, c| card::get_code(d, c)[0] as i64) >= sg.len();
    (res, false)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then
    if a.chk == Some(0) {
        // local sg=Duel.GetMatchingGroup(s.setfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,nil)
        let sg = duel::get_matching_group(d, Some(&setfilter), tp, LOCATION_DECK | LOCATION_GRAVE, 0, NO_EX);
        // return Duel.GetLocationCount(tp,LOCATION_SZONE)>=2 and aux.SelectUnselectGroup(sg,e,tp,2,2,s.setcheck,0) end
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 >= 2
            && matches!(aux::select_unselect_group(d, &sg, Some(e), tp, Some(2), Some(2), Some(&setcheck), 0, tp, None, None, None, false), Err(true));
    }
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, tp, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<2 then return end
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 < 2 {
        return;
    }
    // local sg=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.setfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,nil)
    let nv = aux::necro_valley_filter(setfilter);
    let sg = duel::get_matching_group(d, Some(&nv), tp, LOCATION_DECK | LOCATION_GRAVE, 0, NO_EX);
    // if #sg==0 then return end
    if sg.is_empty() {
        return;
    }
    // local rg=aux.SelectUnselectGroup(sg,e,tp,2,2,s.setcheck,1,tp,HINTMSG_SET,s.setcheck)
    let rg = aux::select_unselect_group(d, &sg, Some(e), tp, Some(2), Some(2), Some(&setcheck), 1, tp, Some(HINTMSG_SET as u64), Some(&setcheck), None, false)
        .unwrap_or_default();
    // if #rg>0 then
    if !rg.is_empty() {
        // Duel.SSet(tp,rg)
        duel::s_set(d, tp, rg.clone(), None, true);
        // for tc in rg:Iter() do
        for tc in group::iter(d, &rg) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
            effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetValue(LOCATION_REMOVED)
            effect::set_value(d, e1, LOCATION_REMOVED as i64);
            // e1:SetReset(RESET_EVENT|RESETS_REDIRECT)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
