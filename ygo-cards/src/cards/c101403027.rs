//! Reindsurm Gigalovendra
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 101403027;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[101403055], listed_series: &[SET_REINDSURM], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // (Quick Effect): You can send 2 cards from your hand to the GY, including this card; add 1 non-FIRE "Reindsurm" monster from your Deck to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.deckthcost)
    effect::set_cost(d, e1, Some(cost_target(deckthcost)));
    // e1:SetTarget(s.deckthtg)
    effect::set_target(d, e1, Some(cost_target(deckthtg)));
    // e1:SetOperation(s.deckthop)
    effect::set_operation(d, e1, Some(operation(deckthop)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some((TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E) as u32));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card is Ritual Summoned, or your opponent Special Summons a monster(s): You can shuffle up to 2 monsters on the field into the Deck
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2a, aux::stringid(ID, 1));
    // e2a:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e2a, CATEGORY_TODECK);
    // e2a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2a:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2a, EFFECT_FLAG_DELAY, 0);
    // e2a:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2a, EVENT_SPSUMMON_SUCCESS);
    // e2a:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2a, 1, ID, 1, 0);
    // e2a:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return e:GetHandler():IsRitualSummoned() end)
    effect::set_condition(
        d,
        e2a,
        Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            card::is_ritual_summoned(d, h)
        })),
    );
    // e2a:SetTarget(s.tdtg)
    effect::set_target(d, e2a, Some(cost_target(tdtg)));
    // e2a:SetOperation(s.tdop)
    effect::set_operation(d, e2a, Some(operation(tdop)));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=e2a:Clone()
    let e2b = effect::clone(d, e2a);
    // e2b:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2b, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2b:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2b, LOCATION_MZONE);
    // e2b:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return eg:IsExists(Card.IsSummonPlayer,1,nil,1-tp) end)
    effect::set_condition(
        d,
        e2b,
        Some(condition(|d, a| {
            let eg = a.eg.clone().nil();
            group::is_exists(d, &eg, &|d, c| card::is_summon_player(d, c, 1 - a.tp), 1, NO_EX)
        })),
    );
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
    // If this card and a WATER "Reindsurm" monster are in your GY: You can add this card to your hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e3, CATEGORY_TOHAND);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetCondition(s.selfthcon)
    effect::set_condition(d, e3, Some(condition(selfthcon)));
    // e3:SetTarget(s.selfthtg)
    effect::set_target(d, e3, Some(cost_target(selfthtg)));
    // e3:SetOperation(s.selfthop)
    effect::set_operation(d, e3, Some(operation(selfthop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.deckthcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn deckthcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToGraveAsCost() and Duel.IsExistingMatchingCard(Card.IsAbleToGraveAsCost,tp,LOCATION_HAND,0,1,c) end
    if a.chk == Some(0) {
        return card::is_able_to_grave_as_cost(d, c)
            && duel::is_existing_matching_card(d, Some(&|d, x| card::is_able_to_grave_as_cost(d, x)), tp, LOCATION_HAND, 0, 1, c.into());
    }
    // local sg=Group.FromCards(c)
    let mut sg = CardSet::from_card(c);
    // local g=Duel.GetMatchingGroup(Card.IsAbleToGraveAsCost,tp,LOCATION_HAND,0,c)
    let mut g = duel::get_matching_group(d, Some(&|d, x| card::is_able_to_grave_as_cost(d, x)), tp, LOCATION_HAND, 0, c.into());
    // while #sg<2 do
    while sg.len() < 2 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // local sc=Group.SelectUnselect(g,sg,tp,false,false,2,2)
        let sc = group::select_unselect(d, &g, Some(&sg), tp, false, false, Some(2), Some(2)).nil();
        // if not sg:IsContains(sc) then
        if !sg.contains(sc) {
            // sg:AddCard(sc)
            group::add_card(&mut sg, &CardSet::from_card(sc));
            // g:RemoveCard(sc)
            group::remove_card(&mut g, &CardSet::from_card(sc));
        // elseif sc~=c then
        } else if sc != c {
            // sg:RemoveCard(sc)
            group::remove_card(&mut sg, &CardSet::from_card(sc));
            // g:AddCard(sc)
            group::add_card(&mut g, &CardSet::from_card(sc));
        }
    }
    // Duel.SendtoGrave(sg,REASON_COST)
    duel::send_to_grave(d, sg, REASON_COST, None, None);
    true
}
// function s.deckthfilter(c)
fn deckthfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttributeExcept(ATTRIBUTE_FIRE) and c:IsSetCard(SET_REINDSURM) and c:IsMonster() and c:IsAbleToHand()
    card::is_attribute_except(d, c, ATTRIBUTE_FIRE, None, 0, None)
        && card::is_set_card(d, c, &[SET_REINDSURM as u16])
        && card::is_monster(d, c)
        && card::is_able_to_hand(d, c, None)
}
// function s.deckthtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn deckthtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.deckthfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&deckthfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.deckthop(e,tp,eg,ep,ev,re,r,rp)
fn deckthop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.deckthfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&deckthfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
// function s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToDeck,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&card::is_able_to_deck), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,PLAYER_EITHER,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, PLAYER_EITHER as u8, LOCATION_MZONE as i32);
    true
}
// function s.tdop(e,tp,eg,ep,ev,re,r,rp)
fn tdop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToDeck,tp,LOCATION_MZONE,LOCATION_MZONE,1,2,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&card::is_able_to_deck), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 2, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
// function s.selfthconfilter(c)
fn selfthconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_WATER) and c:IsSetCard(SET_REINDSURM)
    card::is_attribute(d, c, ATTRIBUTE_WATER) && card::is_set_card(d, c, &[SET_REINDSURM as u16])
}
// function s.selfthcon(e,tp,eg,ep,ev,re,r,rp)
fn selfthcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.selfthconfilter,tp,LOCATION_GRAVE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&selfthconfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
// function s.selfthtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn selfthtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHand() end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
// function s.selfthop(e,tp,eg,ep,ev,re,r,rp)
fn selfthop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SendtoHand(c,nil,REASON_EFFECT)
        duel::send_to_hand(d, CardSet::from_card(c), None, REASON_EFFECT, None);
    }
}
