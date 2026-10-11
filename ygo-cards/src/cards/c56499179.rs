//! Genro Ryu-Ge Hakva
use super::prelude::*;

pub const ID: u32 = 56499179;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[55154344, ID], listed_series: &[SET_RYU_GE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 "Ryu-Ge Realm - Wyrm Winds" from your Deck to your hand
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e1, Some(cost::self_banish()));
    // e1:SetTarget(s.thgatetg)
    effect::set_target(d, e1, Some(cost_target(thgatetg)));
    // e1:SetOperation(s.thgateop)
    effect::set_operation(d, e1, Some(operation(thgateop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon this card
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e2:SetCode(EVENT_REMOVE)
    effect::set_code(d, e2, EVENT_REMOVE);
    // e2:SetRange(LOCATION_REMOVED)
    effect::set_range(d, e2, LOCATION_REMOVED);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp,eg) return not eg:IsContains(e:GetHandler()) and eg:IsExists(aux.FaceupFilter(Card.IsMonster),1,nil) end)
    effect::set_condition(d, e2, Some(condition(cond2)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Add 1 "Ryu-Ge" card from your Deck, GY, or banishment to your hand
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetCost(s.thcost)
    effect::set_cost(d, e3, Some(cost_target(thcost)));
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.thgatefilter(c)`: `return c:IsCode(55154344) and c:IsAbleToHand()`
fn thgatefilter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[55154344]) && card::is_able_to_hand(d, c, None)
}
/// `s.thgatetg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thgatetg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thgatefilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thgatefilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thgateop(e,tp,eg,ep,ev,re,r,rp)`
fn thgateop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thgatefilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thgatefilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
/// `s.spconfilter(c)`: `return c:IsMonster() and c:IsFaceup() and not c:IsCode(id)`
fn spconfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_monster(d, c) && card::is_faceup(d, c) && !card::is_code(d, c, &[ID])
}
/// `s.cond` of e2: `not eg:IsContains(e:GetHandler()) and eg:IsExists(aux.FaceupFilter(Card.IsMonster),1,nil)`
fn cond2(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, a.e).nil();
    !group::is_contains(&eg, h) && group::is_exists(d, &eg, &|d, c| card::is_faceup(d, c) && card::is_monster(d, c), 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //   and Duel.IsExistingMatchingCard(s.spconfilter,tp,LOCATION_REMOVED,LOCATION_REMOVED,2,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None)
            && duel::is_existing_matching_card(d, Some(&spconfilter), a.tp, LOCATION_REMOVED, LOCATION_REMOVED, 2, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, CardSet::from_card(c), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.tdfilter(c)`: `return c:IsCode(55154344) and c:IsFaceup() and c:IsAbleToDeckAsCost()`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[55154344]) && card::is_faceup(d, c) && card::is_able_to_deck_as_cost(d, c)
}
/// `s.thcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let _c = effect::get_handler(d, a.e);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tdfilter,tp,LOCATION_ONFIELD,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tdfilter), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(tp,s.tdfilter,tp,LOCATION_ONFIELD,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&tdfilter), a.tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    // Duel.HintSelection(g)
    duel::hint_selection(d, g.clone(), true);
    // Duel.SendtoDeck(g,nil,SEQ_DECKBOTTOM,REASON_COST)
    duel::send_to_deck(d, g, None, SEQ_DECKBOTTOM as i32, REASON_COST, None);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_RYU_GE) and c:IsAbleToHand() and not c:IsCode(id)
    //   and (c:IsFaceup() or not c:IsLocation(LOCATION_REMOVED))
    card::is_set_card(d, c, &[SET_RYU_GE as u16])
        && card::is_able_to_hand(d, c, None)
        && !card::is_code(d, c, &[ID])
        && (card::is_faceup(d, c) || !card::is_location(d, c, LOCATION_REMOVED))
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK|LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK | LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE|LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE|LOCATION_REMOVED,0,1,1,nil)
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&aux::necro_valley_filter(thfilter)),
        a.tp,
        LOCATION_DECK | LOCATION_GRAVE | LOCATION_REMOVED,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
