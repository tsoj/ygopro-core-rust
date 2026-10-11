//! Alien Shocktrooper M-Frame
//! エーリアン・ソルジャー M/フレーム
use super::prelude::*;

pub const ID: u32 = 74974229;
pub const INFO: ScriptInfo = ScriptInfo {
    counter_place_list: &[COUNTER_A],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_REPTILE),2,2)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_REPTILE, sc, st, Some(tp)))), 2, Some(2), None, None, None);
    // (Quick Effect): discard 1 monster, place A-Counters ...
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e1, CATEGORY_COUNTER);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_END_PHASE));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card is destroyed ... Special Summon non-Link Reptile monsters ...
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.costfilter(c)`
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsDiscardable() and c:GetOriginalLevel()>0
    card::is_monster(d, c) && card::is_discardable(d, c, None, None) && card::get_original_level(d, c) > 0
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&costfilter), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DISCARD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DISCARD as u64);
    // local g=Duel.SelectMatchingCard(tp,s.costfilter,tp,LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&costfilter), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST|REASON_DISCARD)
    duel::send_to_grave(d, g.clone(), REASON_COST | REASON_DISCARD, None, None);
    // e:GetChainData().discarded_level=g:GetFirst():GetOriginalLevel()
    let first = group::get_first(d, &g).nil();
    let lvl = card::get_original_level(d, first);
    chain::set_effect_data(d, a.e, None, chain::field::CARD, Value::Int(lvl as i64));
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local cd=e:GetChainData()
    let level = chain::effect_data(d, a.e, None, chain::field::CARD).int();
    // if #g==1 then g:GetFirst():AddCounter(COUNTER_A,cd.discarded_level)
    if g.len() == 1 {
        let first = group::get_first(d, &g).nil();
        card::add_counter(d, first, COUNTER_A as u16, level as u16, false);
    } else {
        // for i=1,cd.discarded_level do
        for _ in 0..level {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_COUNTER)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_COUNTER as u64);
            // local tc=g:Select(tp,1,1,nil):GetFirst()
            let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
            let tc = group::get_first(d, &sel).nil();
            // tc:AddCounter(COUNTER_A,1)
            card::add_counter(d, tc, COUNTER_A as u16, 1, false);
        }
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsReason(REASON_DESTROY) and c:IsReason(REASON_BATTLE|REASON_EFFECT)
    card::is_reason(d, c, REASON_DESTROY) && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
}
/// `s.acfilter(c)`
fn acfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetCounter(COUNTER_A)>0
    card::get_counter(d, c, COUNTER_A as u16) > 0
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_REPTILE) and not c:IsLinkMonster() and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_REPTILE)
        && !card::is_link_monster(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(s.acfilter,tp,0,LOCATION_MZONE,1,nil)
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&acfilter), tp, 0, LOCATION_MZONE, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft==0 then return end
    if ft == 0 {
        return;
    }
    // local ct=Duel.GetMatchingGroupCount(s.acfilter,tp,0,LOCATION_MZONE,nil)
    let acount = duel::get_matching_group_count(d, Some(&acfilter), tp, 0, LOCATION_MZONE, NO_EX);
    // if ct==0 then return end
    if acount == 0 {
        return;
    }
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE,0,nil,e,tp)
    let nv = aux::necro_valley_filter(move |d: &mut Duel, cc: CardId| spfilter(d, cc, e, tp));
    let g = duel::get_matching_group(d, Some(&|d: &mut Duel, c: CardId| nv(d, c)), tp, LOCATION_GRAVE, 0, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // ct=Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) and 1 or math.min(ft,ct)
    let ct = if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        1
    } else {
        std::cmp::min(ft, acount as i32)
    };
    // local sg=aux.SelectUnselectGroup(g,e,tp,1,ct,aux.dncheck,1,tp,HINTMSG_SPSUMMON)
    let dn = aux::dncheck();
    let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(1), Some(ct as usize), Some(&dn), 1, tp, Some(HINTMSG_SPSUMMON as u64), None, None, false).unwrap_or_default();
    // if #sg>0 then Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP) end
    if !sg.is_empty() {
        duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
