//! D/D/D Human Resources
//! ＤＤＤの人事権
use super::prelude::*;

pub const ID: u32 = 72648577;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_DD) and (c:IsMonster() or c:IsLocation(LOCATION_PZONE))
    //     and (c:IsFaceup() or c:IsLocation(LOCATION_HAND)) and c:IsAbleToDeck()
    card::is_set_card(d, c, &[SET_DD as u16])
        && (card::is_monster(d, c) || card::is_location(d, c, LOCATION_PZONE))
        && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_HAND))
        && card::is_able_to_deck(d, c)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_DD) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_DD as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_MZONE|LOCATION_PZONE|LOCATION_GRAVE|LOCATION_HAND,0,3,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), tp, LOCATION_MZONE | LOCATION_PZONE | LOCATION_GRAVE | LOCATION_HAND, 0, 3, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,3,tp,LOCATION_MZONE|LOCATION_PZONE|LOCATION_GRAVE|LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 3, tp, (LOCATION_MZONE | LOCATION_PZONE | LOCATION_GRAVE | LOCATION_HAND) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,2,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 2, tp, LOCATION_DECK as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.filter),tp,LOCATION_MZONE|LOCATION_PZONE|LOCATION_GRAVE|LOCATION_HAND,0,nil)
    let g = duel::get_matching_group(
        d,
        Some(&aux::necro_valley_filter(|d, c| filter(d, c))),
        tp,
        LOCATION_MZONE | LOCATION_PZONE | LOCATION_GRAVE | LOCATION_HAND,
        0,
        NO_EX,
    );
    // if #g<3 then return end
    if g.len() < 3 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local sg=g:Select(tp,3,3,nil)
    let sg = group::select(d, &g, tp, 3, 3, false, NO_EX).nil();
    // local cg=sg:Filter(Card.IsLocation,nil,LOCATION_HAND)
    let cg = group::filter(d, &sg, &|d, c| card::is_location(d, c, LOCATION_HAND), NO_EX);
    // Duel.ConfirmCards(1-tp,cg)
    duel::confirm_cards(d, 1 - tp, cg);
    // Duel.SendtoDeck(sg,nil,SEQ_DECKTOP,REASON_EFFECT)
    duel::send_to_deck(d, sg.clone(), None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
    // local dg=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let dg = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
    // if #dg>1 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if dg.len() > 1 && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local tg=dg:Select(tp,2,2,nil)
        let tg = group::select(d, &dg, tp, 2, 2, false, NO_EX).nil();
        // Duel.SendtoHand(tg,nil,REASON_EFFECT)
        duel::send_to_hand(d, tg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,tg)
        duel::confirm_cards(d, 1 - tp, tg);
    // else
    } else {
        // if sg:IsExists(Card.IsLocation,1,nil,LOCATION_DECK) then
        if group::is_exists(d, &sg, &|d, c| card::is_location(d, c, LOCATION_DECK), 1, NO_EX) {
            // Duel.ShuffleDeck(tp)
            duel::shuffle_deck(d, tp);
        }
    }
}
