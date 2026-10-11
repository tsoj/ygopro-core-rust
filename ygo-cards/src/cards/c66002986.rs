//! Speedroid Clear Wing Wonder
//! Scripted by The Razgriz
use super::prelude::*;

pub const ID: u32 = 66002986;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy cards your opponent controls up to the number of different Monster Types among the WIND Synchro Monsters you control
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Banish 1 WIND Synchro monster you control, then return it
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp,eg) return eg:IsExists(Card.IsSummonPlayer,1,nil,1-tp) end)
    effect::set_condition(d, e2, Some(condition(cond2)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.rmvrettg)
    effect::set_target(d, e2, Some(cost_target(rmvrettg)));
    // e2:SetOperation(s.rmvretop)
    effect::set_operation(d, e2, Some(operation(rmvretop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.windsynchrofilter(c)`
fn windsynchrofilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_WIND) and c:IsType(TYPE_SYNCHRO) and c:IsFaceup()
    card::is_attribute(d, c, ATTRIBUTE_WIND) && card::is_type(d, c, TYPE_SYNCHRO) && card::is_faceup(d, c)
}
/// `function(e,tp,eg) return eg:IsExists(Card.IsSummonPlayer,1,nil,1-tp) end`
fn cond2(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| card::is_summon_player(d, c, 1 - a.tp), 1, NO_EX)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsControler(1-tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - a.tp);
    }
    // local ct=Duel.GetMatchingGroup(s.windsynchrofilter,tp,LOCATION_MZONE,0,nil):GetBinClassCount(Card.GetRace)
    let g = duel::get_matching_group(d, Some(&windsynchrofilter), a.tp, LOCATION_MZONE, 0, NO_EX);
    let ct = group::get_bin_class_count(d, &g, &|d, c| card::get_race(d, c) as i64);
    // if chk==0 then return ct>0 and Duel.IsExistingTarget(nil,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return ct > 0 && duel::is_existing_target(d, None, a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,0,LOCATION_ONFIELD,1,ct,nil)
    let g = duel::select_target(d, a.tp, None, a.tp, 0, LOCATION_ONFIELD, 1, ct as u16, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, a.tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
/// `s.rmvretfilter(c,tp)`
fn rmvretfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return s.windsynchrofilter(c) and c:IsAbleToRemove() and Duel.GetMZoneCount(tp,c)>0
    windsynchrofilter(d, c) && card::is_able_to_remove(d, c, None, None, None) && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
/// `s.rmvrettg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rmvrettg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.rmvretfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && rmvretfilter(d, chkc, a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.rmvretfilter,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| rmvretfilter(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,s.rmvretfilter,tp,LOCATION_MZONE,0,1,1,nil,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, c| rmvretfilter(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 1, a.tp, 0);
    true
}
/// `s.rmvretop(e,tp,eg,ep,ev,re,r,rp)`
fn rmvretop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.Remove(tc,nil,REASON_EFFECT|REASON_TEMPORARY)>0 and tc:IsLocation(LOCATION_REMOVED)
    //     and not tc:IsReason(REASON_REDIRECT) then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::remove(d, CardSet::from_card(tc), 0, REASON_EFFECT | REASON_TEMPORARY, None, None) > 0
        && card::is_location(d, tc, LOCATION_REMOVED)
        && !card::is_reason(d, tc, REASON_REDIRECT)
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.ReturnToField(tc)
        duel::return_to_field(d, tc, None, None);
    }
}
