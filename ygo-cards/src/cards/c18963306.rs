//! Galaxy-Eyes Cipher Dragon
use super::prelude::*;

pub const ID: u32 = 18963306;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    // Xyz.AddProcedure(c,nil,8,2)
    xyz::add_procedure(d, c, None, Some(8), 2, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // control
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e1, CATEGORY_CONTROL);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsControlerCanBeChanged()
    card::is_faceup(d, c) && card::is_controler_can_be_changed(d, c, false, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&filter), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, g, 1, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
    effect::set_code(d, e1, EFFECT_CANNOT_DIRECT_ATTACK);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(s.atktg)
    effect::set_target(d, e1, Some(effect_card_fn(atktg)));
    // e1:SetLabel(e:GetHandler():GetFieldID())
    let field_id = card::get_field_id(d, c);
    effect::set_label(d, e1, &[field_id as i64]);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and Duel.GetControl(tc,tp,PHASE_END,1) then
    let Some(tc) = tc else { return };
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    duel::get_control(d, tc, tp, PHASE_END as u16, 1, None, None); // (a Lua number is always true)
    // Duel.NegateRelatedChain(tc,RESET_TURN_SET)
    duel::negate_related_chain(d, tc, RESET_TURN_SET);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_DISABLE)
    effect::set_code(d, e2, EFFECT_DISABLE);
    // e2:SetCondition(s.concon)
    effect::set_condition(d, e2, Some(lua_fn(concon)));
    // e2:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
    // tc:RegisterEffect(e2)
    card::register_effect(d, tc, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_DISABLE_EFFECT)
    effect::set_code(d, e3, EFFECT_DISABLE_EFFECT);
    // e3:SetCondition(s.concon)
    effect::set_condition(d, e3, Some(lua_fn(concon)));
    // e3:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
    // tc:RegisterEffect(e3)
    card::register_effect(d, tc, e3, false);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_SET_ATTACK_FINAL)
    effect::set_code(d, e4, EFFECT_SET_ATTACK_FINAL);
    // e4:SetCondition(s.concon)
    effect::set_condition(d, e4, Some(lua_fn(concon)));
    // e4:SetValue(3000)
    effect::set_value(d, e4, 3000);
    // e4:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e4, RESETS_STANDARD_PHASE_END, 1);
    // tc:RegisterEffect(e4)
    card::register_effect(d, tc, e4, false);
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetCode(EFFECT_CHANGE_CODE)
    effect::set_code(d, e5, EFFECT_CHANGE_CODE);
    // e5:SetCondition(s.concon)
    effect::set_condition(d, e5, Some(lua_fn(concon)));
    // e5:SetValue(id)
    effect::set_value(d, e5, ID as i64);
    // e5:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e5, RESETS_STANDARD_PHASE_END, 1);
    // tc:RegisterEffect(e5)
    card::register_effect(d, tc, e5, false);
}
/// `s.concon(e)`
fn concon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // for _,eff in ipairs({e:GetHandler():GetCardEffect(EFFECT_SET_CONTROL)}) do
    let h = effect::get_handler(d, e).nil();
    for eff in card::get_card_effect(d, h, EFFECT_SET_CONTROL) {
        // if eff:GetOwner()==e:GetOwner() then return true end
        if effect::get_owner(d, eff) == effect::get_owner(d, e) {
            return true;
        }
    }
    // return false
    false
}
/// `s.atktg(e,c)`
fn atktg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return e:GetLabel()~=c:GetFieldID()
    effect::get_label(d, e) != card::get_field_id(d, c) as i64
}
