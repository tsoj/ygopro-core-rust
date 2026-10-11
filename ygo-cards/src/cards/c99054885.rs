//! The Agent of Destruction - Venus
use super::prelude::*;

pub const ID: u32 = 99054885;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[64734921, 39552864], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon self
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.hspcost)
    effect::set_cost(d, e1, Some(cost_target(hspcost)));
    // e1:SetTarget(s.hsptg)
    effect::set_target(d, e1, Some(cost_target(hsptg)));
    // e1:SetOperation(s.hspop)
    effect::set_operation(d, e1, Some(operation(hspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(s.spcost)
    effect::set_cost(d, e2, Some(cost_target(spcost)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.hspcostfilter(c,tp)
fn hspcostfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsCode(64734921) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    //     and Duel.GetMZoneCount(tp,c)>0
    card::is_code(d, c, &[64734921])
        && card::is_able_to_remove_as_cost(d, c, None)
        && aux::sp_elim_filter(d, c, true, false)
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
// function s.hspcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn hspcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.hspcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| hspcostfilter(d, c, a.tp)), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.hspcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| hspcostfilter(d, c, a.tp)), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
// function s.hsptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn hsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
// function s.hspop(e,tp,eg,ep,ev,re,r,rp)
fn hspop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(39552864) and c:IsFaceup() and c:IsCanBeEffectTarget(e)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[39552864])
        && card::is_faceup(d, c)
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local sg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,nil,e,tp)
    let sg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, NO_EX);
    // local ct=math.min(#sg,Duel.GetLocationCount(tp,LOCATION_MZONE))
    let mut ct = std::cmp::min(sg.len() as i32, duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0);
    // if chk==0 then return ct>0 and Duel.CheckLPCost(tp,500) end
    if a.chk == Some(0) {
        return ct > 0 && duel::check_lp_cost(d, tp, 500);
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ct=1 end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ct = 1;
    }
    // local lp=Duel.GetLP(tp)
    let lp = duel::get_lp(d, tp);
    // local max=math.min(ct,lp//500)
    let max = std::cmp::min(ct, lp.div_euclid(500));
    // local t={}
    // for i=1,max do t[i]=i*500 end
    let t: Vec<u64> = (1..=max).map(|i| i as u64 * 500).collect();
    // local ac=Duel.AnnounceNumber(tp,table.unpack(t))
    let ac = duel::announce_number(d, tp, &t).nil().0;
    // Duel.PayLPCost(tp,ac)
    duel::pay_lp_cost(d, tp, ac as u32);
    // e:SetLabel(ac//500)
    effect::set_label(d, a.e, &[(ac / 500) as i64]);
    true
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e) as u16;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sg=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,ct,ct,nil,e,tp)
    let sg = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, ct, ct, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,sg,#sg,tp,0)
    let n = sg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(sg), n, tp, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // if #g==0 or Duel.GetLocationCount(tp,LOCATION_MZONE)<#g
    //     or (#g>1 and Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)) then return end
    if g.is_empty()
        || (duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 as i64) < g.len() as i64
        || (g.len() > 1 && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some())
    {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // for sc in g:Iter() do
    for sc in group::iter(d, &g) {
        // if Duel.SpecialSummonStep(sc,0,tp,tp,false,false,POS_FACEUP) then
        if duel::special_summon_step(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
            // Place it on the bottom of the Deck if it leaves the field
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetDescription(3301)
            effect::set_description(d, e1, 3301);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
            effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetReset(RESET_EVENT|RESETS_REDIRECT)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
            // e1:SetValue(LOCATION_DECKBOT)
            effect::set_value(d, e1, LOCATION_DECKBOT as i64);
            // sc:RegisterEffect(e1,true)
            card::register_effect(d, sc, e1, true);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
