//! Out of the Blue
use super::prelude::*;

pub const ID: u32 = 79718768;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Self destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e2:SetCode(EVENT_BECOME_TARGET)
    effect::set_code(d, e2, EVENT_BECOME_TARGET);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetOperation(s.desop1)
    effect::set_operation(d, e2, Some(operation(desop1)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e3:SetCode(EVENT_CHAIN_SOLVED)
    effect::set_code(d, e3, EVENT_CHAIN_SOLVED);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetOperation(s.desop2)
    effect::set_operation(d, e3, Some(operation(desop2)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // e3:SetLabelObject(e2)
    effect::set_label_object(d, e3, Value::Effect(e2));
    // Return cards in the GY to the Deck
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e4, CATEGORY_TODECK);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e4:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e4, EVENT_TO_GRAVE);
    // e4:SetCondition(s.retcon)
    effect::set_condition(d, e4, Some(condition(retcon)));
    // e4:SetTarget(s.rettg)
    effect::set_target(d, e4, Some(cost_target(rettg)));
    // e4:SetOperation(s.retop)
    effect::set_operation(d, e4, Some(operation(retop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.desop1(e,tp,eg,ep,ev,re,r,rp)`
fn desop1(d: &mut Duel, a: &EffArgs) {
    // if rp==tp or not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET)
    //     or not eg:IsExists(Card.IsLocation,1,nil,LOCATION_GRAVE) then
    let skip = a.rp == a.tp
        || !effect::is_has_property(d, a.re.nil(), EFFECT_FLAG_CARD_TARGET, 0)
        || !group::is_exists(d, &a.eg.clone().nil(), &|d, c| card::is_location(d, c, LOCATION_GRAVE), 1, NO_EX);
    if skip {
        // e:SetLabelObject(nil)
        effect::set_label_object(d, a.e, Value::Nil);
    } else {
        // else e:SetLabelObject(re) end
        effect::set_label_object(d, a.e, Value::Effect(a.re.nil()));
    }
}
/// `s.desop2(e,tp,eg,ep,ev,re,r,rp)`
fn desop2(d: &mut Duel, a: &EffArgs) {
    // local pe=e:GetLabelObject():GetLabelObject()
    let e2 = effect::get_label_object(d, a.e).as_effect().nil();
    let pe = effect::get_label_object(d, e2).as_effect();
    // if pe and pe==re then
    if pe.is_some() && pe == a.re {
        // Duel.Destroy(e:GetHandler(),REASON_EFFECT)
        let h = effect::get_handler(d, a.e).nil();
        duel::destroy(d, h, REASON_EFFECT, None, None);
    }
}
/// `s.retcon(e,tp,eg,ep,ev,re,r,rp)`
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return re and re:GetHandler()==e:GetHandler()
    match a.re {
        None => false,
        Some(re) => effect::get_handler(d, re) == effect::get_handler(d, a.e),
    }
}
/// `s.rettg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rettg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_GRAVE)
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_GRAVE);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,1-tp,LOCATION_GRAVE)
    let n = g.len();
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n as u8, 1 - a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.retop(e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_GRAVE)
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_GRAVE);
    // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT,1-tp)
    duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, Some(1 - a.tp));
}
