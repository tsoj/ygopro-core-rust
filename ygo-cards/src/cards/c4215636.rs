//! Shrine of Mist Valley
use super::prelude::*;

pub const ID: u32 = 4215636;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Once per turn, when a WIND monster(s) is destroyed by a card effect and sent to your Graveyard (except during the Damage Step): You can Special Summon 1 Level 3 or lower WIND monster from your hand or Deck. Its effects are negated
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON | CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD | EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e1, EVENT_DESTROYED);
    // e1:SetRange(LOCATION_FZONE)
    effect::set_range(d, e1, LOCATION_FZONE);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.spconfilter(c,tp)
fn spconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_WIND) and c:IsReason(REASON_EFFECT) and c:IsControler(tp) and c:IsLocation(LOCATION_GRAVE)
    card::is_attribute(d, c, ATTRIBUTE_WIND)
        && card::is_reason(d, c, REASON_EFFECT)
        && card::is_controler(d, c, tp)
        && card::is_location(d, c, LOCATION_GRAVE)
        // and (not c:IsPreviousLocation(LOCATION_MZONE) or c:IsPreviousAttributeOnField(ATTRIBUTE_WIND))
        && (!card::is_previous_location(d, c, LOCATION_MZONE) || card::is_previous_attribute_on_field(d, c, ATTRIBUTE_WIND))
        // and not c:IsPreviousLocation(LOCATION_SZONE)
        && !card::is_previous_location(d, c, LOCATION_SZONE)
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.spconfilter,1,nil,tp)
    group::is_exists(d, &a.eg.clone().nil(), &|d, c| spconfilter(d, c, a.tp), 1, NO_EX)
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelBelow(3) and c:IsAttribute(ATTRIBUTE_WIND) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level_below(d, c, 3) && card::is_attribute(d, c, ATTRIBUTE_WIND) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            // and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,e,tp) end
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,e,tp):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if sc and Duel.SpecialSummonStep(sc,0,tp,tp,false,false,POS_FACEUP) then
    if let Some(sc) = sc {
        if duel::special_summon_step(d, sc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) {
            // --Its effects are negated
            // sc:NegateEffects(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            card::negate_effects(d, sc, h, None, false, None);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
