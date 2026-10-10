//! Life Shaver
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 38105306;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    counter_place_list: &[0x208],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:SetUniqueOnField(1,0,id)
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // c:EnableCounterPermit(0x208)
    card::enable_counter_permit(d, c, 0x208, None, None);
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // e0:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e0, 0, Some(TIMING_END_PHASE));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Add counter
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e1, CATEGORY_COUNTER);
    // e1:SetType(EFFECT_TYPE_TRIGGER_F+EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_TRIGGER_F + EFFECT_TYPE_FIELD);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.ctcon)
    effect::set_condition(d, e1, Some(condition(ctcon)));
    // e1:SetOperation(s.ctop)
    effect::set_operation(d, e1, Some(operation(ctop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Discard
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOGRAVE+CATEGORY_HANDES)
    effect::set_category(d, e2, CATEGORY_TOGRAVE + CATEGORY_HANDES);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(s.disccon)
    effect::set_condition(d, e2, Some(condition(disccon)));
    // e2:SetTarget(s.disctg)
    effect::set_target(d, e2, Some(cost_target(disctg)));
    // e2:SetOperation(s.discop)
    effect::set_operation(d, e2, Some(operation(discop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.ctcon(e,tp,eg,ep,ev,re,r,rp)`
fn ctcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp)
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not (c:IsRelateToEffect(e) and c:IsCanAddCounter(0x208,1)) then return end
    if !(card::is_relate_to_effect(d, c, a.e) && card::is_can_add_counter(d, c, 0x208, 1, false, 0)) {
        return;
    }
    // c:AddCounter(0x208,1)
    card::add_counter(d, c, 0x208, 1, false);
}
/// `s.disccon(e,tp,eg,ep,ev,re,r,rp)`
fn disccon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetCounter(0x208)>0 and (Duel.IsMainPhase() or Duel.IsBattlePhase())
    let h = effect::get_handler(d, a.e).nil();
    card::get_counter(d, h, 0x208) > 0 && (duel::is_main_phase(d, None) || duel::is_battle_phase(d, None))
}
/// `s.disctg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn disctg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:GetFlagEffect(id)==0 and c:IsAbleToGrave()
    //     and Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,0,LOCATION_HAND,1,nil,REASON_EFFECT) end
    if a.chk == Some(0) {
        return card::get_flag_effect(d, c, ID) == 0
            && card::is_able_to_grave(d, c, None)
            && duel::is_existing_matching_card(d, Some(&|d, x| card::is_discardable(d, x, Some(REASON_EFFECT), None)), a.tp, 0, LOCATION_HAND, 1, NO_EX);
    }
    // c:RegisterFlagEffect(id,RESET_CHAIN,0,1)
    card::register_flag_effect(d, c, ID, RESET_CHAIN, 0, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(CardSet::from_card(c)), 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,1-tp,c:GetCounter(0x208))
    let ct = card::get_counter(d, c, 0x208);
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, 1 - a.tp, ct);
    true
}
/// `s.discop(e,tp,eg,ep,ev,re,r,rp)`
fn discop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct=c:GetCounter(0x208)
    let ct = card::get_counter(d, c, 0x208);
    // if not c:IsRelateToEffect(e) or ct==0 then return end
    if !card::is_relate_to_effect(d, c, a.e) || ct == 0 {
        return;
    }
    // if Duel.SendtoGrave(c,REASON_EFFECT)>0 and c:IsLocation(LOCATION_GRAVE)
    //     and Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,0,LOCATION_HAND,1,nil,REASON_EFFECT) then
    if duel::send_to_grave(d, c, REASON_EFFECT, None, None) > 0
        && card::is_location(d, c, LOCATION_GRAVE)
        && duel::is_existing_matching_card(d, Some(&|d, x| card::is_discardable(d, x, Some(REASON_EFFECT), None)), a.tp, 0, LOCATION_HAND, 1, NO_EX)
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_DISCARD)
        duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_DISCARD as u64);
        // Duel.DiscardHand(1-tp,Card.IsDiscardable,ct,ct,REASON_EFFECT,nil,REASON_EFFECT)
        duel::discard_hand(d, 1 - a.tp, Some(&|d, x| card::is_discardable(d, x, Some(REASON_EFFECT), None)), ct as u16, ct as u16, REASON_EFFECT, NO_EX);
    }
}
