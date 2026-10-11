//! Storm-Bane Dragon Destorbim
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 94641726;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: 1 Tuner + 1+ non Tuners
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Your opponent cannot target this card with card effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(aux.tgoval)
    effect::set_value_fn(d, e1, aux::tgoval());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // You can banish any number of DARK monsters from your GY that cannot be Normal Summoned/Set; banish that many cards your opponent controls. This is a Quick Effect if your opponent controls more cards than you do
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2a, aux::stringid(ID, 0));
    // e2a:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2a, CATEGORY_REMOVE);
    // e2a:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2a, EFFECT_TYPE_IGNITION);
    // e2a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2a, LOCATION_MZONE);
    // e2a:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e2a, 1, ID, 0, 0);
    // e2a:SetCondition(function(e,tp) return Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)<=Duel.GetFieldGroupCount(tp,LOCATION_ONFIELD,0) end)
    effect::set_condition(d, e2a, Some(condition(e2a_cond)));
    // e2a:SetCost(s.bancost)
    effect::set_cost(d, e2a, Some(cost_target(bancost)));
    // e2a:SetTarget(s.bantg)
    effect::set_target(d, e2a, Some(cost_target(bantg)));
    // e2a:SetOperation(s.banop)
    effect::set_operation(d, e2a, Some(operation(banop)));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=e2a:Clone()
    let e2b = effect::clone(d, e2a);
    // e2b:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2b, EFFECT_TYPE_QUICK_O);
    // e2b:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2b, EVENT_FREE_CHAIN);
    // e2b:SetCondition(function(e,tp) return Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)>Duel.GetFieldGroupCount(tp,LOCATION_ONFIELD,0) end)
    effect::set_condition(d, e2b, Some(condition(e2b_cond)));
    // e2b:SetHintTiming(TIMING_SPSUMMON,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2b, TIMING_SPSUMMON, Some((TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E) as u32));
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
    // If this card is sent to the GY: You can Special Summon 1 of your banished Dragon monsters
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}

/// `function(e,tp) return Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)<=Duel.GetFieldGroupCount(tp,LOCATION_ONFIELD,0) end`
fn e2a_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_field_group_count(d, a.tp, 0, LOCATION_ONFIELD) <= duel::get_field_group_count(d, a.tp, LOCATION_ONFIELD, 0)
}

/// `function(e,tp) return Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)>Duel.GetFieldGroupCount(tp,LOCATION_ONFIELD,0) end`
fn e2b_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_field_group_count(d, a.tp, 0, LOCATION_ONFIELD) > duel::get_field_group_count(d, a.tp, LOCATION_ONFIELD, 0)
}

/// `Card.IsAbleToRemove` as a plain filter
fn able_remove(d: &mut Duel, c: CardId) -> bool {
    card::is_able_to_remove(d, c, None, None, None)
}

/// `s.bancostfilter(c)`
fn bancostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_DARK) and not c:IsSummonableCard() and c:IsAbleToRemoveAsCost()
    card::is_attribute(d, c, ATTRIBUTE_DARK) && !card::is_summonable_card(d, c) && card::is_able_to_remove_as_cost(d, c, None)
}

/// `s.bancost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn bancost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.bancostfilter,tp,LOCATION_GRAVE,0,1,nil)
    //   and Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&bancostfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&able_remove), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // local max_count=Duel.GetMatchingGroupCount(Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD,nil)
    let max_count = duel::get_matching_group_count(d, Some(&able_remove), tp, 0, LOCATION_ONFIELD, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.bancostfilter,tp,LOCATION_GRAVE,0,1,max_count,nil)
    let g = duel::select_matching_card(d, tp, Some(&bancostfilter), tp, LOCATION_GRAVE, 0, 1, max_count as u16, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    let n = g.len() as i64;
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    // e:SetLabel(#g)
    effect::set_label(d, a.e, &[n]);
    true
}

/// `s.bantg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn bantg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,e:GetLabel(),1-tp,LOCATION_ONFIELD)
    let n = effect::get_label(d, a.e);
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, n as u8, 1 - a.tp, LOCATION_ONFIELD as i32);
    true
}

/// `s.banop(e,tp,eg,ep,ev,re,r,rp)`
fn banop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local banish_count=e:GetLabel()
    let banish_count = effect::get_label(d, a.e);
    // if Duel.GetMatchingGroupCount(Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD,nil)<banish_count then return end
    if (duel::get_matching_group_count(d, Some(&able_remove), tp, 0, LOCATION_ONFIELD, NO_EX) as i64) < banish_count {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD,banish_count,banish_count,nil)
    let bc = banish_count as u16;
    let g = duel::select_matching_card(d, tp, Some(&able_remove), tp, 0, LOCATION_ONFIELD, bc, bc, false, NO_EX).nil();
    // if #g==banish_count then
    if g.len() as i64 == banish_count {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}

/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_DRAGON) and c:IsFaceup() and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_DRAGON) && card::is_faceup(d, c) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}

/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_REMOVED,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_REMOVED as i32);
    true
}

/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_REMOVED,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP) end
    if !g.is_empty() {
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
