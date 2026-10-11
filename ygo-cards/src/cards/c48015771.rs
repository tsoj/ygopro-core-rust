//! Summon Over
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 48015771;

pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[0x148], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(0x148)
    card::enable_counter_permit(d, c, 0x148, None, None);
    // c:SetCounterLimit(0x148,6)
    card::set_counter_limit(d, c, 0x148, 6);
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // add counter
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetOperation(s.ctop)
    effect::set_operation(d, e2, Some(operation(ctop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // indes
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCondition(s.incon)
    effect::set_condition(d, e3, Some(condition(incon)));
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // check
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e4:SetCode(EVENT_PHASE_START|PHASE_MAIN1)
    effect::set_code(d, e4, EVENT_PHASE_START | PHASE_MAIN1);
    // e4:SetRange(LOCATION_FZONE)
    effect::set_range(d, e4, LOCATION_FZONE);
    // e4:SetOperation(s.chkop)
    effect::set_operation(d, e4, Some(operation(chkop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // to gy
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e5, CATEGORY_TOGRAVE);
    // e5:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e5, EFFECT_TYPE_IGNITION);
    // e5:SetProperty(EFFECT_FLAG_BOTH_SIDE)
    effect::set_property(d, e5, EFFECT_FLAG_BOTH_SIDE, 0);
    // e5:SetRange(LOCATION_FZONE)
    effect::set_range(d, e5, LOCATION_FZONE);
    // e5:SetCondition(s.gycon)
    effect::set_condition(d, e5, Some(condition(gycon)));
    // e5:SetTarget(s.gytg)
    effect::set_target(d, e5, Some(cost_target(gytg)));
    // e5:SetOperation(s.gyop)
    effect::set_operation(d, e5, Some(operation(gyop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():AddCounter(0x148,1)
    let h = effect::get_handler(d, a.e).nil();
    card::add_counter(d, h, 0x148, 1, false);
}
/// `s.incon(e)`
fn incon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetCounter(0x148)==6
    let h = effect::get_handler(d, a.e).nil();
    card::get_counter(d, h, 0x148) == 6
}
/// `s.chkop(e,tp,eg,ep,ev,re,r,rp)`
fn chkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:GetCounter(0x148)==6 then
    if card::get_counter(d, c, 0x148) == 6 {
        // c:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1,0,aux.Stringid(id,0))
        card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, aux::stringid(ID, 0));
    }
}
/// `s.gycon(e,tp,eg,ep,ev,re,r,rp)`
fn gycon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetFlagEffect(id)~=0
    let h = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, h, ID) != 0
}
/// `Card.IsSummonType` with `SUMMON_TYPE_SPECIAL` (the extra argument of the filter)
fn is_special_summoned(d: &mut Duel, c: CardId) -> bool {
    card::is_summon_type(d, c, &[SUMMON_TYPE_SPECIAL])
}
/// `s.gytg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn gytg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(Card.IsSummonType,tp,0,LOCATION_MZONE,nil,SUMMON_TYPE_SPECIAL)
    let mut g = duel::get_matching_group(d, Some(&is_special_summoned), a.tp, 0, LOCATION_MZONE, NO_EX);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToGrave() and #g>0 end
    if a.chk == Some(0) {
        return card::is_able_to_grave(d, c, None) && !g.is_empty();
    }
    // g:AddCard(c)
    group::add_card(&mut g, &CardSet::from_card(c));
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g), n, 0, 0);
    true
}
/// `s.gyop(e,tp,eg,ep,ev,re,r,rp)`
fn gyop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(Card.IsSummonType,tp,0,LOCATION_MZONE,nil,SUMMON_TYPE_SPECIAL)
    let g = duel::get_matching_group(d, Some(&is_special_summoned), a.tp, 0, LOCATION_MZONE, NO_EX);
    // if c:IsRelateToEffect(e) and #g>0 and Duel.SendtoGrave(c,REASON_EFFECT)>0 and c:IsLocation(LOCATION_GRAVE) then
    if card::is_relate_to_effect(d, c, a.e)
        && !g.is_empty()
        && duel::send_to_grave(d, CardSet::from_card(c), REASON_EFFECT, None, None) > 0
        && card::is_location(d, c, LOCATION_GRAVE)
    {
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
