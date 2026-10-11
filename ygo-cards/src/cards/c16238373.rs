//! Psychic Arsenal
use super::prelude::*;

pub const ID: u32 = 16238373;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 Machine monster with the from your Deck to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SEARCH+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_SEARCH + CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Return 1 banished Psychic/Machine monster to deck and add another to the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TODECK+CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(aux.exccon)
    effect::set_condition(d, e2, Some(aux::exccon()));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.cfilter(c,tp)
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_PSYCHIC) and c:HasLevel() and Duel.CheckLPCost(tp,c:GetLevel()*200)
    //   and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil,c:GetAttribute(),c:GetLevel())
    if !(card::is_faceup(d, c) && card::is_race(d, c, RACE_PSYCHIC) && card::has_level(d, c)) {
        return false;
    }
    let lp = card::get_level(d, c) * 200;
    if !duel::check_lp_cost(d, tp, lp) {
        return false;
    }
    let att = card::get_attribute(d, c);
    let lv = card::get_level(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| thfilter(d, x, att, lv)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
// function s.thfilter(c,att,lv)
fn thfilter(d: &mut Duel, c: CardId, att: u32, lv: u32) -> bool {
    // return c:IsRace(RACE_MACHINE) and c:IsAttribute(att) and c:GetLevel()>lv and c:IsAbleToHand()
    card::is_race(d, c, RACE_MACHINE) && card::is_attribute(d, c, att) && card::get_level(d, c) > lv && card::is_able_to_hand(d, c, None)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.cfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && cfilter(d, chkc, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.cfilter,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| cfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local tc=Duel.SelectTarget(tp,s.cfilter,tp,LOCATION_MZONE,0,1,1,nil,tp):GetFirst()
    let sel = duel::select_target(d, tp, Some(&|d, c| cfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel).nil();
    // Duel.PayLPCost(tp,tc:GetLevel()*200)
    let lp = card::get_level(d, tc) * 200;
    duel::pay_lp_cost(d, tp, lp);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) or tc:IsFacedown() then return end
    if !card::is_relate_to_effect(d, tc, a.e) || card::is_facedown(d, tc) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil,tc:GetAttribute(),tc:GetLevel())
    let att = card::get_attribute(d, tc);
    let lv = card::get_level(d, tc);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| thfilter(d, c, att, lv)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
// function s.tdhndfilter(c,e)
fn tdhndfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_PSYCHIC|RACE_MACHINE)
    //   and (c:IsAbleToDeck() or c:IsAbleToHand()) and c:IsCanBeEffectTarget(e)
    card::is_faceup(d, c)
        && card::is_race(d, c, RACE_PSYCHIC | RACE_MACHINE)
        && (card::is_able_to_deck(d, c) || card::is_able_to_hand(d, c, None))
        && card::is_can_be_effect_target(d, c, Some(e))
}
// function s.rescon(sg,e,tp,mg)
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetClassCount(Card.GetRace)==#sg
    //   and sg:IsExists(Card.IsAbleToDeck,1,nil)
    //   and sg:IsExists(Card.IsAbleToHand,1,nil)
    let res = group::get_class_count(d, sg, &|d, c| card::get_race(d, c) as i64) == sg.len()
        && group::is_exists(d, sg, &|d, c| card::is_able_to_deck(d, c), 1, NO_EX)
        && group::is_exists(d, sg, &|d, c| card::is_able_to_hand(d, c, None), 1, NO_EX);
    (res, false)
}
// function s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.tdhndfilter,tp,LOCATION_REMOVED,0,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, c| tdhndfilter(d, c, e)), tp, LOCATION_REMOVED, 0, NO_EX);
    // if chk==0 then return #g>=2 and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return g.len() >= 2
            && matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false), Err(true));
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_TARGET)
    let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_TARGET as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(sg)
    duel::set_target_card(d, sg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,sg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(sg.clone()), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,sg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(sg), 1, tp, 0);
    true
}
// function s.tdop(e,tp,eg,ep,ev,re,r,rp)
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tg=Duel.GetTargetCards(e)
    let mut tg = duel::get_target_cards(d, a.e).nil();
    // if #tg==0 then return end
    if tg.is_empty() {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local tdg=tg:FilterSelect(tp,Card.IsAbleToDeck,1,1,nil)
    let tdg = group::filter_select(d, &tg, tp, &card::is_able_to_deck, 1, 1, false, NO_EX).nil();
    // if #tdg==0 then return end
    if tdg.is_empty() {
        return;
    }
    // Duel.HintSelection(tdg,true)
    duel::hint_selection(d, tdg.clone(), true);
    // if Duel.SendtoDeck(tdg,nil,SEQ_DECKBOTTOM,REASON_EFFECT)==0 or #tg==1 then return end
    if duel::send_to_deck(d, tdg.clone(), None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None) == 0 || tg.len() == 1 {
        return;
    }
    // tg:RemoveCard(tdg)
    group::remove_card(&mut tg, &tdg);
    // if Duel.SendtoHand(tg,nil,REASON_EFFECT)>0 then
    if duel::send_to_hand(d, tg.clone(), None, REASON_EFFECT, None) > 0 {
        // Duel.ConfirmCards(1-tp,tg)
        duel::confirm_cards(d, 1 - tp, tg);
    }
}
