//! Original Bamboo Sword
use super::prelude::*;

pub const ID: u32 = 33578406;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BAMBOO_SWORD], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c)
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // Gains 0 ATK
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e0, EFFECT_TYPE_EQUIP);
    // e0:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e0, EFFECT_UPDATE_ATTACK);
    // e0:SetValue(0)
    effect::set_value(d, e0, 0);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Destroy all monsters your opponent controls
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e1, EVENT_BATTLE_DAMAGE);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCondition(s.descon)
    effect::set_condition(d, e1, Some(condition(descon)));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Equip 1 "Bamboo Sword" Equip Spell from your Deck to a face-up monster on the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e2, CATEGORY_EQUIP);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCost(s.eqcost)
    effect::set_cost(d, e2, Some(cost_target(eqcost)));
    // e2:SetTarget(s.eqtg)
    effect::set_target(d, e2, Some(cost_target(eqtg)));
    // e2:SetOperation(s.eqop)
    effect::set_operation(d, e2, Some(operation(eqop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetAttackTarget()==nil and Duel.GetAttacker()==e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    duel::get_attack_target(d).is_none() && duel::get_attacker(d) == card::get_equip_target(d, h)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_MZONE)
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_MZONE);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_MZONE)
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_MZONE);
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, &g, REASON_EFFECT, None, None);
}
/// `s.eqcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn eqcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:GetEquipTarget() and c:IsAbleToGraveAsCost() end
    if a.chk == Some(0) {
        return card::get_equip_target(d, c).is_some() && card::is_able_to_grave_as_cost(d, c);
    }
    // Duel.SendtoGrave(c,REASON_COST)
    duel::send_to_grave(d, c, REASON_COST, None, None);
    true
}
/// `s.eqfilter(c,tp)`
fn eqfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsEquipSpell() and c:IsSetCard(SET_BAMBOO_SWORD) and not c:IsCode(id) and c:CheckUniqueOnField(tp)
    //     and not c:IsForbidden()
    //     and Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil,c)
    card::is_equip_spell(d, c)
        && card::is_set_card(d, c, &[SET_BAMBOO_SWORD as u16])
        && !card::is_code(d, c, &[ID])
        && card::check_unique_on_field(d, c, tp, None, None)
        && !card::is_forbidden(d, c)
        && duel::is_existing_matching_card(d, Some(&|d, x| tgfilter(d, x, c)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
}
/// `s.tgfilter(c,ec)`
fn tgfilter(d: &mut Duel, c: CardId, ec: CardId) -> bool {
    // return c:IsFaceup() and ec:CheckEquipTarget(c)
    card::is_faceup(d, c) && card::check_equip_target(d, ec, c)
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.eqfilter,tp,LOCATION_DECK,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| eqfilter(d, c, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)==0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // local ec=Duel.SelectMatchingCard(tp,s.eqfilter,tp,LOCATION_DECK,0,1,1,nil,tp):GetFirst()
    let ec_group = duel::select_matching_card(d, a.tp, Some(&|d, c| eqfilter(d, c, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let ec = group::get_first(d, &ec_group);
    // if not ec then return end
    let Some(ec) = ec else { return };
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil,ec):GetFirst()
    let tc_group = duel::select_matching_card(d, a.tp, Some(&|d, c| tgfilter(d, c, ec)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &tc_group);
    // if tc then
    if let Some(tc) = tc {
        // Duel.Equip(tp,ec,tc)
        duel::equip(d, a.tp, ec, tc, true, false);
    }
}
