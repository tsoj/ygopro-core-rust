//! Esper Girl
use super::prelude::*;

pub const ID: u32 = 99070951;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // remove
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(s.rmcon)
    effect::set_condition(d, e1, Some(condition(rmcon)));
    // e1:SetTarget(s.rmtg)
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    // e1:SetOperation(s.rmop)
    effect::set_operation(d, e1, Some(operation(rmop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // add to hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCondition(s.thcon)
    effect::set_condition(d, e2, Some(condition(thcon)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.rmcon(e,tp,eg,ep,ev,re,r,rp)`
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPreviousLocation(LOCATION_REMOVED)
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, h, LOCATION_REMOVED)
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetDecktopGroup(tp,1)
    let g = duel::get_decktop_group(d, tp, 1);
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // if not tc or not tc:IsAbleToRemove() then return end
    let Some(tc) = tc else { return };
    if !card::is_able_to_remove(d, tc, None, None, None) {
        return;
    }
    // Duel.DisableShuffleCheck()
    duel::disable_shuffle_check(d, true);
    // Duel.Remove(tc,POS_FACEDOWN,REASON_EFFECT)
    duel::remove(d, tc, POS_FACEDOWN as u8, REASON_EFFECT, None, None);
    // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1)
    card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
    // if c:IsLocation(LOCATION_MZONE) then
    if card::is_location(d, c, LOCATION_MZONE) {
        // c:RegisterFlagEffect(id,RESET_EVENT|RESET_TODECK|RESET_TOHAND|RESET_REMOVE,0,1)
        card::register_flag_effect(d, c, ID, RESET_EVENT | RESET_TODECK | RESET_TOHAND | RESET_REMOVE, 0, 1, 0, 0);
    }
    // e:SetLabelObject(tc)
    effect::set_label_object(d, a.e, Value::Card(tc));
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetLabelObject():GetLabelObject()
    let e1 = effect::get_label_object(d, a.e).as_effect().nil();
    let tc = effect::get_label_object(d, e1).as_card();
    // return e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD) and e:GetHandler():GetFlagEffect(id)~=0
    //     and tc and tc:GetFlagEffect(id)~=0
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, h, LOCATION_ONFIELD)
        && card::get_flag_effect(d, h, ID) != 0
        && tc.is_some_and(|tc| card::get_flag_effect(d, tc, ID) != 0)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local tc=e:GetLabelObject():GetLabelObject()
    let e1 = effect::get_label_object(d, a.e).as_effect().nil();
    let tc = effect::get_label_object(d, e1).as_card();
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, tc.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject():GetLabelObject()
    let e1 = effect::get_label_object(d, a.e).as_effect().nil();
    let tc = effect::get_label_object(d, e1).as_card();
    // if tc and tc:GetFlagEffect(id)~=0 then
    if let Some(tc) = tc {
        if card::get_flag_effect(d, tc, ID) != 0 {
            // Duel.SendtoHand(tc,nil,REASON_EFFECT)
            duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
        }
    }
}
