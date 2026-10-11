//! 裁きの光
//! Light of Judgment
use super::prelude::*;

pub const ID: u32 = 44595286;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_SANCTUARY_SKY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetHintTiming(0,TIMING_TOHAND|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_TOHAND | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.envfilter(c)`
fn envfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsCode(CARD_SANCTUARY_SKY)
    card::is_faceup(d, c) && card::is_code(d, c, &[CARD_SANCTUARY_SKY])
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.envfilter,e:GetHandlerPlayer(),LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
    //     or Duel.IsEnvironment(CARD_SANCTUARY_SKY)
    let hp = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&envfilter), hp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX)
        || duel::is_environment(d, CARD_SANCTUARY_SKY, None, None)
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsDiscardable() and c:IsAbleToGraveAsCost()
    card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_discardable(d, c, None, None) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,s.cfilter,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, tp, Some(&cfilter), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD|LOCATION_HAND)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, 0, LOCATION_ONFIELD | LOCATION_HAND) > 0;
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g1=Duel.GetFieldGroup(tp,0,LOCATION_ONFIELD)
    let g1 = duel::get_field_group(d, tp, 0, LOCATION_ONFIELD);
    // local g2=Duel.GetFieldGroup(tp,0,LOCATION_HAND)
    let g2 = duel::get_field_group(d, tp, 0, LOCATION_HAND);
    // local opt=0
    let mut opt = 0;
    // if #g1>0 and #g2>0 then
    if !g1.is_empty() && !g2.is_empty() {
        // opt=Duel.SelectOption(tp,aux.Stringid(id,0),aux.Stringid(id,1))+1
        opt = duel::select_option(d, tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1)]) + 1;
    } else if !g1.is_empty() {
        // elseif #g1>0 then opt=1
        opt = 1;
    } else if !g2.is_empty() {
        // elseif #g2>0 then opt=2
        opt = 2;
    }
    if opt == 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // local g=g1:Select(tp,1,1,nil)
        let g = group::select(d, &g1, tp, 1, 1, false, NO_EX).nil();
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    } else if opt == 2 {
        // Duel.ConfirmCards(tp,g2)
        duel::confirm_cards(d, tp, &g2);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // local g=g2:Select(tp,1,1,nil)
        let g = group::select(d, &g2, tp, 1, 1, false, NO_EX).nil();
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
        // Duel.ShuffleHand(1-tp)
        duel::shuffle_hand(d, 1 - tp);
    }
}
