//! Favorite Hero
use super::prelude::*;

pub const ID: u32 = 11881272;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_HERO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Equip only to lvl 5+ hero
    // aux.AddEquipProcedure(c,nil,s.filter)
    aux::add_equip_procedure(d, c, None, Some(aux::equip_filter_fn(|d, x, _e, _tp| filter(d, x))), None, None, None, None, None, None);
    // Provide ATK equal to the DEF
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetCondition(s.tgocon)
    effect::set_condition(d, e1, Some(condition(tgocon)));
    // e1:SetValue(s.atkval)
    effect::set_value_fn(d, e1, effect_card_value(atkval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Prevent effect target
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetCondition(s.tgocon)
    effect::set_condition(d, e2, Some(condition(tgocon)));
    // e2:SetValue(aux.tgoval)
    effect::set_value_fn(d, e2, aux::tgoval());
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Activate 1 Field Spell
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_PHASE|PHASE_BATTLE_START)
    effect::set_code(d, e3, EVENT_PHASE | PHASE_BATTLE_START);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetTarget(s.acttg)
    effect::set_target(d, e3, Some(cost_target(acttg)));
    // e3:SetOperation(s.actop)
    effect::set_operation(d, e3, Some(operation(actop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Second attack in a row
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e4, EVENT_BATTLE_DESTROYING);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCondition(s.atkcon)
    effect::set_condition(d, e4, Some(condition(atkcon)));
    // e4:SetCost(Cost.SelfToGrave)
    effect::set_cost(d, e4, Some(cost::self_to_grave()));
    // e4:SetOperation(s.atkop)
    effect::set_operation(d, e4, Some(operation(atkop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.filter(c)`: `c:IsLevelAbove(5) and c:IsSetCard(SET_HERO)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_level_above(d, c, 5) && card::is_set_card(d, c, &[SET_HERO as u16])
}
/// `s.tgocon(e)`
fn tgocon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldCard(e:GetHandlerPlayer(),LOCATION_FZONE,0)
    let p = effect::get_handler_player(d, a.e);
    duel::get_field_card(d, p, LOCATION_FZONE, 0).is_some()
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // local def=c:GetBaseDefense()
    let def = card::get_base_defense(d, c) as i64;
    // return def>=0 and def or 0
    if def >= 0 { def } else { 0 }
}
/// `s.actfilter(c,tp)`
fn actfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFieldSpell() and c:GetActivateEffect():IsActivatable(tp,true,true)
    if !card::is_field_spell(d, c) {
        return false;
    }
    let te = card::get_activate_effect(d, c);
    match te.first() {
        Some(&te) => effect::is_activatable(d, te, tp, true, true),
        None => false,
    }
}
/// `s.acttg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn acttg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.actfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| actfilter(d, c, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.actop(e,tp,eg,ep,ev,re,r,rp)`
fn actop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOFIELD as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.actfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,tp):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| actfilter(d, c, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // Duel.ActivateFieldSpell(sc,e,tp,eg,ep,ev,re,r,rp)
    duel::activate_field_spell(d, sc, a, None);
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local ec=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let ec = card::get_equip_target(d, h);
    // return ec==eg:GetFirst() and ec==Duel.GetAttacker()
    //     and ec:IsStatus(STATUS_OPPO_BATTLE) and ec:CanChainAttack()
    let eg = a.eg();
    if ec != group::get_first(d, &eg) {
        return false;
    }
    if ec != duel::get_attacker(d) {
        return false;
    }
    let ec = ec.nil();
    card::is_status(d, ec, STATUS_OPPO_BATTLE) && card::can_chain_attack(d, ec, None, false)
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, _a: &EffArgs) {
    // Duel.ChainAttack()
    duel::chain_attack(d, None);
}
