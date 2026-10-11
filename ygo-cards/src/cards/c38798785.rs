//! Echelon of the Fire Kings
use super::prelude::*;

pub const ID: u32 = 38798785;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FIRE_KING], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 3 FIRE monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_STANDBY_PHASE,TIMING_STANDBY_PHASE|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, TIMING_STANDBY_PHASE, Some(TIMING_STANDBY_PHASE | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Prevent opponent from responding to Normal/Special Summon of "Fire King" monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetHintTiming(TIMING_STANDBY_PHASE)
    effect::set_hint_timing(d, e2, TIMING_STANDBY_PHASE, None);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) if chk==0 then return not Duel.HasFlagEffect(tp,id) end end)
    effect::set_target(d, e2, Some(cost_target(flag_target)));
    // e2:SetOperation(s.chlimop)
    effect::set_operation(d, e2, Some(operation(chlimop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk) if chk==0 then return not Duel.HasFlagEffect(tp,id) end end`
fn flag_target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return !duel::has_flag_effect(d, a.tp, ID, None);
    }
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_FIRE) and c:IsRace(RACES_BEAST_BWARRIOR_WINGB) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_attribute(d, c, ATTRIBUTE_FIRE)
        && card::is_race(d, c, RACES_BEAST_BWARRIOR_WINGB as u64)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetClassCount(Card.GetLocation)==3 and sg:GetClassCount(Card.GetRace)==3
    let a = group::get_class_count(d, sg, &|d, c| card::get_location(d, c) as i64) == 3;
    let b = group::get_class_count(d, sg, &|d, c| card::get_race(d, c) as i64) == 3;
    (a && b, false)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK|LOCATION_HAND|LOCATION_GRAVE,0,nil,e,tp)
        let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK | LOCATION_HAND | LOCATION_GRAVE, 0, NO_EX);
        // return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
        //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>=3 and aux.SelectUnselectGroup(g,e,tp,3,3,s.rescon,0)
        return duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 >= 3
            && matches!(
                aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(&rescon), 0, tp, None, None, None, false),
                Err(true)
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,3,tp,LOCATION_DECK|LOCATION_HAND|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 3, tp, (LOCATION_DECK | LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) or Duel.GetLocationCount(tp,LOCATION_MZONE)<3 then return end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some()
        || duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 3
    {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK|LOCATION_HAND|LOCATION_GRAVE,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK | LOCATION_HAND | LOCATION_GRAVE, 0, NO_EX);
    // if #g<3 then return end
    if g.len() < 3 {
        return;
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,3,3,s.rescon,1,tp,HINTMSG_SPSUMMON)
    let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(&rescon), 1, tp, Some(HINTMSG_SPSUMMON as u64), None, None, false)
        .unwrap_or_default();
    // if #sg~=3 then return end
    if sg.len() != 3 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // for sc in sg:Iter() do
    for sc in group::iter(d, &sg) {
        // if Duel.SpecialSummonStep(sc,0,tp,tp,false,false,POS_FACEUP) then
        if duel::special_summon_step(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
            // Negate their effects
            // sc:NegateEffects(c)
            card::negate_effects(d, sc, c, None, false, None);
        }
    }
    // Destroy them during the End Phase
    // aux.DelayedOperation(sg,PHASE_END,id,e,tp,function(ag) Duel.Destroy(ag,REASON_EFFECT) end,nil,0,1,aux.Stringid(id,2))
    aux::delayed_operation(
        d,
        sg.clone(),
        PHASE_END,
        ID,
        e,
        tp,
        Some(std::sync::Arc::new(destroy_ag)),
        None,
        Some(0),
        Some(1),
        Some(aux::stringid(ID, 2)),
        None,
    );
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `function(ag) Duel.Destroy(ag,REASON_EFFECT) end`
fn destroy_ag(d: &mut Duel, ag: &CardSet, _a: &EffArgs) -> bool {
    duel::destroy(d, ag.clone(), REASON_EFFECT, None, None);
    true
}
/// `s.chlimop(e,tp,eg,ep,ev,re,r,rp)`
fn chlimop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.HasFlagEffect(tp,id) then return end
    if duel::has_flag_effect(d, tp, ID, None) {
        return;
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetOperation(s.limop)
    effect::set_operation(d, e1, Some(operation(limop)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetCode(EVENT_CHAIN_END)
    effect::set_code(d, e3, EVENT_CHAIN_END);
    // e3:SetOperation(s.limop2)
    effect::set_operation(d, e3, Some(operation(limop2)));
    // Duel.RegisterEffect(e3,tp)
    duel::register_effect(d, e3, tp);
    // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,3))
    aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 3)), 0, None);
}
/// `s.limfilter(c,tp)`
fn limfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_FIRE_KING) and c:IsFaceup() and c:IsControler(tp)
    card::is_set_card(d, c, &[SET_FIRE_KING as u16]) && card::is_faceup(d, c) && card::is_controler(d, c, tp)
}
/// `function(re,rp,tp) return rp==tp end`
fn same_player_limit(_d: &mut Duel, a: Args) -> bool {
    a.p(1) == a.p(2)
}
/// `s.limop(e,tp,eg,ep,ev,re,r,rp)`
fn limop(d: &mut Duel, a: &EffArgs) {
    // if eg:IsExists(s.limfilter,1,nil,tp) then
    let eg = a.eg.clone().nil();
    if group::is_exists(d, &eg, &|d, c| limfilter(d, c, a.tp), 1, NO_EX) {
        // Duel.SetChainLimitTillChainEnd(function(re,rp,tp) return rp==tp end)
        duel::set_chain_limit_till_chain_end(d, lua_fn(same_player_limit));
    }
}
/// `s.limop2(e,tp,eg,ep,ev,re,r,rp)`
fn limop2(d: &mut Duel, a: &EffArgs) {
    // local ex,sg=Duel.CheckEvent(EVENT_SPSUMMON_SUCCESS,true)
    // if ex and sg:IsExists(s.limfilter,1,nil,tp) then
    if let Some(evt) = duel::check_event(d, EVENT_SPSUMMON_SUCCESS) {
        let sg = evt.event_cards.clone().nil();
        if group::is_exists(d, &sg, &|d, c| limfilter(d, c, a.tp), 1, NO_EX) {
            // Duel.SetChainLimitTillChainEnd(function(re,rp,tp) return rp==tp end)
            duel::set_chain_limit_till_chain_end(d, lua_fn(same_player_limit));
        }
    }
}
