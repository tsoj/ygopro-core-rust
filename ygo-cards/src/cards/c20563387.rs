//! Number C80: Requiem in Berserk
use super::prelude::*;

pub const ID: u32 = 20563387;

pub const INFO: ScriptInfo = ScriptInfo {
    xyz_number: Some(80),
    listed_names: &[93568288],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    // Xyz.AddProcedure(c,nil,5,3)
    xyz::add_procedure(d, c, None, Some(5), 3, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
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
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // remove
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.rmcon)
    effect::set_condition(d, e2, Some(condition(rmcon)));
    // e2:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e2:SetTarget(s.rmtg)
    effect::set_target(d, e2, Some(cost_target(rmtg)));
    // e2:SetOperation(s.rmop)
    effect::set_operation(d, e2, Some(operation(rmop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_XYZ)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ)
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc) and chkc~=e:GetHandler() end
    if let Some(chkc) = a.chkc() {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc) && chkc != h;
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, 1, false, h.into());
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or c:IsFacedown() then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 or tc:GetControler()~=tp or tc:IsFacedown() or not tc:IsRelateToEffect(e) then
    if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 <= 0
        || card::get_controler(d, tc.nil()) != a.tp
        || card::is_facedown(d, tc.nil())
        || !card::is_relate_to_effect(d, tc.nil(), a.e)
    {
        // Duel.SendtoGrave(c,REASON_EFFECT)
        duel::send_to_grave(d, CardSet::from_card(c), REASON_EFFECT, None, None);
        return;
    }
    let tc = tc.nil();
    // Duel.Equip(tp,c,tc,true)
    duel::equip(d, a.tp, c, tc, true, false);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_EQUIP_LIMIT)
    effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // e1:SetValue(s.eqlimit)
    effect::set_value_fn(d, e1, lua_fn(eqlimit));
    // e1:SetLabelObject(tc)
    effect::set_label_object(d, e1, Value::Card(tc));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // atkup
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetValue(2000)
    effect::set_value(d, e2, 2000);
    // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // destroy sub
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    // e3:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e3, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e3:SetCode(EFFECT_DESTROY_SUBSTITUTE)
    effect::set_code(d, e3, EFFECT_DESTROY_SUBSTITUTE);
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.eqlimit(e,c)`
fn eqlimit(d: &mut Duel, a: Args) -> bool {
    // return c==e:GetLabelObject()
    effect::get_label_object(d, a.e(0)).as_card() == a.card(1)
}
/// `s.rmcon(e,tp,eg,ep,ev,re,r,rp)`
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetOverlayGroup():IsExists(Card.IsCode,1,nil,93568288)
    let h = effect::get_handler(d, a.e).nil();
    let og = card::get_overlay_group(d, h);
    group::is_exists(d, &og, &|d, x| card::is_code(d, x, &[93568288]), 1, NO_EX)
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsAbleToRemove() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_able_to_remove(d, chkc, None, None, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToRemove,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToRemove,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, 0, 0);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, CardSet::from_card(tc), POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
