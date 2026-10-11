//! Old Entity Hastorr
use super::prelude::*;

pub const ID: u32 = 70913714;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --synchro summon
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // --equip
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    // e1:SetCondition(s.eqcon)
    effect::set_condition(d, e1, Some(condition(eqcon)));
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.eqcon(e,tp,eg,ep,ev,re,r,rp)`
fn eqcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPreviousLocation(LOCATION_MZONE)
    let c = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, c, LOCATION_MZONE)
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler() (only used below)
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //   and Duel.IsExistingTarget(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, x| card::is_faceup(d, x)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // Duel.SelectTarget(tp,Card.IsFaceup,tp,0,LOCATION_MZONE,1,1,nil)
    duel::select_target(d, a.tp, Some(&|d, x| card::is_faceup(d, x)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,e:GetHandler(),1,0,0)
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(c)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.eqlimit(e,c)`: `return e:GetOwner()==c`
fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    Some(c) == effect::get_owner(d, e)
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, tc.nil()) && card::is_relate_to_effect(d, tc.nil(), a.e) {
        let tc = tc.nil();
        // Duel.Equip(tp,c,tc,true)
        duel::equip(d, a.tp, c, tc, true, false);
        // --Add Equip limit
        // local e1=Effect.CreateEffect(tc)
        let e1 = effect::create_effect(d, tc);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_EQUIP_LIMIT)
        effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // e1:SetValue(s.eqlimit)
        effect::set_value_fn(d, e1, effect_card_fn(eqlimit));
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_EQUIP)
        effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
        // e2:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e2, EFFECT_DISABLE);
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
        // local e3=e2:Clone()
        let e3 = effect::clone(d, e2);
        // e3:SetCode(EFFECT_CANNOT_ATTACK)
        effect::set_code(d, e3, EFFECT_CANNOT_ATTACK);
        // c:RegisterEffect(e3)
        card::register_effect(d, c, e3, false);
        // --control
        // local e4=Effect.CreateEffect(c)
        let e4 = effect::create_effect(d, c);
        // e4:SetDescription(aux.Stringid(id,1))
        effect::set_description(d, e4, aux::stringid(ID, 1));
        // e4:SetCategory(CATEGORY_CONTROL)
        effect::set_category(d, e4, CATEGORY_CONTROL);
        // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
        effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
        // e4:SetCode(EVENT_LEAVE_FIELD)
        effect::set_code(d, e4, EVENT_LEAVE_FIELD);
        // e4:SetCondition(s.ctcon)
        effect::set_condition(d, e4, Some(condition(ctcon)));
        // e4:SetTarget(s.cttg)
        effect::set_target(d, e4, Some(cost_target(cttg)));
        // e4:SetOperation(s.ctop)
        effect::set_operation(d, e4, Some(operation(ctop)));
        // e4:SetReset(RESET_EVENT|RESET_TURN_SET|RESET_TOFIELD)
        effect::set_reset(d, e4, RESET_EVENT | RESET_TURN_SET | RESET_TOFIELD, 1);
        // c:RegisterEffect(e4)
        card::register_effect(d, c, e4, false);
    }
}
/// `s.ctcon(e,tp,eg,ep,ev,re,r,rp)`
fn ctcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not e:GetHandler():IsReason(REASON_LOST_TARGET)
    let c = effect::get_handler(d, a.e).nil();
    !card::is_reason(d, c, REASON_LOST_TARGET)
}
/// `s.cttg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local ec=e:GetHandler():GetPreviousEquipTarget()
    let c = effect::get_handler(d, a.e).nil();
    let ec = card::get_previous_equip_target(d, c).nil();
    // if ec:IsLocation(LOCATION_MZONE) and ec:IsControlerCanBeChanged() then
    if card::is_location(d, ec, LOCATION_MZONE) && card::is_controler_can_be_changed(d, ec, false, None) {
        // Duel.SetTargetCard(ec)
        duel::set_target_card(d, ec);
        // Duel.SetOperationInfo(0,CATEGORY_CONTROL,ec,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(CardSet::from_card(ec)), 1, 0, 0);
    }
    true
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // local ec=Duel.GetFirstTarget()
    // if ec and ec:IsRelateToEffect(e) then
    if let Some(ec) = duel::first_target(d) {
        if card::is_relate_to_effect(d, ec, a.e) {
            // Duel.GetControl(ec,tp)
            duel::get_control(d, ec, a.tp, 0, 0, None, None);
        }
    }
}
