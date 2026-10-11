//! GMX Lab #5
use super::prelude::*;

pub const ID: u32 = 74378580;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    listed_series: &[SET_GMX],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Your opponent cannot activate cards or effects when you Normal or Special Summon a "GMX" monster(s)
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1a, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1a, EVENT_SUMMON_SUCCESS);
    // e1a:SetRange(LOCATION_FZONE)
    effect::set_range(d, e1a, LOCATION_FZONE);
    // e1a:SetOperation(s.limop1)
    effect::set_operation(d, e1a, Some(operation(limop1)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1b, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // local e1c=e1a:Clone()
    let e1c = effect::clone(d, e1a);
    // e1c:SetCode(EVENT_CHAIN_END)
    effect::set_code(d, e1c, EVENT_CHAIN_END);
    // e1c:SetOperation(s.limop2)
    effect::set_operation(d, e1c, Some(operation(limop2)));
    // c:RegisterEffect(e1c)
    card::register_effect(d, c, e1c, false);
    // During your Main Phase: You can Set 1 "GMX" Spell/Trap from your Deck, except "GMX Lab #5", then place 1 card
    // from your hand on top of the Deck. You can only use this effect of "GMX Lab #5" once per turn
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SET+CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_SET + CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.settg)
    effect::set_target(d, e2, Some(cost_target(settg)));
    // e2:SetOperation(s.setop)
    effect::set_operation(d, e2, Some(operation(setop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.limopfilter(c,tp)`
fn limopfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_GMX) and c:IsFaceup() and c:IsSummonPlayer(tp)
    card::is_set_card(d, c, &[SET_GMX as u16]) && card::is_faceup(d, c) && card::is_summon_player(d, c, tp)
}
/// `function(re,rp,tp) return tp==rp end`
fn chain_limit_fn() -> LuaFn {
    lua_fn(|_d, a: Args| -> bool { a.p(2) == a.p(1) })
}
/// `s.limop1(e,tp,eg,ep,ev,re,r,rp)`
fn limop1(d: &mut Duel, a: &EffArgs) {
    // if eg and eg:IsExists(s.limopfilter,1,nil,tp) then
    if let Some(eg) = &a.eg {
        if group::is_exists(d, eg, &|d, c| limopfilter(d, c, a.tp), 1, NO_EX) {
            // Duel.SetChainLimitTillChainEnd(function(re,rp,tp) return tp==rp end)
            duel::set_chain_limit_till_chain_end(d, chain_limit_fn());
        }
    }
}
/// `s.limop2(e,tp,eg,ep,ev,re,r,rp)`
fn limop2(d: &mut Duel, a: &EffArgs) {
    // local _,g=Duel.CheckEvent(EVENT_SPSUMMON_SUCCESS,true)
    let g = duel::check_event(d, EVENT_SPSUMMON_SUCCESS).and_then(|ev| ev.event_cards);
    // if g and g:IsExists(s.limopfilter,1,nil,tp) and Duel.CheckEvent(EVENT_SPSUMMON_SUCCESS) then
    if let Some(g) = &g {
        if group::is_exists(d, g, &|d, c| limopfilter(d, c, a.tp), 1, NO_EX) && duel::check_event(d, EVENT_SPSUMMON_SUCCESS).is_some() {
            // Duel.SetChainLimitTillChainEnd(function(re,rp,tp) return tp==rp end)
            duel::set_chain_limit_till_chain_end(d, chain_limit_fn());
        }
    }
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_GMX) and c:IsSpellTrap() and c:IsSSetable() and not c:IsCode(id)
    card::is_set_card(d, c, &[SET_GMX as u16])
        && card::is_spell_trap(d, c)
        && card::is_s_setable(d, c, false, None)
        && !card::is_code(d, c, &[ID])
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_DECK,0,1,nil)
    //     and Duel.IsExistingMatchingCard(Card.IsAbleToDeck,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_deck(d, c)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SET,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SET, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    // local g=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&setfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SSet(tp,g)>0 then
    if !g.is_empty() && duel::s_set(d, tp, g, None, true) > 0 {
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, tp);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
        // local sg=Duel.SelectMatchingCard(tp,Card.IsAbleToDeck,tp,LOCATION_HAND,0,1,1,nil)
        let sg = duel::select_matching_card(d, tp, Some(&|d, c| card::is_able_to_deck(d, c)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoDeck(sg,nil,SEQ_DECKTOP,REASON_EFFECT)
            duel::send_to_deck(d, sg, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
        }
    }
}
