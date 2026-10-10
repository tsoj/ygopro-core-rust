//! Labrynth Set-Up
use super::prelude::*;

pub const ID: u32 = 69895264;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[69895264], listed_series: &[SET_LABRYNTH], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Send to Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.tdtg)
    effect::set_target(d, e1, Some(cost_target(tdtg)));
    // e1:SetOperation(s.tdop)
    effect::set_operation(d, e1, Some(operation(tdop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_LABRYNTH) and c:IsSpellTrap() and not c:IsCode(id) and c:IsAbleToDeck()
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_LABRYNTH as u16])
        && card::is_spell_trap(d, c)
        && !card::is_code(d, c, &[ID])
        && card::is_able_to_deck(d, c)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE|LOCATION_REMOVED) and s.tdfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE | LOCATION_REMOVED) && tdfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tdfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,2,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tdfilter), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 2, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.tdfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,2,2,nil)
    let g = duel::select_target(d, a.tp, Some(&tdfilter), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 2, 2, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, 0, 0);
    true
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return not c:IsSetCard(SET_LABRYNTH) and c:IsNormalTrap() and c:IsSSetable()
    !card::is_set_card(d, c, &[SET_LABRYNTH as u16]) && card::is_normal_trap(d, c) && card::is_s_setable(d, c, false, None)
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).unwrap_or_default();
    // if #tg<1 or Duel.SendtoDeck(tg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)<1 then return end
    if tg.is_empty() || duel::send_to_deck(d, tg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) < 1 {
        return;
    }
    // local ct=Duel.GetOperatedGroup():FilterCount(Card.IsLocation,nil,LOCATION_DECK)
    let operated = duel::get_operated_group(d);
    let ct = group::filter_count(d, &operated, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX) as i32;
    // if ct<1 or ct>Duel.GetLocationCount(tp,LOCATION_SZONE)
    //   or not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_FIEND),tp,LOCATION_MZONE,0,1,nil) then return end
    if ct < 1
        || ct > duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0
        || !duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_race(d, c, RACE_FIEND))), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
    {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.setfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&setfilter), a.tp, LOCATION_DECK, 0, NO_EX);
    // if aux.SelectUnselectGroup(g,e,tp,ct,ct,aux.dncheck,0) and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    let ct_n = ct as usize;
    if matches!(
        aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(ct_n), Some(ct_n), Some(&aux::dncheck()), 0, a.tp, None, None, None, false),
        Err(true)
    ) && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1))
    {
        // local sg=aux.SelectUnselectGroup(g,e,tp,ct,ct,aux.dncheck,1,tp,HINTMSG_SET)
        let sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(ct_n), Some(ct_n), Some(&aux::dncheck()), 1, a.tp, Some(HINTMSG_SET as u64), None, None, false)
            .unwrap_or_default();
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SSet(tp,sg)
            duel::s_set(d, a.tp, sg, None, true);
        }
    }
}
