//! Link Spider
use super::prelude::*;

pub const ID: u32 = 98978921;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_NORMAL),1,1)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_NORMAL, sc, st, Some(tp)))), 1, Some(1), None, None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,e,tp,zone)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, zone: u32) -> bool {
    // return c:IsLevelBelow(4) and c:IsType(TYPE_NORMAL) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,tp,zone)
    card::is_level_below(d, c, 4) && card::is_type(d, c, TYPE_NORMAL) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(tp), Some(zone))
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local zone=e:GetHandler():GetLinkedZone(tp)
        let h = effect::get_handler(d, a.e).nil();
        let zone = card::get_linked_zone(d, h, Some(a.tp));
        // return zone~=0 and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND,0,1,nil,e,tp,zone)
        return zone != 0 && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, a.tp, zone)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    // local zone=e:GetHandler():GetLinkedZone(tp)
    let h = effect::get_handler(d, a.e).nil();
    let zone = card::get_linked_zone(d, h, Some(a.tp));
    // if zone==0 then return end
    if zone == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_HAND,0,1,1,nil,e,tp,zone)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| filter(d, c, a.e, a.tp, zone)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP,zone)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, Some(zone));
    }
}
