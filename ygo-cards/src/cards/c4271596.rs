//! King of the Ashened City
use super::prelude::*;

pub const ID: u32 = 4271596;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_OBSIDIM_ASHENED_CITY, ID], listed_series: &[SET_ASHENED], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card (from your hand)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.selfspcon)
    effect::set_condition(d, e1, Some(lua_fn(selfspcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Ashened" monster from your hand or Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.hdsptg)
    effect::set_target(d, e2, Some(cost_target(hdsptg)));
    // e2:SetOperation(s.hdspop)
    effect::set_operation(d, e2, Some(operation(hdspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.selfspcon(e,c)`
fn selfspcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // return Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_OBSIDIM_ASHENED_CITY),0,LOCATION_FZONE,LOCATION_FZONE,1,nil)
    let tp = card::get_controler(d, c);
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&faceup_obsidim), 0, LOCATION_FZONE, LOCATION_FZONE, 1, NO_EX)
}
/// `aux.FaceupFilter(Card.IsCode,CARD_OBSIDIM_ASHENED_CITY)`
fn faceup_obsidim(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_code(d, c, &[CARD_OBSIDIM_ASHENED_CITY])
}
/// `aux.FaceupFilter(Card.IsAttackAbove,2800)`
fn faceup_atk_2800(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_attack_above(d, c, 2800)
}
/// `s.hdspfilter(c,e,tp)`
fn hdspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_ASHENED) and not c:IsCode(id) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_ASHENED as u16]) && !card::is_code(d, c, &[ID]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.hdsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn hdsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local sum_loc=LOCATION_HAND
    let mut sum_loc: u32 = LOCATION_HAND;
    // if Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsAttackAbove,2800),tp,0,LOCATION_MZONE,1,nil) then
    if duel::is_existing_matching_card(d, Some(&faceup_atk_2800), tp, 0, LOCATION_MZONE, 1, NO_EX) {
        // sum_loc=sum_loc|LOCATION_DECK
        sum_loc |= LOCATION_DECK;
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.hdspfilter,tp,sum_loc,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| hdspfilter(d, c, e, tp)), tp, sum_loc, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,sum_loc)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, sum_loc as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
/// `s.hdspop(e,tp,eg,ep,ev,re,r,rp)`
fn hdspop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local sum_loc=LOCATION_HAND
    let mut sum_loc: u32 = LOCATION_HAND;
    // if Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsAttackAbove,2800),tp,0,LOCATION_MZONE,1,nil) then
    if duel::is_existing_matching_card(d, Some(&faceup_atk_2800), tp, 0, LOCATION_MZONE, 1, NO_EX) {
        // sum_loc=sum_loc|LOCATION_DECK
        sum_loc |= LOCATION_DECK;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.hdspfilter,tp,sum_loc,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| hdspfilter(d, c, e, tp)), tp, sum_loc, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
