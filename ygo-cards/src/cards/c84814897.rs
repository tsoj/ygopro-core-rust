//! Kiryu
use super::prelude::*;

pub const ID: u32 = 84814897;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[11321183], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddUnionProcedure(c,aux.FilterBoolFunction(Card.IsCode,11321183),true)
    aux::add_union_procedure(d, c, Some(std::sync::Arc::new(|d: &mut Duel, cc: CardId| card::is_code(d, cc, &[11321183]))), true, None);
    // Atk up
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(900)
    effect::set_value(d, e1, 900);
    // e1:SetCondition(aux.IsUnionState)
    effect::set_condition(d, e1, Some(aux::is_union_state()));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Def up
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    // e2:SetValue(900)
    effect::set_value(d, e2, 900);
    // e2:SetCondition(aux.IsUnionState)
    effect::set_condition(d, e2, Some(aux::is_union_state()));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // direct_attack
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCondition(aux.IsUnionState)
    effect::set_condition(d, e3, Some(aux::is_union_state()));
    // e3:SetCost(s.atkcost)
    effect::set_cost(d, e3, Some(cost_target(atkcost)));
    // e3:SetOperation(s.atkop)
    effect::set_operation(d, e3, Some(operation(atkop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.atkcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsReleasable() end
    if a.chk == Some(0) {
        return card::is_releasable(d, h, None, None);
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, desc);
    // Duel.SetTargetCard(e:GetHandler():GetEquipTarget())
    let target = card::get_equip_target(d, h).nil();
    duel::set_target_card(d, target);
    // Duel.Release(e:GetHandler(),REASON_COST)
    duel::release(d, h, REASON_COST, None);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_DIRECT_ATTACK)
        effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
