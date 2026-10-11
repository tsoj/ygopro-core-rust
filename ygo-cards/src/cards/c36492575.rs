//! Marincess Sea Horse
//! Scripted by Larry126
use super::prelude::*;

pub const ID: u32 = 36492575;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MARINCESS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself from hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetValue(s.spval)
    effect::set_value_fn(d, e1, lua_fn(spval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 WATER monster from hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(aux.exccon)
    effect::set_condition(d, e2, Some(aux::exccon()));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.spval(e,c)
//   return 0,aux.GetMMZonesPointedTo(c:GetControler(),Card.IsSetCard,nil,nil,nil,SET_MARINCESS)
// end
fn spval(d: &mut Duel, a: Args) -> Value {
    // c:GetControler()
    let p = card::get_controler(d, a.c(1));
    // aux.GetMMZonesPointedTo(..., Card.IsSetCard, ..., SET_MARINCESS)
    let zones = aux::get_mm_zones_pointed_to(d, p, Some(&|d, x| card::is_set_card(d, x, &[SET_MARINCESS as u16])), None, None, None);
    Value::Multi(vec![Value::Int(0), Value::Int(zones as i64)])
}
// function s.spfilter(c,e,tp,zone)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, zone: u32) -> bool {
    // return c:IsAttribute(ATTRIBUTE_WATER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,tp,zone)
    card::is_attribute(d, c, ATTRIBUTE_WATER)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(tp), Some(zone))
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local zone=aux.GetMMZonesPointedTo(tp,Card.IsSetCard,nil,nil,nil,SET_MARINCESS)
    let zone = aux::get_mm_zones_pointed_to(d, tp, Some(&|d, x| card::is_set_card(d, x, &[SET_MARINCESS as u16])), None, None, None);
    // if chk==0 then return zone~=0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp,zone) end
    if a.chk == Some(0) {
        return zone != 0
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, zone)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local zone=aux.GetMMZonesPointedTo(tp,Card.IsSetCard,nil,nil,nil,SET_MARINCESS)
    let zone = aux::get_mm_zones_pointed_to(d, tp, Some(&|d, x| card::is_set_card(d, x, &[SET_MARINCESS as u16])), None, None, None);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp,zone)
    let g = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, e, tp, zone)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP,zone)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, Some(zone));
    }
}
