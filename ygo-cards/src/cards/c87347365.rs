//! Revival Golem
use super::prelude::*;

pub const ID: u32 = 87347365;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.condtion)
    effect::set_condition(d, e1, Some(condition(condtion)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condtion(e,tp,eg,ep,ev,re,r,rp)`
fn condtion(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPreviousLocation(LOCATION_DECK)
    let c = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, c, LOCATION_DECK)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local opt=0
    let mut opt: i64 = 0;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local b1=Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    let b1 = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    // local b2=c:IsAbleToHand()
    let b2 = card::is_able_to_hand(d, c, None);
    // if b1 and b2 then
    if b1 && b2 {
        // opt=Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2))+1
        opt = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)]) as i64 + 1;
    } else if b1 {
        // opt=Duel.SelectOption(tp,aux.Stringid(id,1))+1
        opt = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1)]) as i64 + 1;
    } else if b2 {
        // opt=Duel.SelectOption(tp,aux.Stringid(id,2))+2
        opt = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 2)]) as i64 + 2;
    }
    // e:SetLabel(opt)
    effect::set_label(d, a.e, &[opt]);
    // if opt==1 then
    if opt == 1 {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    } else if opt == 2 {
        // e:SetCategory(CATEGORY_TOHAND)
        effect::set_category(d, a.e, CATEGORY_TOHAND);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,e:GetHandler(),1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, 0, 0);
    } else {
        // e:SetCategory(0)
        effect::set_category(d, a.e, 0);
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if e:GetLabel()==1 then
    let label = effect::get_label(d, a.e);
    if label == 1 {
        // if c:IsRelateToEffect(e) then
        if card::is_relate_to_effect(d, c, a.e) {
            // Duel.SpecialSummon(e:GetHandler(),0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    } else if label == 2 {
        // if c:IsRelateToEffect(e) then
        if card::is_relate_to_effect(d, c, a.e) {
            // Duel.SendtoHand(c,nil,REASON_EFFECT)
            duel::send_to_hand(d, c, None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,c)
            duel::confirm_cards(d, 1 - a.tp, c);
        }
    }
}
