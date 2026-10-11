//! War Rock Dignity
//! Scripted by Rundas
use super::prelude::*;

pub const ID: u32 = 47882774;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_WAR_ROCK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(function(e,tp,eg,ep,ev) Duel.NegateEffect(ev) end)
    effect::set_operation(d, e1, Some(operation(|d, a| {
        duel::negate_effect(d, a.ev as u8);
    })));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // if not (Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_WAR_ROCK),tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsChainDisablable(ev)) then return false end
    if !(duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_WAR_ROCK as u16])), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::is_chain_disablable(d, a.ev as u8))
    {
        return false;
    }
    // local trig_loc,trig_contr=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION,CHAININFO_TRIGGERING_CONTROLER)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION, CHAININFO_TRIGGERING_CONTROLER]);
    let trig_loc = info.first().filter(|v| !v.is_nil()).map(|v| v.int());
    let trig_contr = info.get(1).filter(|v| !v.is_nil()).map(|v| v.int());
    let re = a.re.nil();
    // return (re:IsMonsterEffect() and trig_loc==LOCATION_MZONE and trig_contr==1-tp)
    //     or (Duel.IsBattlePhase() and rp==1-tp and (re:IsMonsterEffect() or re:IsHasType(EFFECT_TYPE_ACTIVATE)))
    if effect::is_monster_effect(d, re) && trig_loc == Some(LOCATION_MZONE as i64) && trig_contr == Some((1 - a.tp) as i64) {
        return true;
    }
    duel::is_battle_phase(d, None) && a.rp == 1 - a.tp && (effect::is_monster_effect(d, re) || effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE))
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, a.tp, 0);
    true
}
