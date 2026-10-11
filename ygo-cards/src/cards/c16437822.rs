//! Flint Missile
use super::prelude::*;

pub const ID: u32 = 16437822;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[75560629], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2a:SetCode(EVENT_LEAVE_FIELD_P)
    effect::set_code(d, e2a, EVENT_LEAVE_FIELD_P);
    // e2a:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2a, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2a:SetOperation(function(e) e:SetLabel(e:GetHandler():IsStatus(STATUS_LEAVE_CONFIRMED) and 1 or 0) end)
    effect::set_operation(d, e2a, Some(operation(leave_label)));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=Effect.CreateEffect(c)
    let e2b = effect::create_effect(d, c);
    // e2b:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2b, aux::stringid(ID, 0));
    // e2b:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e2b, CATEGORY_TODECK);
    // e2b:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2b, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2b:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2b, EVENT_TO_GRAVE);
    // e2b:SetCondition(function(e) return e:GetLabelObject():GetLabel()==1 end)
    effect::set_condition(d, e2b, Some(condition(label_is_one)));
    // e2b:SetTarget(s.rettg)
    effect::set_target(d, e2b, Some(cost_target(rettg)));
    // e2b:SetOperation(s.retop)
    effect::set_operation(d, e2b, Some(operation(retop)));
    // e2b:SetLabelObject(e2a)
    effect::set_label_object(d, e2b, Value::Effect(e2a));
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetEquipCount()>0 and c:GetEquipGroup():IsExists(Card.IsCode,1,nil,75560629)
    if card::get_equip_count(d, c) == 0 {
        return false;
    }
    let eq = card::get_equip_group(d, c);
    group::is_exists(d, &eq, &|d, x| card::is_code(d, x, &[75560629]), 1, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then Duel.Destroy(tc,REASON_EFFECT) end
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None);
    }
}
/// The label object of an effect as an effect (`e:GetLabelObject()` of the register effect).
fn label_effect(d: &mut Duel, e: EffectId) -> EffectId {
    match effect::get_label_object(d, e) {
        Value::Effect(x) => x,
        _ => crate::lua_error("attempt to index a nil value"),
    }
}
/// `function(e) e:SetLabel(e:GetHandler():IsStatus(STATUS_LEAVE_CONFIRMED) and 1 or 0) end`
fn leave_label(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    let v = if card::is_status(d, h, STATUS_LEAVE_CONFIRMED) { 1 } else { 0 };
    effect::set_label(d, a.e, &[v]);
}
/// `function(e) return e:GetLabelObject():GetLabel()==1 end`
fn label_is_one(d: &mut Duel, a: &EffArgs) -> bool {
    let le = label_effect(d, a.e);
    effect::get_label(d, le) == 1
}
/// `s.rettg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rettg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsAbleToDeck() end
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_deck(d, h);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.retop(e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():IsRelateToEffect(e) then
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) {
        // Duel.SendtoDeck(e:GetHandler(),nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, CardSet::from_card(h), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
