//! Super B.E.S. Metal Slave
use super::prelude::*;

pub const ID: u32 = 41516133;
/// `local COUNTER_BES=0x1f`
const COUNTER_BES: u32 = 0x1f;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BES], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(COUNTER_BES)
    card::enable_counter_permit(d, c, COUNTER_BES as u16, None, None);
    // Special Summon this card from your hand, then place counters on it equal to the number of cards sent to the GY to activate this effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_COUNTER)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_COUNTER);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.spcost)
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy 2 face-up cards on the field, including a "B.E.S." monster you control
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,0,EFFECT_COUNT_CODE_CHAIN)
    effect::set_count_limit(d, e2, 1, 0, 0, EFFECT_COUNT_CODE_CHAIN as u8);
    // e2:SetCost(Cost.RemoveCounterFromSelf(COUNTER_BES,1))
    effect::set_cost(d, e2, Some(cost::remove_counter_from_self(COUNTER_BES as u16, 1)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE as u32 | TIMING_MAIN_END as u32 | TIMINGS_CHECK_MONSTER_E as u32));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.spcostfilter(c)
fn spcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelBelow(10) and c:IsSetCard(SET_BES) and c:IsAbleToGraveAsCost()
    card::is_level_below(d, c, 10) && card::is_set_card(d, c, &[SET_BES as u16]) && card::is_able_to_grave_as_cost(d, c)
}
// function s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spcostfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,c) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&spcostfilter), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, c.into());
    }
    // local g=Duel.GetMatchingGroup(s.spcostfilter,tp,LOCATION_HAND|LOCATION_DECK,0,c)
    let g = duel::get_matching_group(d, Some(&spcostfilter), tp, LOCATION_HAND | LOCATION_DECK, 0, c.into());
    // local sg=aux.SelectUnselectGroup(g,e,tp,1,5,aux.dncheck,1,tp,HINTMSG_TOGRAVE)
    let sg = aux::select_unselect_group(
        d,
        &g,
        Some(e),
        tp,
        Some(1),
        Some(5),
        Some(&aux::dncheck()),
        1,
        tp,
        Some(HINTMSG_TOGRAVE as u64),
        None,
        None,
        false,
    )
    .unwrap_or_default();
    // e:SetLabel(Duel.SendtoGrave(sg,REASON_COST))
    let sent = duel::send_to_grave(d, &sg, REASON_COST, None, None);
    effect::set_label(d, e, &[sent as i64]);
    true
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and c:IsCanAddCounter(COUNTER_BES,1,false,LOCATION_MZONE) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
            && card::is_can_add_counter(d, c, COUNTER_BES as u16, 1, false, LOCATION_MZONE);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_COUNTER,nil,e:GetLabel(),tp,COUNTER_BES)
    let n = effect::get_label(d, e) as u8;
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, None, n, tp, COUNTER_BES as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local count=e:GetLabel()
    let count = effect::get_label(d, e) as u16;
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0
    //     and c:IsCanAddCounter(COUNTER_BES,count) then
    if card::is_relate_to_effect(d, c, e)
        && duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0
        && card::is_can_add_counter(d, c, COUNTER_BES as u16, count, false, 0)
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // c:AddCounter(COUNTER_BES,count)
        card::add_counter(d, c, COUNTER_BES as u16, count, false);
    }
}
// function s.besmonsterfilter(c,tp)
fn besmonsterfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_BES) and c:IsLocation(LOCATION_MZONE) and c:IsControler(tp)
    card::is_set_card(d, c, &[SET_BES as u16]) && card::is_location(d, c, LOCATION_MZONE) && card::get_controler(d, c) == tp
}
// function s.rescon(sg,e,tp,mg)
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:IsExists(s.besmonsterfilter,1,nil,tp)
    (group::is_exists(d, sg, &|d, c| besmonsterfilter(d, c, tp), 1, NO_EX), false)
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsCanBeEffectTarget,e),tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(
        d,
        Some(&aux::faceup_filter(move |d: &mut Duel, c: CardId| card::is_can_be_effect_target(d, c, Some(e)))),
        tp,
        LOCATION_ONFIELD,
        LOCATION_ONFIELD,
        NO_EX,
    );
    // if chk==0 then return #g>=2 and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return g.len() >= 2
            && matches!(
                aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false),
                Err(true)
            );
    }
    // local g=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_DESTROY)
    let g = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_DESTROY as u64), None, None, false)
        .unwrap_or_default();
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 2, tp, 0);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp,chk)
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.Destroy(tg,REASON_EFFECT)
        duel::destroy(d, &tg, REASON_EFFECT, None, None);
    }
}
