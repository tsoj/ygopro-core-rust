//! Evoltile Megachirella
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 13046291;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 Level 6 or lower FIRE Dinosaur monster from your Deck
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
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.spcost)
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Attach up to 2 Reptile and/or Dinosaur monsters with different names from your hand and/or GY to 1 Dragon Xyz monster you control
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(s.atchcost)
    effect::set_cost(d, e2, Some(cost_target(atchcost)));
    // e2:SetTarget(s.atchtg)
    effect::set_target(d, e2, Some(cost_target(atchtg)));
    // e2:SetOperation(s.atchop)
    effect::set_operation(d, e2, Some(operation(atchop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_DINOSAUR) and c:IsAttribute(ATTRIBUTE_FIRE) and c:IsLevelBelow(6) and c:IsCanBeSpecialSummoned(e,151,tp,false,false,POS_FACEUP)
    card::is_race(d, c, RACE_DINOSAUR)
        && card::is_attribute(d, c, ATTRIBUTE_FIRE)
        && card::is_level_below(d, c, 6)
        && card::is_can_be_special_summoned(d, c, e, 151, tp, false, false, Some(POS_FACEUP as u8), None, None)
}
/// `s.costfilter(c,e,tp,ft)`
fn costfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32) -> bool {
    // return c:IsRace(RACE_REPTILE) and (ft>0 or (c:IsControler(tp) and c:GetSequence()<5)) and (c:IsControler(tp) or c:IsFaceup())
    card::is_race(d, c, RACE_REPTILE)
        && (ft > 0 || (card::is_controler(d, c, tp) && card::get_sequence(d, c) < 5))
        && (card::is_controler(d, c, tp) || card::is_faceup(d, c))
        // and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp)
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return ft>-1 and Duel.CheckReleaseGroupCost(tp,s.costfilter,1,false,nil,nil,e,tp,ft)
    //     and Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return ft > -1
            && duel::check_release_group_cost(d, tp, Some(&|d, c| costfilter(d, c, e, tp, ft)), 1, 1, false, None, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // local sg=Duel.SelectReleaseGroupCost(tp,s.costfilter,1,1,false,nil,nil,e,tp,ft)
    let sg = duel::select_release_group_cost(d, tp, Some(&|d, c| costfilter(d, c, e, tp, ft)), 1, 1, false, None, NO_EX);
    // Duel.Release(sg,REASON_COST)
    duel::release(d, sg, REASON_COST, None);
    // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>-1
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > -1
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // if tc then
    if let Some(tc) = tc {
        // Duel.SpecialSummon(tc,151,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, 151, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.xyzfilter(c)`
fn xyzfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_DRAGON) and c:IsType(TYPE_XYZ) and c:GetOverlayCount()==0
    card::is_faceup(d, c) && card::is_race(d, c, RACE_DRAGON) && card::is_type(d, c, TYPE_XYZ) && card::get_overlay_count(d, c) == 0
}
/// `s.atchcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atchcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local h=e:GetHandler() (inline in the Lua; it has no side effect)
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsAbleToRemoveAsCost()
    //     and Duel.IsExistingTarget(s.xyzfilter,tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(Card.IsRace,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,RACE_REPTILE|RACE_DINOSAUR) end
    if a.chk == Some(0) {
        return card::is_able_to_remove_as_cost(d, h, None)
            && duel::is_existing_target(d, Some(&xyzfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_race(d, c, RACE_REPTILE | RACE_DINOSAUR)), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Remove(e:GetHandler(),POS_FACEUP,REASON_COST)
    duel::remove(d, h, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.atchtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atchtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.xyzfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && xyzfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.xyzfilter,tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(Card.IsRace,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,RACE_REPTILE|RACE_DINOSAUR) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&xyzfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_race(d, c, RACE_REPTILE | RACE_DINOSAUR)), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.xyzfilter,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, a.tp, Some(&xyzfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.atchop(e,tp,eg,ep,ev,re,r,rp)`
fn atchop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and not tc:IsImmuneToEffect(e)
    //     and Duel.IsExistingMatchingCard(Card.IsRace,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,RACE_REPTILE|RACE_DINOSAUR) then
    if card::is_relate_to_effect(d, tc, a.e)
        && card::is_faceup(d, tc)
        && !card::is_immune_to_effect(d, tc, a.e)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_race(d, c, RACE_REPTILE | RACE_DINOSAUR)), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX)
    {
        // local g=Duel.GetMatchingGroup(Card.IsRace,tp,LOCATION_HAND|LOCATION_GRAVE,0,nil,RACE_REPTILE|RACE_DINOSAUR)
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_race(d, c, RACE_REPTILE | RACE_DINOSAUR)), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, NO_EX);
        // local sg=aux.SelectUnselectGroup(g,e,tp,1,2,aux.dncheck,1,tp,HINTMSG_ATTACH)
        if let Ok(sg) = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(1), Some(2), Some(&aux::dncheck()), 1, a.tp, Some(HINTMSG_ATTACH as u64), None, None, false) {
            // Duel.Overlay(tc,sg)
            duel::overlay(d, tc, sg, false);
        }
    }
}
