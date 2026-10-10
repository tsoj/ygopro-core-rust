//! Galatea, the Orcust Automaton
use super::prelude::*;

pub const ID: u32 = 30741503;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ORCUST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_EFFECT),2,nil,s.matcheck)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_EFFECT, sc, st, Some(tp)))),
        2,
        None,
        Some(group_check(matcheck)),
        None,
        None,
    );
    // --Cannot be destroyed by battle while it is linked
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetCondition(function(e) return e:GetHandler():IsLinked() end)
    effect::set_condition(d, e1, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_linked(d, h)
    })));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Shuffle 1 Machine monster into the Deck and set 1 "Orcust" Spell/Trap
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TODECK+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.tdcon1)
    effect::set_condition(d, e2, Some(condition(tdcon1)));
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetCondition(s.tdcon2)
    effect::set_condition(d, e3, Some(condition(tdcon2)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.matcheck(g,lc,sumtype,tp)
fn matcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:IsExists(Card.IsSetCard,1,nil,SET_ORCUST,lc,sumtype,tp)
    group::is_exists(d, g, &|d, c| card::is_set_card_ctx(d, c, &[SET_ORCUST as u16], Some(lc), sumtype, Some(tp)), 1, NO_EX)
}
// function s.tdcon1(e,tp,eg,ep,ev,re,r,rp)
fn tdcon1(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsPlayerAffectedByEffect(tp,CARD_ORCUSTRATED_BABEL)
    duel::is_player_affected_by_effect(d, a.tp, CARD_ORCUSTRATED_BABEL).is_none()
}
// function s.tdcon2(e,tp,eg,ep,ev,re,r,rp)
fn tdcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsPlayerAffectedByEffect(tp,CARD_ORCUSTRATED_BABEL)
    duel::is_player_affected_by_effect(d, a.tp, CARD_ORCUSTRATED_BABEL).is_some()
}
// function s.tdfilter(c)
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_MACHINE) and c:IsAbleToDeck()
    card::is_faceup(d, c) && card::is_race(d, c, RACE_MACHINE) && card::is_able_to_deck(d, c)
}
// function s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_REMOVED) and s.tdfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_REMOVED) && tdfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tdfilter,tp,LOCATION_REMOVED,0,1,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tdfilter), a.tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.tdfilter,tp,LOCATION_REMOVED,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&tdfilter), a.tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,0,0)
    let n = g.clone().nil().len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, g, n, 0, 0);
    true
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ORCUST) and c:IsSpellTrap() and c:IsSSetable()
    card::is_set_card(d, c, &[SET_ORCUST as u16]) && card::is_spell_trap(d, c) && card::is_s_setable(d, c, false, None)
}
// function s.tdop(e,tp,eg,ep,ev,re,r,rp)
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let _c = effect::get_handler(d, a.e).nil();
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg==0 then return end
    if tg.is_empty() {
        return;
    }
    // Duel.SendtoDeck(tg,nil,SEQ_DECKTOP,REASON_EFFECT)
    duel::send_to_deck(d, tg, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
    // local og=Duel.GetOperatedGroup()
    let og = duel::get_operated_group(d);
    // if #og==0 then return end
    if og.is_empty() {
        return;
    }
    // if og:IsExists(Card.IsLocation,1,nil,LOCATION_DECK) then Duel.ShuffleDeck(tp) end
    if group::is_exists(d, &og, &|d, c| card::is_location(d, c, LOCATION_DECK), 1, NO_EX) {
        duel::shuffle_deck(d, a.tp);
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_DECK, 0, NO_EX);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        // Duel.SSet(tp,sg)
        duel::s_set(d, a.tp, sg, None, true);
    }
}
