//! Cursed Armaments
use super::prelude::*;

pub const ID: u32 = 98867329;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c)
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // Atk up
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(s.atkval)
    effect::set_value_fn(d, e1, effect_value(atkval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // equip
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e2, CATEGORY_EQUIP);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCondition(s.eqcon)
    effect::set_condition(d, e2, Some(condition(eqcon)));
    // e2:SetTarget(s.eqtg)
    effect::set_target(d, e2, Some(cost_target(eqtg)));
    // e2:SetOperation(s.eqop)
    effect::set_operation(d, e2, Some(operation(eqop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.atkval(e,c)`: `return Duel.GetFieldGroupCount(e:GetHandlerPlayer(),LOCATION_MZONE,0)*-600`
fn atkval(d: &mut Duel, e: EffectId) -> i64 {
    let p = effect::get_handler_player(d, e);
    duel::get_field_group_count(d, p, LOCATION_MZONE, 0) as i64 * -600
}
/// `s.eqcon(e,tp,eg,ep,ev,re,r,rp)`: `return e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD)`
fn eqcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, c, LOCATION_ONFIELD)
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0 and Duel.IsExistingTarget(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // Duel.SelectTarget(tp,Card.IsFaceup,tp,0,LOCATION_MZONE,1,1,nil)
    duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    let c = effect::get_handler(d, a.e).nil();
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(c)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and tc and tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, c, a.e) && card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
            // Duel.Equip(tp,c,tc)
            duel::equip(d, a.tp, c, tc, true, false);
        }
    }
}
