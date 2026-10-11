//! Monster Assortment
use super::prelude::*;

pub const ID: u32 = 23270035;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
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
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsType(TYPE_NORMAL|TYPE_EFFECT) and c:HasLevel() and c:IsAbleToHand()
    card::is_monster(d, c) && card::is_type(d, c, TYPE_NORMAL | TYPE_EFFECT) && card::has_level(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.check(sg,e,tp)`
fn check(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetClassCount(Card.GetRace)==1 and sg:GetClassCount(Card.GetAttribute)==1
    //     and sg:GetClassCount(Card.GetLevel)==1 and sg:FilterCount(Card.IsType,nil,TYPE_NORMAL)==1
    //     and sg:FilterCount(Card.IsType,nil,TYPE_EFFECT)==1
    let res = group::get_class_count(d, sg, &|d, c| card::get_race(d, c) as i64) == 1
        && group::get_class_count(d, sg, &|d, c| card::get_attribute(d, c) as i64) == 1
        && group::get_class_count(d, sg, &|d, c| card::get_level(d, c) as i64) == 1
        && group::filter_count(d, sg, &|d, c| card::is_type(d, c, TYPE_NORMAL), NO_EX) == 1
        && group::filter_count(d, sg, &|d, c| card::is_type(d, c, TYPE_EFFECT), NO_EX) == 1;
    (res, false)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_DECK,0,nil)
        let g = duel::get_matching_group(d, Some(&filter), tp, LOCATION_DECK, 0, NO_EX);
        // return aux.SelectUnselectGroup(g,e,tp,2,2,s.check,0)
        return matches!(
            aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&check), 0, tp, None, None, None, false),
            Err(true)
        );
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,0,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, 0, LOCATION_DECK as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local dg=Duel.GetMatchingGroup(s.filter,tp,LOCATION_DECK,0,nil)
    let dg = duel::get_matching_group(d, Some(&filter), tp, LOCATION_DECK, 0, NO_EX);
    // local g=aux.SelectUnselectGroup(dg,e,tp,2,2,s.check,1,tp,HINTMSG_CONFIRM)
    let g = aux::select_unselect_group(d, &dg, Some(e), tp, Some(2), Some(2), Some(&check), 1, tp, Some(HINTMSG_CONFIRM as u64), None, None, false).unwrap_or_default();
    // if #g==2 then
    if g.len() == 2 {
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g.clone());
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, tp);
        // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_ATOHAND as u64);
        // local tc=g:Select(1-tp,1,1,nil):GetFirst()
        let sel = group::select(d, &g, 1 - tp, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &sel).nil();
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
    }
}
