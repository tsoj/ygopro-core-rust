//! Dark Spirit's Mastery
use super::prelude::*;

pub const ID: u32 = 14386013;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_DESTINY_BOARD], listed_series: &[SET_SPIRIT_MESSAGE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // To Deck and Draw
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_DRAW);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(aux::exccon()));
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(tdrtg)));
    effect::set_operation(d, e2, Some(operation(tdrop)));
    card::register_effect(d, c, e2, false);
}
/// `s.cost(e,tp,...,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), a.tp, LOCATION_HAND, 0, 1, h.into());
    }
    // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, a.tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsCode(CARD_DESTINY_BOARD) or c:IsRace(RACE_FIEND) and c:GetLevel()==8) and c:IsAbleToHand()
    (card::is_code(d, c, &[CARD_DESTINY_BOARD]) || card::is_race(d, c, RACE_FIEND) && card::get_level(d, c) == 8) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil)
    let nf = aux::necro_valley_filter(thfilter);
    let g = duel::select_matching_card(d, a.tp, Some(&nf), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, &g);
    }
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsCode(CARD_DESTINY_BOARD) or c:IsSetCard(SET_SPIRIT_MESSAGE)) and c:IsAbleToDeck()
    (card::is_code(d, c, &[CARD_DESTINY_BOARD]) || card::is_set_card(d, c, &[SET_SPIRIT_MESSAGE as u16])) && card::is_able_to_deck(d, c)
}
/// `s.tdrtg(e,tp,...,chk)`
fn tdrtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(s.tdfilter,tp,LOCATION_HAND|LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&tdfilter), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, NO_EX);
    // if chk==0 then return Duel.IsPlayerCanDraw(tp)
    //   and aux.SelectUnselectGroup(g,e,tp,1,g:GetClassCount(Card.GetCode),aux.dncheck,0) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 0) && {
            let ct = group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64);
            let dn = aux::dncheck();
            matches!(aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(1), Some(ct), Some(&dn), 0, a.tp, None, None, None, false), Err(true))
        };
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// `s.tdrop(e,tp,...)`
fn tdrop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.tdfilter),tp,LOCATION_HAND|LOCATION_GRAVE,0,nil)
    let nf = aux::necro_valley_filter(tdfilter);
    let g = duel::get_matching_group(d, Some(&nf), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,1,g:GetClassCount(Card.GetCode),s.rescon,1,tp,HINTMSG_TODECK)
    //   (s.rescon is not defined by this script: nil)
    let ct = group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64);
    let sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(1), Some(ct), None, 1, a.tp, Some(HINTMSG_TODECK as u64), None, None, false).unwrap_or_default();
    // Duel.ConfirmCards(1-tp,sg)
    duel::confirm_cards(d, 1 - a.tp, &sg);
    // Duel.SendtoDeck(sg,nil,SEQ_DECKBOTTOM,REASON_EFFECT)
    duel::send_to_deck(d, sg, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None);
    // local og=Duel.GetOperatedGroup()
    let og = duel::get_operated_group(d);
    // local ct=og:FilterCount(Card.IsLocation,nil,LOCATION_DECK)
    let ct = group::filter_count(d, &og, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX) as i64;
    // if ct>0 then
    if ct > 0 {
        // Duel.SortDeckbottom(tp,tp,ct)
        duel::sort_deckbottom(d, a.tp, a.tp, ct as u16);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(tp,ct,REASON_EFFECT)
        duel::draw(d, a.tp, ct as u32, REASON_EFFECT);
    }
}
