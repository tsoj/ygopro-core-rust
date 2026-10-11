//! Armored Xyz
use super::prelude::*;

pub const ID: u32 = 73046708;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Equip 1 Xyz monster to another monster on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
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
}
/// `s.eqfilter(c)`
fn eqfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_XYZ) and not c:IsForbidden()
    card::is_type(d, c, TYPE_XYZ) && !card::is_forbidden(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let h = effect::get_handler(d, a.e).nil();
    let mut ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    // if e:GetHandler():IsLocation(LOCATION_HAND) then ft=ft-1 end
    if card::is_location(d, h, LOCATION_HAND) {
        ft -= 1;
    }
    // if chk==0 then return ft>0 and Duel.IsExistingTarget(Card.IsFaceup,tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingTarget(s.eqfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return ft > 0
            && duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&eqfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local tc=Duel.SelectTarget(tp,Card.IsFaceup,tp,LOCATION_MZONE,0,1,1,nil)
    let _tc = duel::select_target(d, tp, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    // local eqc=Duel.SelectTarget(tp,s.eqfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let eqc = duel::select_target(d, tp, Some(&eqfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,eqc,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, eqc, 1, tp, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, e).nil();
    // if #g~=2 then return end
    if g.len() != 2 {
        return;
    }
    // local tc=g:Filter(Card.IsLocation,nil,LOCATION_MZONE):GetFirst()
    let mzone = group::filter(d, &g, &|d, c| card::is_location(d, c, LOCATION_MZONE), NO_EX);
    let tc = group::get_first(d, &mzone);
    // local eqc=g:Filter(Card.IsLocation,nil,LOCATION_GRAVE):GetFirst()
    let grave = group::filter(d, &g, &|d, c| card::is_location(d, c, LOCATION_GRAVE), NO_EX);
    let eqc = group::get_first(d, &grave);
    // if tc and eqc then
    if let (Some(tc), Some(eqc)) = (tc, eqc) {
        // if not tc:EquipByEffectAndLimitRegister(e,tp,eqc,nil,true) then return end
        if !card::equip_by_effect_and_limit_register(d, tc, e, tp, eqc, None, Some(true)) {
            return;
        }
        // local atk=eqc:GetBaseAttack()
        let atk = card::get_base_attack(d, eqc);
        // local att=eqc:GetAttribute()
        let att = card::get_attribute(d, eqc);
        // Equip limit
        // local e0=Effect.CreateEffect(eqc)
        let e0 = effect::create_effect(d, eqc);
        // e0:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
        // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e0:SetCode(EFFECT_EQUIP_LIMIT)
        effect::set_code(d, e0, EFFECT_EQUIP_LIMIT);
        // e0:SetValue(function(e,c) return c==tc end)
        effect::set_value_fn(d, e0, lua_fn(move |_d: &mut Duel, a: Args| a.c(1) == tc));
        // e0:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e0, RESET_EVENT | RESETS_STANDARD, 1);
        // eqc:RegisterEffect(e0)
        card::register_effect(d, eqc, e0, false);
        // ATK becomes the equipped monster's
        // local e1=Effect.CreateEffect(eqc)
        let e1 = effect::create_effect(d, eqc);
        // e1:SetType(EFFECT_TYPE_EQUIP)
        effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
        // e1:SetCode(EFFECT_SET_ATTACK)
        effect::set_code(d, e1, EFFECT_SET_ATTACK);
        // e1:SetValue(atk)
        effect::set_value(d, e1, atk as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // eqc:RegisterEffect(e1)
        card::register_effect(d, eqc, e1, false);
        // Gains the attribute of the equipped monster
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_ADD_ATTRIBUTE)
        effect::set_code(d, e2, EFFECT_ADD_ATTRIBUTE);
        // e2:SetValue(att)
        effect::set_value(d, e2, att as i64);
        // eqc:RegisterEffect(e2)
        card::register_effect(d, eqc, e2, false);
        // Second attack in a row
        // local e3=Effect.CreateEffect(eqc)
        let e3 = effect::create_effect(d, eqc);
        // e3:SetDescription(aux.Stringid(id,1))
        effect::set_description(d, e3, aux::stringid(ID, 1));
        // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
        effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
        // e3:SetCode(EVENT_DAMAGE_STEP_END)
        effect::set_code(d, e3, EVENT_DAMAGE_STEP_END);
        // e3:SetRange(LOCATION_SZONE)
        effect::set_range(d, e3, LOCATION_SZONE);
        // e3:SetCondition(s.atkcon)
        effect::set_condition(d, e3, Some(condition(atkcon)));
        // e3:SetCost(Cost.SelfToGrave)
        effect::set_cost(d, e3, Some(cost::self_to_grave()));
        // e3:SetOperation(function() Duel.ChainAttack() end)
        effect::set_operation(d, e3, Some(operation(chain_attack_op)));
        // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
        // eqc:RegisterEffect(e3)
        card::register_effect(d, eqc, e3, false);
    }
}
/// `function() Duel.ChainAttack() end`
fn chain_attack_op(d: &mut Duel, _a: &EffArgs) {
    duel::chain_attack(d, None);
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local ec=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let ec = card::get_equip_target(d, h);
    // return ec==Duel.GetAttacker() and ec:CanChainAttack()
    if ec != duel::get_attacker(d) {
        return false;
    }
    card::can_chain_attack(d, ec.nil(), None, false)
}
