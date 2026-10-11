//! Train Connection
use super::prelude::*;

pub const ID: u32 = 60879050;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,s.filter,s.eqlimit,s.cost)
    aux::add_equip_procedure(d, c, None, Some(aux::equip_filter_fn(|d, c, _e, _tp| filter(d, c))), Some(effect_card_fn(eqlimit)), Some(cost_target(cost_fn)), None, None, None, None);
    // Atk Change
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetCode(EFFECT_SET_ATTACK)
    effect::set_code(d, e1, EFFECT_SET_ATTACK);
    // e1:SetValue(s.value)
    effect::set_value_fn(d, e1, effect_card_value(value));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Pierce
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e2, EFFECT_PIERCE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // cannot attack
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e3, EFFECT_CANNOT_ATTACK);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e3, LOCATION_MZONE, 0);
    // e3:SetTarget(s.ftarget)
    effect::set_target(d, e3, Some(effect_card_fn(ftarget)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.eqlimit(e,c)
fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return e:GetHandler():GetEquipTarget()==c and c:IsRace(RACE_MACHINE) and c:IsAttribute(ATTRIBUTE_EARTH)
    let h = effect::get_handler(d, e).nil();
    card::get_equip_target(d, h) == Some(c) && card::is_race(d, c, RACE_MACHINE) && card::is_attribute(d, c, ATTRIBUTE_EARTH)
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_MACHINE) and c:IsAttribute(ATTRIBUTE_EARTH)
    card::is_faceup(d, c) && card::is_race(d, c, RACE_MACHINE) && card::is_attribute(d, c, ATTRIBUTE_EARTH)
}
// function s.rmfilter(c)
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_MACHINE) and c:IsLevelAbove(10) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    card::is_race(d, c, RACE_MACHINE) && card::is_level_above(d, c, 10) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
}
// function s.rescon(sg,e,tp,mg)
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return Duel.IsExistingTarget(s.filter,0,LOCATION_MZONE,LOCATION_MZONE,1,sg)
    (duel::is_existing_target(d, Some(&filter), 0, LOCATION_MZONE, LOCATION_MZONE, 1, sg.into()), false)
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetMatchingGroup(s.rmfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&rmfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // if chk==0 then return aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false), Err(true));
    }
    // local rg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_REMOVE)
    let rg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, false).unwrap_or_default();
    // Duel.Remove(rg,POS_FACEUP,REASON_COST)
    duel::remove(d, rg, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
// function s.value(e,c)
fn value(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetBaseAttack()*2
    card::get_base_attack(d, c) as i64 * 2
}
// function s.ftarget(e,c)
fn ftarget(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return e:GetHandler():GetEquipTarget()~=c
    let h = effect::get_handler(d, e).nil();
    card::get_equip_target(d, h) != Some(c)
}
