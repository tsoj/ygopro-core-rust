//! Dogmatikaturgy
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 42158279;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DOGMATIKA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Ritual.CreateProc({handler=c,lvtype=RITPROC_EQUAL,filter=aux.FilterBoolFunction(Card.IsSetCard,SET_DOGMATIKA),location=LOCATION_HAND|LOCATION_DECK,matfilter=s.mfilter})
    let e1 = ritual::create_proc(
        d,
        c,
        ritual::RitualProc {
            lvtype: RITPROC_EQUAL,
            filter: Some(Arc::new(|d: &mut Duel, x: CardId| card::is_set_card(d, x, &[SET_DOGMATIKA as u16]))),
            location: Some(LOCATION_HAND | LOCATION_DECK),
            matfilter: Some(ritual::MatFilt::F(Arc::new(|d: &mut Duel, x: CardId, _e: EffectId, _tp: u8| mfilter(d, x)))),
            ..Default::default()
        },
    );
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add 2 "Dogmatika" monsters from your GY to your hand and Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_TODECK);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_END_PHASE));
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.thdtg)
    effect::set_target(d, e2, Some(cost_target(thdtg)));
    // e2:SetOperation(s.thdop)
    effect::set_operation(d, e2, Some(operation(thdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.mfilter(c)`
fn mfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLocation(LOCATION_MZONE) and (c:IsSetCard(SET_DOGMATIKA) or c:IsType(TYPE_RITUAL+TYPE_FUSION+TYPE_SYNCHRO))
    card::is_location(d, c, LOCATION_MZONE) && (card::is_set_card(d, c, &[SET_DOGMATIKA as u16]) || card::is_type(d, c, TYPE_RITUAL + TYPE_FUSION + TYPE_SYNCHRO))
}
/// `s.thdfilter(c,e)`
fn thdfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsSetCard(SET_DOGMATIKA) and c:HasLevel() and (c:IsAbleToHand() or c:IsAbleToDeck())
    //     and c:IsCanBeEffectTarget(e)
    card::is_set_card(d, c, &[SET_DOGMATIKA as u16]) && card::has_level(d, c) && (card::is_able_to_hand(d, c, None) || card::is_able_to_deck(d, c)) && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetClassCount(Card.GetLevel)==#sg and sg:FilterCount(Card.IsAbleToHand,nil)>=1
    //     and sg:FilterCount(Card.IsAbleToDeck,nil)>=1
    let ok = group::get_class_count(d, sg, &|d, c| card::get_level(d, c) as i64) == sg.len()
        && group::filter_count(d, sg, &|d, c| card::is_able_to_hand(d, c, None), NO_EX) >= 1
        && group::filter_count(d, sg, &|d, c| card::is_able_to_deck(d, c), NO_EX) >= 1;
    (ok, false)
}
/// `s.thdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local rg=Duel.GetMatchingGroup(s.thdfilter,tp,LOCATION_GRAVE,0,nil,e)
    let rg = duel::get_matching_group(d, Some(&|d, c| thdfilter(d, c, e)), tp, LOCATION_GRAVE, 0, NO_EX);
    // if chk==0 then return aux.SelectUnselectGroup(rg,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return matches!(
            aux::select_unselect_group(d, &rg, Some(e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false),
            Err(true)
        );
    }
    // local g=aux.SelectUnselectGroup(rg,e,tp,2,2,s.rescon,1,tp,aux.Stringid(id,2))
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(aux::stringid(ID, 2)), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g.clone()), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), 1, 0, 0);
    true
}
/// `s.thdop(e,tp,eg,ep,ev,re,r,rp)`
fn thdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // if #g==2 then
    if g.len() == 2 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local hg=g:FilterSelect(tp,Card.IsAbleToHand,1,1,nil)
        let hg = group::filter_select(d, &g, tp, &|d, c| card::is_able_to_hand(d, c, None), 1, 1, false, NO_EX).nil();
        // if #hg==0 or Duel.SendtoHand(hg,nil,REASON_EFFECT)==0 then return end
        if hg.is_empty() || duel::send_to_hand(d, hg.clone(), None, REASON_EFFECT, None) == 0 {
            return;
        }
        // Duel.ConfirmCards(1-tp,hg)
        duel::confirm_cards(d, 1 - tp, hg.clone());
        // local dg=g-hg
        let mut dg = g.clone();
        group::remove_card(&mut dg, &hg);
        // if #dg==0 then return end
        if dg.is_empty() {
            return;
        }
        // Duel.HintSelection(dg,true)
        duel::hint_selection(d, dg.clone(), true);
        // Duel.SendtoDeck(dg,nil,SEQ_DECKBOTTOM,REASON_EFFECT)
        duel::send_to_deck(d, dg, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None);
    }
}
