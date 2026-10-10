//! Mausoleum of the Emperor
use super::prelude::*;

pub const ID: u32 = 80921533;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Activate 1 of these effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SUMMON+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_SUMMON + CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_BOTH_SIDE)
    effect::set_property(d, e2, EFFECT_FLAG_BOTH_SIDE, 0);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTarget(s.sumefftg)
    effect::set_target(d, e2, Some(cost_target(sumefftg)));
    // e2:SetOperation(s.sumeffop)
    effect::set_operation(d, e2, Some(operation(sumeffop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Hardcode
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(80921533)
    effect::set_code(d, e3, ID);
    // e3:SetValue(SUMMON_TYPE_NORMAL)
    effect::set_value(d, e3, SUMMON_TYPE_NORMAL as i64);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // e2:SetLabelObject(e3)
    effect::set_label_object(d, e2, Value::Effect(e3));
}
/// `s.sumfilter(c,se,ct)`
fn sumfilter(d: &mut Duel, c: CardId, se: Option<EffectId>, ct: i32) -> bool {
    // if not (c:IsSummonableCard() and c:CanSummonOrSet(false,se)) then return false end
    if !(card::is_summonable_card(d, c) && card::can_summon_or_set(d, c, false, se, 0, None)) {
        return false;
    }
    // local mi,ma=c:GetTributeRequirement()
    let (mi, ma) = card::get_tribute_requirement(d, c);
    // return mi==ct or ma==ct
    mi == ct || ma == ct
}
/// `s.sumefftg(e,tp,...,chk)`
fn sumefftg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local se=e:GetLabelObject()
    let se = effect::get_label_object(d, e).as_effect();
    // local b1=Duel.CheckLPCost(tp,1000) and Duel.IsExistingMatchingCard(s.sumfilter,tp,LOCATION_HAND,0,1,nil,se,1)
    let b1 = duel::check_lp_cost(d, tp, 1000) && duel::is_existing_matching_card(d, Some(&|d, c| sumfilter(d, c, se, 1)), tp, LOCATION_HAND, 0, 1, NO_EX);
    // local b2=Duel.CheckLPCost(tp,2000) and Duel.IsExistingMatchingCard(s.sumfilter,tp,LOCATION_HAND,0,1,nil,se,2)
    let b2 = duel::check_lp_cost(d, tp, 2000) && duel::is_existing_matching_card(d, Some(&|d, c| sumfilter(d, c, se, 2)), tp, LOCATION_HAND, 0, 1, NO_EX);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and (b1 or b2) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && (b1 || b2);
    }
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,1)},{b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]).nil() as u32;
    // Duel.PayLPCost(tp,op*1000)
    duel::pay_lp_cost(d, tp, op * 1000);
    // e:SetLabel(op)
    effect::set_label(d, e, &[op as i64]);
    // Duel.SetOperationInfo(0,CATEGORY_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
/// `s.sumeffop(e,tp,...)`
fn sumeffop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local op=e:GetLabel()
    let op = effect::get_label(d, e);
    // local se=e:GetLabelObject()
    let se = effect::get_label_object(d, e).as_effect();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.sumfilter,tp,LOCATION_HAND,0,1,1,nil,se,op):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, c| sumfilter(d, c, se, op as i32)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if sc then
    if let Some(sc) = sc {
        // Duel.SummonOrSet(tp,sc,false,se)
        duel::summon_or_set(d, tp, sc, false, se, 0, None);
    }
}
