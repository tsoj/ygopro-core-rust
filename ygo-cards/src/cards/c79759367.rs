//! Madolche Nights
use super::prelude::*;

pub const ID: u32 = 79759367;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[74641045], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_NEGATE)
    effect::set_category(d, e1, CATEGORY_NEGATE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return re:IsMonsterEffect() and Duel.IsChainNegatable(ev)
    let re = a.re.nil();
    effect::is_monster_effect(d, re) && duel::is_chain_negatable(d, a.ev as u8)
        // and not Duel.IsExistingMatchingCard(Card.IsMonster,tp,LOCATION_GRAVE,0,1,nil)
        && !duel::is_existing_matching_card(d, Some(&|d, c| card::is_monster(d, c)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    true
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsCode(74641045)
    card::is_faceup(d, c) && card::is_code(d, c, &[74641045])
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_ONFIELD,0,1,nil)
    if duel::negate_activation(d, a.ev as u8)
        && duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
        // and Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)>0 then
        && duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) > 0
    {
        // local g=Duel.GetFieldGroup(tp,0,LOCATION_HAND):RandomSelect(tp,1)
        let hand = duel::get_field_group(d, a.tp, 0, LOCATION_HAND);
        let g = group::random_select(d, &hand, a.tp, 1);
        // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
