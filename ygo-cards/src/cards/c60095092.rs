//! Vaylantz Wakening - Solo Activation
use super::prelude::*;

pub const ID: u32 = 60095092;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_VAYLANTZ],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.pztg)
    effect::set_target(d, e1, Some(cost_target(pztg)));
    // e1:SetOperation(s.pzop)
    effect::set_operation(d, e1, Some(operation(pzop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
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
    // e2:SetCondition(function() return Duel.GetFieldGroupCount(0,LOCATION_FZONE,LOCATION_FZONE)>0 end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.mmvtg)
    effect::set_target(d, e2, Some(cost_target(mmvtg)));
    // e2:SetOperation(s.mmvop)
    effect::set_operation(d, e2, Some(operation(mmvop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.pzfilter(c)`
fn pzfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_VAYLANTZ) and c:IsType(TYPE_PENDULUM) and not c:IsForbidden()
    card::is_set_card(d, c, &[SET_VAYLANTZ as u16]) && card::is_type(d, c, TYPE_PENDULUM) && !card::is_forbidden(d, c)
}
/// `s.pztg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn pztg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.pzfilter,tp,LOCATION_DECK,0,1,nil)
    //   and Duel.CheckPendulumZones(tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&pzfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX) && duel::check_pendulum_zones(d, a.tp);
    }
    true
}
/// `s.pzop(e,tp,eg,ep,ev,re,r,rp)`
fn pzop(d: &mut Duel, a: &EffArgs) {
    // if not Duel.CheckPendulumZones(tp) then return end
    if !duel::check_pendulum_zones(d, a.tp) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOFIELD as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.pzfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&pzfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if tc then
    if let Some(tc) = group::get_first(d, &sel) {
        // Duel.MoveToField(tc,tp,tp,LOCATION_PZONE,POS_FACEUP,true)
        duel::move_to_field(d, tc, a.tp, a.tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
    }
}
/// `function() return Duel.GetFieldGroupCount(0,LOCATION_FZONE,LOCATION_FZONE)>0 end`
fn e2_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::get_field_group_count(d, 0, LOCATION_FZONE, LOCATION_FZONE) > 0
}
/// `s.mmvfilter(c)`
fn mmvfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_VAYLANTZ) and c:CheckAdjacent()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_VAYLANTZ as u16]) && card::check_adjacent(d, c)
}
/// `s.mmvtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn mmvtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.mmvfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && mmvfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.mmvfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&mmvfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.mmvfilter,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, a.tp, Some(&mmvfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.mmvop(e,tp,eg,ep,ev,re,r,rp)`
fn mmvop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsControler(tp) and not tc:IsImmuneToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_controler(d, tc, a.tp) && !card::is_immune_to_effect(d, tc, a.e) {
        // tc:MoveAdjacent()
        card::move_adjacent(d, tc, None);
    }
}
