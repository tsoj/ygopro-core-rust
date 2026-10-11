//! Huge Revolution
use super::prelude::*;

pub const ID: u32 = 65396880;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[12143771, 58538870, 85936485], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp) and Duel.IsMainPhase()
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,12143771),tp,LOCATION_ONFIELD,0,1,nil)
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,58538870),tp,LOCATION_ONFIELD,0,1,nil)
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,85936485),tp,LOCATION_ONFIELD,0,1,nil)
    duel::is_turn_player(d, a.tp)
        && duel::is_main_phase(d, None)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[12143771])), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[58538870])), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[85936485])), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local desg=Duel.GetFieldGroup(tp,0,LOCATION_ONFIELD)
    let desg = duel::get_field_group(d, a.tp, 0, LOCATION_ONFIELD);
    // if chk==0 then return #desg>0 or Duel.IsExistingMatchingCard(Card.IsAbleToGrave,tp,0,LOCATION_HAND,1,nil) end
    if a.chk == Some(0) {
        return !desg.is_empty() || duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), a.tp, 0, LOCATION_HAND, 1, NO_EX);
    }
    // local hg=Duel.GetMatchingGroup(Card.IsAbleToGrave,tp,0,LOCATION_HAND,nil)
    let hg = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), a.tp, 0, LOCATION_HAND, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,hg,#hg,tp,0)
    let nh = hg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(hg), nh, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,desg,#desg,tp,0)
    let nd = desg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(desg), nd, a.tp, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local hg=Duel.GetMatchingGroup(Card.IsAbleToGrave,tp,0,LOCATION_HAND,nil)
    let hg = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), a.tp, 0, LOCATION_HAND, NO_EX);
    // if #hg>0 then Duel.SendtoGrave(hg,REASON_EFFECT) end
    if !hg.is_empty() {
        duel::send_to_grave(d, &hg, REASON_EFFECT, None, None);
    }
    // local desg=Duel.GetFieldGroup(tp,0,LOCATION_ONFIELD)
    let desg = duel::get_field_group(d, a.tp, 0, LOCATION_ONFIELD);
    // if #desg>0 then Duel.Destroy(desg,REASON_EFFECT) end
    if !desg.is_empty() {
        duel::destroy(d, desg, REASON_EFFECT, None, None);
    }
}
