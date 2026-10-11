//! Inzektor Giga-Cricket
use super::prelude::*;

pub const ID: u32 = 65844845;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_INZEKTOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // equip
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.eqcost)
    effect::set_cost(d, e1, Some(cost_target(eqcost)));
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // equip effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_SET_BASE_ATTACK)
    effect::set_code(d, e2, EFFECT_SET_BASE_ATTACK);
    // e2:SetValue(2000)
    effect::set_value(d, e2, 2000);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    // e3:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e3, EFFECT_PIERCE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsRace(RACE_INSECT) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    //   and Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,c)
    card::is_race(d, c, RACE_INSECT as u64)
        && card::is_able_to_remove_as_cost(d, c, None)
        && aux::sp_elim_filter(d, c, true, false)
        && duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, 0, 1, c.into())
}
/// `s.eqcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn eqcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler() (for the Exclude argument)
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,e:GetHandler(),tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| cfilter(d, x, a.tp)), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,e:GetHandler(),tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| cfilter(d, x, a.tp)), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, c.into()).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_INZEKTOR)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_INZEKTOR as u16])
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //   and Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,e:GetHandler(),1,0,0)
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(c)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 or tc:IsFacedown() or not tc:IsRelateToEffect(e) then
    if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 <= 0
        || card::is_facedown(d, tc.nil())
        || !card::is_relate_to_effect(d, tc.nil(), a.e)
    {
        return;
    }
    // Duel.Equip(tp,c,tc,true)
    duel::equip(d, a.tp, c, tc.nil(), true, false);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_EQUIP_LIMIT)
    effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // e1:SetValue(s.eqlimit)
    effect::set_value_fn(d, e1, effect_card_fn(eqlimit));
    // e1:SetLabelObject(tc)
    effect::set_label_object(d, e1, Value::Card(tc.nil()));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.eqlimit(e,c)`
fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c==e:GetLabelObject()
    effect::get_label_object(d, e).as_card() == Some(c)
}
