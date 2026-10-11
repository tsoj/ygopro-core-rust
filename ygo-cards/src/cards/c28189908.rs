//! T.G. Limiter Removal
use super::prelude::*;

pub const ID: u32 = 28189908;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_TG],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Search 2 "T.G." monsters with different names
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Shuffle 1 "T.G." monster in your GY into the Deck, or add it to your hand
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
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), a.tp, LOCATION_HAND, 0, 1, h.into());
    }
    // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, a.tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_TG) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_TG as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, NO_EX);
    // if chk==0 then return #g>=2 and aux.SelectUnselectGroup(g,e,tp,2,2,aux.dncheck,0) end
    if a.chk == Some(0) {
        return group::get_count(&g) >= 2
            && matches!(
                aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&aux::dncheck()), 0, a.tp, None, None, None, false),
                Err(true)
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 2, a.tp, LOCATION_DECK as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, NO_EX);
    // if #g<2 then return end
    if group::get_count(&g) < 2 {
        return;
    }
    // local thg=aux.SelectUnselectGroup(g,e,tp,2,2,aux.dncheck,1,tp,HINTMSG_ATOHAND)
    let thg = aux::select_unselect_group(
        d,
        &g,
        Some(a.e),
        a.tp,
        Some(2),
        Some(2),
        Some(&aux::dncheck()),
        1,
        a.tp,
        Some(HINTMSG_ATOHAND as u64),
        None,
        None,
        false,
    )
    .unwrap_or_default();
    // if #thg>0 then
    if group::get_count(&thg) > 0 {
        // Duel.SendtoHand(thg,nil,REASON_EFFECT)
        duel::send_to_hand(d, thg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,thg)
        duel::confirm_cards(d, 1 - a.tp, thg);
    }
}
// function s.tohandfilter(c)
fn tohandfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_TG) and c:IsRace(RACE_MACHINE)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_TG as u16]) && card::is_race(d, c, RACE_MACHINE)
}
// function s.tdfilter(c,tohand)
fn tdfilter(d: &mut Duel, c: CardId, tohand: bool) -> bool {
    // return c:IsSetCard(SET_TG) and c:IsMonster() and (c:IsAbleToDeck() or (tohand and c:IsAbleToHand()))
    card::is_set_card(d, c, &[SET_TG as u16])
        && card::is_monster(d, c)
        && (card::is_able_to_deck(d, c) || (tohand && card::is_able_to_hand(d, c, None)))
}
// function s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local tohand=Duel.IsExistingMatchingCard(s.tohandfilter,tp,LOCATION_MZONE,0,1,nil)
    let tohand = duel::is_existing_matching_card(d, Some(&tohandfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.tdfilter(chkc,tohand) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && tdfilter(d, chkc, tohand);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tdfilter,tp,LOCATION_GRAVE,0,1,nil,tohand) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| tdfilter(d, c, tohand)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,s.tdfilter,tp,LOCATION_GRAVE,0,1,1,nil,tohand)
    let g = duel::select_target(d, a.tp, Some(&|d, c| tdfilter(d, c, tohand)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // if not tohand then
    if !tohand {
        // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_TODECK, g.clone(), 1, a.tp, 0);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TODECK,g,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, g.clone(), 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,g,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, g, 1, a.tp, 0);
    true
}
// function s.tdop(e,tp,eg,ep,ev,re,r,rp)
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local tohand=Duel.IsExistingMatchingCard(s.tohandfilter,tp,LOCATION_MZONE,0,1,nil)
    let tohand = duel::is_existing_matching_card(d, Some(&tohandfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    // if tohand then
    if tohand {
        // aux.ToHandOrElse(tc,tp, function() return tc:IsAbleToDeck() end,
        //     function() Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT) end, aux.Stringid(id,2))
        aux::to_hand_or_else(
            d,
            tc,
            a.tp,
            Some(&|d, _c| card::is_able_to_deck(d, tc)),
            Some(&|d, _g| {
                duel::send_to_deck(d, CardSet::from_card(tc), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
                0
            }),
            Some(aux::stringid(ID, 2)),
        );
    } else {
        // Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, CardSet::from_card(tc), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
