//! Ib the World Chalice Priestess
use super::prelude::*;

pub const ID: u32 = 77610772;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_WORLD_CHALICE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,nil,2,nil,s.spcheck)
    link::add_procedure(d, c, None, 2, None, Some(group_check(spcheck)), None, None);
    // indes
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetCondition(s.incon)
    effect::set_condition(d, e1, Some(condition(incon)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e3, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e3:SetValue(aux.tgoval)
    effect::set_value_fn(d, e3, aux::tgoval());
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // destroy replace
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e4:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e4, EFFECT_DESTROY_REPLACE);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetTarget(s.reptg)
    effect::set_target(d, e4, Some(cost_target(reptg)));
    // e4:SetValue(s.repval)
    effect::set_value_fn(d, e4, effect_card_fn(repval));
    // e4:SetOperation(s.repop)
    effect::set_operation(d, e4, Some(operation(repop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // spsummon
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e5, aux::stringid(ID, 1));
    // e5:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e5, CATEGORY_SPECIAL_SUMMON);
    // e5:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e5:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e5, EVENT_TO_GRAVE);
    // e5:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e5, EFFECT_FLAG_DELAY, 0);
    // e5:SetCondition(s.spcon2)
    effect::set_condition(d, e5, Some(condition(spcon2)));
    // e5:SetTarget(s.sptg2)
    effect::set_target(d, e5, Some(cost_target(sptg2)));
    // e5:SetOperation(s.spop2)
    effect::set_operation(d, e5, Some(operation(spop2)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.spcheck(g,lc,sumtype,tp)`
fn spcheck(d: &mut Duel, g: &CardSet, _lc: CardId, _sumtype: u64, _tp: u8) -> bool {
    // return g:CheckDifferentPropertyBinary(Card.GetRace,lc,sumtype,tp) and g:CheckDifferentPropertyBinary(Card.GetAttribute,lc,sumtype,tp)
    group::check_different_property_binary(d, g, &|d, c| card::get_race(d, c))
        && group::check_different_property_binary(d, g, &|d, c| card::get_attribute(d, c) as u64)
}
/// `s.incon(e)`
fn incon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLinked()
    let h = effect::get_handler(d, a.e).nil();
    card::is_linked(d, h)
}
/// `s.repfilter(c,tp,hc)`
fn repfilter(d: &mut Duel, c: CardId, tp: u8, hc: CardId) -> bool {
    // return c:IsFaceup() and c:IsLocation(LOCATION_MZONE) and not c:IsReason(REASON_REPLACE)
    //   and c:IsControler(tp) and c:IsReason(REASON_EFFECT) and hc:GetLinkedGroup():IsContains(c)
    if !(card::is_faceup(d, c) && card::is_location(d, c, LOCATION_MZONE) && !card::is_reason(d, c, REASON_REPLACE)) {
        return false;
    }
    if !(card::is_controler(d, c, tp) && card::is_reason(d, c, REASON_EFFECT)) {
        return false;
    }
    let linked = card::get_linked_group(d, hc);
    group::is_contains(&linked, c)
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsAbleToGrave() and eg:IsExists(s.repfilter,1,nil,tp,e:GetHandler()) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return card::is_able_to_grave(d, h, None) && group::is_exists(d, &eg, &|d, c| repfilter(d, c, a.tp, h), 1, NO_EX);
    }
    // return Duel.SelectEffectYesNo(tp,e:GetHandler(),96)
    duel::select_effect_yes_no(d, a.tp, h, Some(96))
}
/// `s.repval(e,c)`
fn repval(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return s.repfilter(c,e:GetHandlerPlayer(),e:GetHandler())
    let p = effect::get_handler_player(d, e);
    let h = effect::get_handler(d, e).nil();
    repfilter(d, c, p, h)
}
/// `s.repop(e,tp,eg,ep,ev,re,r,rp)`
fn repop(d: &mut Duel, a: &EffArgs) {
    // Duel.SendtoGrave(e:GetHandler(),REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    duel::send_to_grave(d, h, REASON_EFFECT, None, None);
}
/// `s.spcon2(e,tp,eg,ep,ev,re,r,rp)`
fn spcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD)
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, h, LOCATION_ONFIELD)
}
/// `s.spfilter2(c,e,tp)`
fn spfilter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_WORLD_CHALICE) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_WORLD_CHALICE as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg2(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.spfilter2,tp,LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter2(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
/// `s.spop2(e,tp,eg,ep,ev,re,r,rp)`
fn spop2(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter2,tp,LOCATION_HAND,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter2(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
