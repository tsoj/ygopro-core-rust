//! Infernoid Flood
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 34446231;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_INFERNOID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 2+ monsters, including an "Infernoid" monster
    // Link.AddProcedure(c,nil,2,nil,s.matcheck)
    let matcheck: GroupCheck = Arc::new(matcheck);
    link::add_procedure(d, c, None, 2, None, Some(matcheck), None, None);
    // Negate an opponent's Special Summon and banish that monster(s)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DISABLE_SUMMON+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_DISABLE_SUMMON + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_SPSUMMON)
    effect::set_code(d, e1, EVENT_SPSUMMON);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e,tp,eg,ep) return ep==1-tp and Duel.GetCurrentChain()==0 end)
    effect::set_condition(d, e1, Some(condition(negsum_cond)));
    // e1:SetCost(s.negsumcost)
    effect::set_cost(d, e1, Some(cost_target(negsumcost)));
    // e1:SetTarget(s.negsumtg)
    effect::set_target(d, e1, Some(cost_target(negsumtg)));
    // e1:SetOperation(s.negsumop)
    effect::set_operation(d, e1, Some(operation(negsumop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Banish 1 card on the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_REMOVE)
    effect::set_code(d, e2, EVENT_REMOVE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.rmcon)
    effect::set_condition(d, e2, Some(condition(rmcon)));
    // e2:SetTarget(s.rmtg)
    effect::set_target(d, e2, Some(cost_target(rmtg)));
    // e2:SetOperation(s.rmop)
    effect::set_operation(d, e2, Some(operation(rmop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon 1 "Infernoid" monster from your Deck
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.matcheck(g,lnkc,sumtype,tp)`
fn matcheck(d: &mut Duel, g: &CardSet, lnkc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:IsExists(Card.IsSetCard,1,nil,SET_INFERNOID,lnkc,sumtype,tp)
    group::is_exists(d, g, &|d, c| card::is_set_card_ctx(d, c, &[SET_INFERNOID as u16], Some(lnkc), sumtype, Some(tp)), 1, NO_EX)
}
/// `function(e,tp,eg,ep) return ep==1-tp and Duel.GetCurrentChain()==0 end`
fn negsum_cond(d: &mut Duel, a: &EffArgs) -> bool {
    a.ep == 1 - a.tp && duel::get_current_chain(d, false) == 0
}
/// `s.negsumcost(e,tp,...,chk)`
fn negsumcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,nil,1,false,nil,nil) end
    if a.chk == Some(0) {
        return aux::check_release_group_cost(d, a.tp, None, 1, 1, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,nil,1,1,false,nil,nil)
    let g = aux::select_release_group_cost(d, a.tp, None, 1, 1, false, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.negsumtg(e,tp,...,chk)`
fn negsumtg(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    // if chk==0 then return eg:IsExists(Card.IsAbleToRemove,1,nil) end
    if a.chk == Some(0) {
        return group::is_exists(d, &eg, &|d, c| card::is_able_to_remove(d, c, None, None, None), 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE_SUMMON,eg,#eg,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE_SUMMON, Some(eg.clone()), eg.len() as u8, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,eg,#eg,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(eg.clone()), eg.len() as u8, a.tp, 0);
    true
}
/// `s.negsumop(e,tp,...)`
fn negsumop(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg.clone().nil();
    // Duel.NegateSummon(eg)
    duel::negate_summon(d, eg.clone());
    // Duel.Remove(eg,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, eg, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
/// `s.rmconfilter(c,tp)`
fn rmconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousLocation(LOCATION_GRAVE) and c:IsPreviousControler(tp)
    card::is_previous_location(d, c, LOCATION_GRAVE) && card::is_previous_controler(d, c, tp)
}
/// `s.rmcon(e,tp,eg,ep,ev,re,r,rp)`
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.rmconfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| rmconfilter(d, c, a.tp), 1, NO_EX)
}
/// `s.rmtg(e,tp,...,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,PLAYER_EITHER,LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 1, PLAYER_EITHER as u8, LOCATION_ONFIELD as i32);
    true
}
/// `s.rmop(e,tp,...)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToRemove,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return rp==1-tp and c:IsLinkSummoned() and c:IsPreviousControler(tp) and c:IsPreviousLocation(LOCATION_MZONE)
    a.rp == 1 - a.tp && card::is_link_summoned(d, c) && card::is_previous_controler(d, c, a.tp) && card::is_previous_location(d, c, LOCATION_MZONE)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_INFERNOID) and c:IsMonster() and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    card::is_set_card(d, c, &[SET_INFERNOID as u16]) && card::is_monster(d, c) && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
/// `s.sptg(e,tp,...,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.spop(e,tp,...)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then Duel.SpecialSummon(g,0,tp,tp,true,false,POS_FACEUP) end
    if !g.is_empty() {
        duel::special_summon(d, g, 0, tp, tp, true, false, POS_FACEUP as u8, None);
    }
}
