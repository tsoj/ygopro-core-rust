//! Blackwing - Zephyros the Elite
use super::prelude::*;

pub const ID: u32 = 14785765;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_DUEL)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_DUEL as u8);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `aux.FaceupFilter(Card.IsAbleToHandAsCost)`
fn faceup_hand_cost(d: &mut Duel, c: CardId) -> bool {
    aux::faceup_filter(|d, c| card::is_able_to_hand_as_cost(d, c))(d, c)
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chk==0 then
    if a.chk == Some(0) {
        // if ft<0 then return false end
        if ft < 0 {
            return false;
        }
        // if ft==0 then
        if ft == 0 {
            // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsAbleToHandAsCost),tp,LOCATION_MZONE,0,1,nil)
            return duel::is_existing_matching_card(d, Some(&faceup_hand_cost), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
        } else {
            // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsAbleToHandAsCost),tp,LOCATION_ONFIELD,0,1,nil)
            return duel::is_existing_matching_card(d, Some(&faceup_hand_cost), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX);
        }
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // if ft==0 then
    if ft == 0 {
        // local g=Duel.SelectMatchingCard(tp,aux.FaceupFilter(Card.IsAbleToHandAsCost),tp,LOCATION_MZONE,0,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&faceup_hand_cost), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
        // Duel.SendtoHand(g,nil,REASON_COST)
        duel::send_to_hand(d, g, None, REASON_COST, None);
    } else {
        // local g=Duel.SelectMatchingCard(tp,aux.FaceupFilter(Card.IsAbleToHandAsCost),tp,LOCATION_ONFIELD,0,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&faceup_hand_cost), a.tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
        // Duel.SendtoHand(g,nil,REASON_COST)
        duel::send_to_hand(d, g, None, REASON_COST, None);
    }
    true
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, h, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,tp,400)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, a.tp, 400);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
        // Duel.Damage(tp,400,REASON_EFFECT)
        duel::damage(d, a.tp, 400, REASON_EFFECT, false, None);
    }
}
