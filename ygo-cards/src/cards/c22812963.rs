//! Regenesis Lord
use super::prelude::*;

pub const ID: u32 = 22812963;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_REGENESIS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // c:AddMustBeSpecialSummoned()
    card::add_must_be_special_summoned(d, c);
    // Must be Special Summoned (from your hand) by banishing 1 face-up monster you control with 2500 original ATK and DEF
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e0, aux::stringid(ID, 0));
    // e0:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e0, EFFECT_SPSUMMON_PROC);
    // e0:SetRange(LOCATION_HAND)
    effect::set_range(d, e0, LOCATION_HAND);
    // e0:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e0, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e0:SetCondition(s.selfspcon)
    effect::set_condition(d, e0, Some(lua_fn(selfspcon)));
    // e0:SetTarget(s.selfsptg)
    effect::set_target(d, e0, Some(lua_fn(selfsptg)));
    // e0:SetOperation(s.selfspop)
    effect::set_operation(d, e0, Some(lua_fn(selfspop)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Set 1 "Regenesis" Spell/Trap from your Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetTarget(s.settg)
    effect::set_target(d, e1, Some(cost_target(settg)));
    // e1:SetOperation(s.setop)
    effect::set_operation(d, e1, Some(operation(setop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Gains 2500 ATK during the Battle Phase only
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(function() return Duel.IsBattlePhase() end)
    effect::set_condition(d, e2, Some(condition(battle_phase_cond)));
    // e2:SetValue(2500)
    effect::set_value(d, e2, 2500);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Inflicts piercing battle damage
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e3, EFFECT_PIERCE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function() return Duel.IsBattlePhase() end`
fn battle_phase_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_battle_phase(d, None)
}
/// `s.selfspconfilter(c)`
fn selfspconfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsBaseAttack(2500) and c:IsBaseDefense(2500)) and c:IsFaceup() and c:IsAbleToRemoveAsCost()
    (card::is_base_attack(d, c, 2500) && card::is_base_defense(d, c, 2500)) && card::is_faceup(d, c) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.selfspcon(e,c)`
fn selfspcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local g=Duel.GetMatchingGroup(s.selfspconfilter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&selfspconfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // return #g>0 and Duel.GetMZoneCount(tp,g)>0
    !g.is_empty() && duel::get_m_zone_count(d, tp, (&g).into(), None, None, None).0 > 0
}
/// `s.selfsptg(e,tp,eg,ep,ev,re,r,rp,chk,c)`
fn selfsptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local rg=Duel.GetMatchingGroup(s.selfspconfilter,tp,LOCATION_MZONE,0,nil)
    let rg = duel::get_matching_group(d, Some(&selfspconfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // local g=aux.SelectUnselectGroup(rg,e,tp,1,1,aux.ChkfMMZ(1),1,tp,HINTMSG_REMOVE,nil,nil,true)
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(1), Some(1), Some(&aux::chkf_mmz(1)), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, true).unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.selfspop(e,tp,eg,ep,ev,re,r,rp,c)`
fn selfspop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if g then
    if let Value::Group(g) = effect::get_label_object(d, e) {
        // Duel.Remove(g,POS_FACEUP,REASON_COST)
        duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    }
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_REGENESIS) and c:IsSpellTrap() and c:IsSSetable()
    card::is_set_card(d, c, &[SET_REGENESIS as u16]) && card::is_spell_trap(d, c) && card::is_s_setable(d, c, false, None)
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    // local g=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&setfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SSet(tp,g)
        duel::s_set(d, tp, g, None, true);
    }
}
