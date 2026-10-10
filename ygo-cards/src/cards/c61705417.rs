//! Graverobber
use super::prelude::*;

pub const ID: u32 = 61705417;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpell() and c:IsAbleToHand()
    card::is_spell(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,...,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_GRAVE) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_location(d, chkc, LOCATION_GRAVE) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,0,LOCATION_GRAVE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, 0, LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,0,LOCATION_GRAVE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc:IsRelateToEffect(e) then
    let tc = tc.nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoHand(tc,tp,REASON_EFFECT)
        duel::send_to_hand(d, tc, Some(a.tp), REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,tc)
        duel::confirm_cards(d, 1 - a.tp, tc);
        // tc:RegisterFlagEffect(id,RESET_EVENT|RESET_TOGRAVE|RESET_REMOVE|RESET_TEMP_REMOVE|RESET_TODECK|RESET_PHASE|PHASE_END,0,1)
        card::register_flag_effect(d, tc, ID, RESET_EVENT | RESET_TOGRAVE | RESET_REMOVE | RESET_TEMP_REMOVE | RESET_TODECK | RESET_PHASE | PHASE_END, 0, 1, 0, 0);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
        effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        // e1:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetCondition(s.tgcon)
        effect::set_condition(d, e1, Some(condition(tgcon)));
        // e1:SetOperation(s.tgop)
        effect::set_operation(d, e1, Some(operation(tgop)));
        // e1:SetLabelObject(tc)
        effect::set_label_object(d, e1, Value::Card(tc));
        // e1:SetReset(RESET_EVENT|RESET_TOGRAVE|RESET_REMOVE|RESET_TEMP_REMOVE|RESET_TODECK|RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_EVENT | RESET_TOGRAVE | RESET_REMOVE | RESET_TEMP_REMOVE | RESET_TODECK | RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
        // local e3=Effect.CreateEffect(e:GetHandler())
        let e3 = effect::create_effect(d, h);
        // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e3:SetCode(EVENT_CHAINING)
        effect::set_code(d, e3, EVENT_CHAINING);
        // e3:SetCondition(s.actcon)
        effect::set_condition(d, e3, Some(condition(actcon)));
        // e3:SetOperation(s.actop)
        effect::set_operation(d, e3, Some(operation(actop)));
        // e3:SetLabelObject(tc)
        effect::set_label_object(d, e3, Value::Card(tc));
        // e3:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e3,tp)
        duel::register_effect(d, e3, a.tp);
        // local e4=Effect.CreateEffect(e:GetHandler())
        let e4 = effect::create_effect(d, h);
        // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e4:SetCode(EVENT_CHAIN_SOLVED)
        effect::set_code(d, e4, EVENT_CHAIN_SOLVED);
        // e4:SetCondition(s.damcon)
        effect::set_condition(d, e4, Some(condition(damcon)));
        // e4:SetOperation(s.damop)
        effect::set_operation(d, e4, Some(operation(damop)));
        // e4:SetLabelObject(tc)
        effect::set_label_object(d, e4, Value::Card(tc));
        // e4:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e4, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e4,tp)
        duel::register_effect(d, e4, a.tp);
    }
}
/// `s.tgcon(e,tp,...)`
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // return tc:GetControler()~=tc:GetOwner() and tc:GetFlagEffect(id)~=0
    card::get_controler(d, tc) != card::get_owner(d, tc) && card::get_flag_effect(d, tc, ID) != 0
}
/// `s.tgop(e,tp,...)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // Duel.SendtoGrave(tc,REASON_EFFECT)
    duel::send_to_grave(d, tc, REASON_EFFECT, None, None);
}
/// `s.actcon(e,tp,...)`
fn actcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==tp and re:GetHandler()==e:GetLabelObject() and re:GetHandler():GetFlagEffect(id)~=0
    if a.rp != a.tp {
        return false;
    }
    let re = a.re.nil();
    let label_object = effect::get_label_object(d, a.e).as_card();
    if effect::get_handler(d, re) != label_object {
        return false;
    }
    let rh = effect::get_handler(d, re).nil();
    card::get_flag_effect(d, rh, ID) != 0
}
/// `s.actop(e,tp,...)`
fn actop(d: &mut Duel, a: &EffArgs) {
    // re:GetHandler():RegisterFlagEffect(id+1,RESET_CHAIN,0,1)
    let rh = effect::get_handler(d, a.re.nil()).nil();
    card::register_flag_effect(d, rh, ID + 1, RESET_CHAIN, 0, 1, 0, 0);
    // e:Reset()
    effect::reset(d, a.e);
}
/// `s.damcon(e,tp,...)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==tp and re:GetHandler()==e:GetLabelObject() and re:GetHandler():GetFlagEffect(id+1)~=0
    if a.rp != a.tp {
        return false;
    }
    let re = a.re.nil();
    let label_object = effect::get_label_object(d, a.e).as_card();
    if effect::get_handler(d, re) != label_object {
        return false;
    }
    let rh = effect::get_handler(d, re).nil();
    card::get_flag_effect(d, rh, ID + 1) != 0
}
/// `s.damop(e,tp,...)`
fn damop(d: &mut Duel, a: &EffArgs) {
    // re:GetHandler():ResetFlagEffect(id+1)
    let rh = effect::get_handler(d, a.re.nil()).nil();
    card::reset_flag_effect(d, rh, ID + 1);
    // Duel.Damage(tp,2000,REASON_EFFECT)
    duel::damage(d, a.tp, 2000, REASON_EFFECT, false, None);
    // e:Reset()
    effect::reset(d, a.e);
}
