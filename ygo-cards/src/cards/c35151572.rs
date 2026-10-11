//! Shaman of the Ashened City
use super::prelude::*;

pub const ID: u32 = 35151572;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_OBSIDIM_ASHENED_CITY], listed_series: &[SET_ASHENED], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card (from your hand)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.selfspcon)
    effect::set_condition(d, e1, Some(lua_fn(selfspcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Shuffle 3 of your Pyro monsters that are banished and/or in your GY into the Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TODECK+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.selfspcon(e,c)`
fn selfspcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // return Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_OBSIDIM_ASHENED_CITY),0,LOCATION_FZONE,LOCATION_FZONE,1,nil)
    let tp = card::get_controler(d, c);
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&faceup_obsidim), 0, LOCATION_FZONE, LOCATION_FZONE, 1, NO_EX)
}
/// `aux.FaceupFilter(Card.IsCode,CARD_OBSIDIM_ASHENED_CITY)`
fn faceup_obsidim(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_code(d, c, &[CARD_OBSIDIM_ASHENED_CITY])
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_PYRO) and c:IsFaceup() and c:IsAbleToDeck()
    card::is_race(d, c, RACE_PYRO) && card::is_faceup(d, c) && card::is_able_to_deck(d, c)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tdfilter,tp,LOCATION_REMOVED|LOCATION_GRAVE,0,3,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tdfilter), tp, LOCATION_REMOVED | LOCATION_GRAVE, 0, 3, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.tdfilter,tp,LOCATION_REMOVED|LOCATION_GRAVE,0,3,3,nil)
    let g = duel::select_target(d, tp, Some(&tdfilter), tp, LOCATION_REMOVED | LOCATION_GRAVE, 0, 3, 3, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,3,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, g, 3, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARD_OBSIDIM_ASHENED_CITY) and c:IsAbleToHand()
    card::is_code(d, c, &[CARD_OBSIDIM_ASHENED_CITY]) && card::is_able_to_hand(d, c, None)
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg==0 or Duel.SendtoDeck(tg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)==0 then return end
    if tg.is_empty() || duel::send_to_deck(d, tg.clone(), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) == 0 {
        return;
    }
    // local og=Duel.GetOperatedGroup():Filter(Card.IsLocation,nil,LOCATION_DECK|LOCATION_EXTRA)
    let operated = duel::get_operated_group(d);
    let og = group::filter(d, &operated, &|d, c| card::is_location(d, c, LOCATION_DECK | LOCATION_EXTRA), NO_EX);
    // if #og==0 or not og:IsExists(Card.IsSetCard,1,nil,SET_ASHENED) then return end
    if og.is_empty() || !group::is_exists(d, &og, &|d, c| card::is_set_card(d, c, &[SET_ASHENED as u16]), 1, NO_EX) {
        return;
    }
    // if og:IsExists(Card.IsLocation,1,nil,LOCATION_DECK) then Duel.ShuffleDeck(tp) end
    if group::is_exists(d, &og, &|d, c| card::is_location(d, c, LOCATION_DECK), 1, NO_EX) {
        duel::shuffle_deck(d, tp);
    }
    // if Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX) && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, g);
        }
    }
}
