//! Tearlaments Kashtira
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 4928565;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TEARLAMENTS, SET_KASHTIRA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_MAIN_END));
    // e1:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Mill 3 cards from either Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DECKDES)
    effect::set_category(d, e2, CATEGORY_DECKDES);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.smltg)
    effect::set_target(d, e2, Some(cost_target(smltg)));
    // e2:SetOperation(s.smlop)
    effect::set_operation(d, e2, Some(operation(smlop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Mill 2 cards from your Deck
    // local e4=e2:Clone()
    let e4 = effect::clone(d, e2);
    // e4:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e4, aux::stringid(ID, 2));
    // e4:SetCategory(CATEGORY_DECKDES)
    effect::set_category(d, e4, CATEGORY_DECKDES);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    // e4:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e4, EVENT_TO_GRAVE);
    // e4:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e4, 1, ID, 2, 0);
    // e4:SetCondition(function(e) return e:GetHandler():IsReason(REASON_EFFECT) end)
    effect::set_condition(d, e4, Some(condition(|d, a| {
        let handler = effect::get_handler(d, a.e).nil();
        card::is_reason(d, handler, REASON_EFFECT)
    })));
    // e4:SetTarget(s.tgmltg)
    effect::set_target(d, e4, Some(cost_target(tgmltg)));
    // e4:SetOperation(s.tgmlop)
    effect::set_operation(d, e4, Some(operation(tgmlop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.rmfilter(c)`
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard({SET_TEARLAMENTS,SET_KASHTIRA}) and c:IsAbleToRemove()
    //     and (c:IsLocation(LOCATION_HAND) or aux.SpElimFilter(c,true))
    card::is_set_card(d, c, &[SET_TEARLAMENTS as u16, SET_KASHTIRA as u16])
        && card::is_able_to_remove(d, c, None, None, None)
        && (card::is_location(d, c, LOCATION_HAND) || aux::sp_elim_filter(d, c, true, false))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.IsExistingMatchingCard(s.rmfilter,tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_MZONE,0,1,c)
    // end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None)
            && duel::is_existing_matching_card(d, Some(&rmfilter), tp, LOCATION_HAND | LOCATION_GRAVE | LOCATION_MZONE, 0, 1, c.into());
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, tp, (LOCATION_HAND | LOCATION_GRAVE | LOCATION_MZONE) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)==0 then return end
    if !card::is_relate_to_effect(d, c, a.e) || duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.rmfilter,tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&rmfilter), tp, LOCATION_HAND | LOCATION_GRAVE | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
/// `s.smltg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn smltg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsPlayerCanDiscardDeck(tp,3) or Duel.IsPlayerCanDiscardDeck(1-tp,3) end
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck(d, tp, 3) || duel::is_player_can_discard_deck(d, 1 - tp, 3);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DECKDES,nil,0,PLAYER_EITHER,3)
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, PLAYER_EITHER as u8, 3);
    true
}
/// `s.smlop(e,tp,eg,ep,ev,re,r,rp)`
fn smlop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local b1=Duel.IsPlayerCanDiscardDeck(tp,3)
    let b1 = duel::is_player_can_discard_deck(d, tp, 3);
    // local b2=Duel.IsPlayerCanDiscardDeck(1-tp,3)
    let b2 = duel::is_player_can_discard_deck(d, 1 - tp, 3);
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,3)},{b2,aux.Stringid(id,4)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 3)), (b2, aux::stringid(ID, 4))]);
    // if op then Duel.DiscardDeck(op==1 and tp or 1-tp,3,REASON_EFFECT) end
    if let Some(op) = op {
        let p = if op == 1 { tp } else { 1 - tp };
        duel::discard_deck(d, p, 3, REASON_EFFECT);
    }
}
/// `s.tgmltg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgmltg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsPlayerCanDiscardDeck(tp,2) end
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck(d, tp, 2);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DECKDES,nil,0,tp,2)
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, tp, 2);
    true
}
/// `s.tgmlop(e,tp,eg,ep,ev,re,r,rp)`
fn tgmlop(d: &mut Duel, a: &EffArgs) {
    // Duel.DiscardDeck(tp,2,REASON_EFFECT)
    duel::discard_deck(d, a.tp, 2, REASON_EFFECT);
}
