//! Exosister Pax
use super::prelude::*;

pub const ID: u32 = 77913594;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_EXOSISTER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Search
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SEARCH+CATEGORY_TOHAND+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SEARCH + CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // e1:SetCondition(function()return Duel.IsMainPhase()end)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(Cost.PayLP(800))
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(800))));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `function()return Duel.IsMainPhase()end`
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_main_phase(d, None)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_EXOSISTER) and not c:IsCode(id) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_EXOSISTER as u16]) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.listfilter(c,sc)`
fn listfilter(d: &mut Duel, c: CardId, sc: CardId) -> bool {
    // return c:IsSetCard(SET_EXOSISTER) and c:IsFaceup() and c:IsMonster() and sc:ListsCode(c:GetCode())
    let codes = card::get_code(d, c);
    card::is_set_card(d, c, &[SET_EXOSISTER as u16]) && card::is_faceup(d, c) && card::is_monster(d, c) && card::lists_code(d, sc, &codes)
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
    let sg = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sg);
    // if not sc or Duel.SendtoHand(sc,nil,REASON_EFFECT)<1 or not sc:IsLocation(LOCATION_HAND) then return end
    let Some(sc) = sc else { return };
    if duel::send_to_hand(d, CardSet::from_card(sc), None, REASON_EFFECT, None) < 1 || !card::is_location(d, sc, LOCATION_HAND) {
        return;
    }
    // Duel.ConfirmCards(1-tp,sc)
    duel::confirm_cards(d, 1 - a.tp, CardSet::from_card(sc));
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, a.tp);
    // if sc:IsMonster() and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and sc:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.IsExistingMatchingCard(s.listfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil,sc)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if card::is_monster(d, sc)
        && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && card::is_can_be_special_summoned(d, sc, a.e, 0, a.tp, false, false, None, None, None)
        && duel::is_existing_matching_card(d, Some(&|d, c| listfilter(d, c, sc)), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0))
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, CardSet::from_card(sc), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
