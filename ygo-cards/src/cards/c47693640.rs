//! Zombie Tiger
use super::prelude::*;

pub const ID: u32 = 47693640;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[10209545], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddUnionProcedure(c,aux.FilterBoolFunction(Card.IsCode,10209545),true)
    aux::add_union_procedure(d, c, Some(std::sync::Arc::new(|d: &mut Duel, cc: CardId| card::is_code(d, cc, &[10209545]))), true, None);
    // Atk up
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    // e3:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    // e3:SetValue(500)
    effect::set_value(d, e3, 500);
    // e3:SetCondition(aux.IsUnionState)
    effect::set_condition(d, e3, Some(aux::is_union_state()));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Def up
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e4, EFFECT_TYPE_EQUIP);
    // e4:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e4, EFFECT_UPDATE_DEFENSE);
    // e4:SetValue(500)
    effect::set_value(d, e4, 500);
    // e4:SetCondition(aux.IsUnionState)
    effect::set_condition(d, e4, Some(aux::is_union_state()));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // handes
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e5, aux::stringid(ID, 2));
    // e5:SetCategory(CATEGORY_HANDES)
    effect::set_category(d, e5, CATEGORY_HANDES);
    // e5:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e5:SetRange(LOCATION_SZONE)
    effect::set_range(d, e5, LOCATION_SZONE);
    // e5:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e5, EVENT_BATTLE_DESTROYING);
    // e5:SetCondition(s.hdcon)
    effect::set_condition(d, e5, Some(condition(hdcon)));
    // e5:SetTarget(s.hdtg)
    effect::set_target(d, e5, Some(cost_target(hdtg)));
    // e5:SetOperation(s.hdop)
    effect::set_operation(d, e5, Some(operation(hdop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.hdcon(e,tp,eg,ep,ev,re,r,rp)`
fn hdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return aux.IsUnionState(e) and eg:GetFirst()==e:GetHandler():GetEquipTarget()
    if !aux::is_union_state().call(d, &[Value::Effect(a.e)]).truthy() {
        return false;
    }
    let first = group::get_first(d, &a.eg.clone().nil());
    let h = effect::get_handler(d, a.e).nil();
    first == card::get_equip_target(d, h)
}
/// `s.hdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn hdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,0,0,1-tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, 1 - a.tp, 1);
    true
}
/// `s.hdop(e,tp,eg,ep,ev,re,r,rp)`
fn hdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_HAND)
    let g = duel::get_field_group(d, tp, 0, LOCATION_HAND);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local sg=g:RandomSelect(1-tp,1)
    let sg = group::random_select(d, &g, 1 - tp, 1);
    // Duel.SendtoGrave(sg,REASON_DISCARD|REASON_EFFECT)
    duel::send_to_grave(d, sg, REASON_DISCARD | REASON_EFFECT, None, None);
}
