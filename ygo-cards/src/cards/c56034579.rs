//! GMX Chairman Kimridge
use super::prelude::*;

pub const ID: u32 = 56034579;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[18795635],
    listed_series: &[SET_GMX],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can reveal this card in your hand; add 1 "GMX Applied Experiment #55" from your Deck or GY to your hand,
    // and if you do, shuffle this card into the Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.SelfReveal)
    effect::set_cost(d, e1, Some(cost::self_reveal()));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card is Special Summoned by a monster effect: You can target 2 cards in your GY, that are "GMX" cards
    // and/or Dinosaur monsters; place them on top of your Deck in any order, then you can destroy 1 Attack Position
    // monster your opponent controls
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TODECK+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return re and re:IsMonsterEffect() end)
    effect::set_condition(d, e2, Some(condition(monster_effect_cond)));
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return re and re:IsMonsterEffect() end`
fn monster_effect_cond(d: &mut Duel, a: &EffArgs) -> bool {
    match a.re {
        Some(re) => effect::is_monster_effect(d, re),
        None => false,
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(18795635) and c:IsAbleToHand()
    card::is_code(d, c, &[18795635]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToDeck() and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return card::is_able_to_deck(d, c)
            && duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&aux::necro_valley_filter(thfilter)),
        tp,
        LOCATION_DECK | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 and Duel.SendtoHand(g,nil,REASON_EFFECT)>0 then
    if !g.is_empty() && duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) > 0 {
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
        // if c:IsRelateToEffect(e) then
        if card::is_relate_to_effect(d, c, a.e) {
            // Duel.SendtoDeck(c,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
            duel::send_to_deck(d, CardSet::from_card(c), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        }
    }
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsSetCard(SET_GMX) or c:IsRace(RACE_DINOSAUR)) and c:IsAbleToDeck()
    (card::is_set_card(d, c, &[SET_GMX as u16]) || card::is_race(d, c, RACE_DINOSAUR as u64)) && card::is_able_to_deck(d, c)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tdfilter,tp,LOCATION_GRAVE,0,2,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tdfilter), tp, LOCATION_GRAVE, 0, 2, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.tdfilter,tp,LOCATION_GRAVE,0,2,2,nil)
    let g = duel::select_target(d, tp, Some(&tdfilter), tp, LOCATION_GRAVE, 0, 2, 2, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, g, 2, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,1-tp,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, 1 - tp, LOCATION_MZONE as i32);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg>0 and Duel.SendtoDeck(tg,nil,SEQ_DECKTOP,REASON_EFFECT)>0
    //     and tg:IsExists(Card.IsLocation,1,nil,LOCATION_DECK|LOCATION_EXTRA) then
    if !tg.is_empty()
        && duel::send_to_deck(d, tg.clone(), None, SEQ_DECKTOP as i32, REASON_EFFECT, None) > 0
        && group::is_exists(d, &tg, &|d, c| card::is_location(d, c, LOCATION_DECK | LOCATION_EXTRA), 1, NO_EX)
    {
        // if Duel.GetOperatedGroup():FilterCount(Card.IsLocation,nil,LOCATION_DECK)==2 then
        let og = duel::get_operated_group(d);
        if group::filter_count(d, &og, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX) == 2 {
            // Duel.SortDecktop(tp,tp,2)
            duel::sort_decktop(d, tp, tp, 2);
        }
        // if Duel.IsExistingMatchingCard(Card.IsAttackPos,tp,0,LOCATION_MZONE,1,nil)
        //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if duel::is_existing_matching_card(d, Some(&|d, c| card::is_attack_pos(d, c)), tp, 0, LOCATION_MZONE, 1, NO_EX)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
        {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
            // local g=Duel.SelectMatchingCard(tp,Card.IsAttackPos,tp,0,LOCATION_MZONE,1,1,nil)
            let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_attack_pos(d, c)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX)
                .nil();
            // if #g>0 then
            if !g.is_empty() {
                // Duel.HintSelection(g)
                duel::hint_selection(d, g.clone(), true);
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Destroy(g,REASON_EFFECT)
                duel::destroy(d, g, REASON_EFFECT, None, None);
            }
        }
    }
}
