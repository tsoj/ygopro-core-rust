//! No.33: Chronomaly Machu Mech
use super::prelude::*;

pub const ID: u32 = 39139935;
pub const INFO: ScriptInfo = ScriptInfo {
    xyz_number: Some(33),
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    // Xyz.AddProcedure(c,nil,5,2)
    xyz::add_procedure(d, c, None, Some(5), 2, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // attack up
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DAMAGE+CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_DAMAGE + CATEGORY_ATKCHANGE);
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
    // return c:IsFaceup() and c:GetAttack()~=c:GetBaseAttack()
    card::is_faceup(d, c) && card::get_attack(d, c) != card::get_base_attack(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| filter(d, c)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g).nil();
    // local atk=tc:GetAttack()
    let atk = card::get_attack(d, tc) as i64;
    // local batk=tc:GetBaseAttack()
    let batk = card::get_base_attack(d, tc) as i64;
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,(batk>atk) and (batk-atk) or (atk-batk))
    let dif = if batk > atk { batk - atk } else { atk - batk };
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, dif as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // local atk=tc:GetAttack()
        let atk = card::get_attack(d, tc) as i64;
        // local batk=tc:GetBaseAttack()
        let batk = card::get_base_attack(d, tc) as i64;
        // if batk~=atk then
        if batk != atk {
            // local dif=(batk>atk) and (batk-atk) or (atk-batk)
            let dif = if batk > atk { batk - atk } else { atk - batk };
            // local dam=Duel.Damage(1-tp,dif,REASON_EFFECT)
            let dam = duel::damage(d, 1 - a.tp, dif, REASON_EFFECT, false, None);
            // if dam>0 and c:IsRelateToEffect(e) and c:IsFaceup() then
            if dam > 0 && card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetCode(EFFECT_UPDATE_ATTACK)
                effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
                // e1:SetValue(dam)
                effect::set_value(d, e1, dam as i64);
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
                // c:RegisterEffect(e1)
                card::register_effect(d, c, e1, false);
            }
        }
    }
}
