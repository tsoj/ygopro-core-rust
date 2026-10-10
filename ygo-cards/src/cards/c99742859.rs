//! Rescue Hedgehog
//! レスキューヘッジホッグ
use super::prelude::*;

pub const ID: u32 = 99742859;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 2 Level 3 or lower monsters from your Deck with the same Type, Attribute, and Level
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e1, Some(cost::self_banish()));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelBelow(3) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level_below(d, c, 3) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetClassCount(Card.GetRace)==1
    //     and sg:GetClassCount(Card.GetAttribute)==1
    //     and sg:GetClassCount(Card.GetLevel)==1
    //     and sg:FilterCount(Card.IsType,nil,TYPE_NORMAL)==1
    //     and sg:FilterCount(Card.IsType,nil,TYPE_EFFECT)==1
    let res = group::get_class_count(d, sg, &|d, x| card::get_race(d, x) as i64) == 1
        && group::get_class_count(d, sg, &|d, x| card::get_attribute(d, x) as i64) == 1
        && group::get_class_count(d, sg, &|d, x| card::get_level(d, x) as i64) == 1
        && group::filter_count(d, sg, &|d, x| card::is_type(d, x, TYPE_NORMAL), NO_EX) == 1
        && group::filter_count(d, sg, &|d, x| card::is_type(d, x, TYPE_EFFECT), NO_EX) == 1;
    (res, false)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil,e,tp)
        let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, NO_EX);
        // return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) and Duel.GetMZoneCount(tp,e:GetHandler())>=2
        //     and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0)
        let h = effect::get_handler(d, e).nil();
        return duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_m_zone_count(d, tp, h.into(), None, None, None).0 >= 2
            && matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false), Err(true));
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,2,tp,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 2, tp, LOCATION_MZONE as i32);
    true
}
/// `function(ag) Duel.Destroy(ag,REASON_EFFECT) end`
fn destroy_oper(d: &mut Duel, ag: &CardSet, _a: &EffArgs) -> bool {
    duel::destroy(d, ag.clone(), REASON_EFFECT, None, None);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) or Duel.GetLocationCount(tp,LOCATION_MZONE)<2 then return end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some()
        || duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 2
    {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, NO_EX);
    // if #g<2 then return end
    if g.len() < 2 {
        return;
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_SPSUMMON)
    let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_SPSUMMON as u64), None, None, false).unwrap_or_default();
    // if #sg~=2 then return end
    if sg.len() != 2 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // for sc in sg:Iter() do
    for sc in group::iter(d, &sg) {
        // if Duel.SpecialSummonStep(sc,0,tp,tp,false,false,POS_FACEUP) then
        if duel::special_summon_step(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
            // sc:NegateEffects(c)
            card::negate_effects(d, sc, c, None, false, None);
        }
    }
    // Destroy them during the End Phase
    // aux.DelayedOperation(sg,PHASE_END,id,e,tp,function(ag) Duel.Destroy(ag,REASON_EFFECT) end,nil,0,1,aux.Stringid(id,1))
    aux::delayed_operation(
        d,
        sg.clone(),
        PHASE_END,
        ID,
        e,
        tp,
        Some(std::sync::Arc::new(destroy_oper)),
        None,
        Some(0),
        Some(1),
        Some(aux::stringid(ID, 1)),
        None,
    );
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
