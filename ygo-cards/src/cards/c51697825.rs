//! Jack-In-The-Hand
//! Scripted by Naim
use super::prelude::*;

pub const ID: u32 = 51697825;

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
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.thfilter(c) return c:IsLevel(1) and c:IsMonster() end
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_level(d, c, &[1]) && card::is_monster(d, c)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local rvg=Duel.GetMatchingGroup(aux.AND(s.thfilter,Card.IsAbleToHand),tp,LOCATION_DECK,0,nil)
    let rvg = duel::get_matching_group(d, Some(&|d, c| thfilter(d, c) && card::is_able_to_hand(d, c, None)), a.tp, LOCATION_DECK, 0, NO_EX);
    // if chk==0 then return rvg:GetClassCount(Card.GetCode)>=3 end
    if a.chk == Some(0) {
        return group::get_class_count(d, &rvg, &|d, x| card::get_code(d, x)[0] as i64) >= 3;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,PLAYER_ALL,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, PLAYER_ALL as u8, LOCATION_DECK as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local rvg=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let rvg = duel::get_matching_group(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, NO_EX);
    // local g=aux.SelectUnselectGroup(rvg,e,tp,3,3,aux.dncheck,1,tp,HINTMSG_CONFIRM)
    let dn = aux::dncheck();
    let mut g = aux::select_unselect_group(d, &rvg, Some(a.e), a.tp, Some(3), Some(3), Some(&dn), 1, a.tp, Some(HINTMSG_CONFIRM as u64), None, None, false)
        .unwrap_or_default();
    // if #g==3 then
    if g.len() == 3 {
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g.clone());
        // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_ATOHAND as u64);
        // local sg=g:Select(1-tp,1,1,nil):GetFirst()
        let sel = group::select(d, &g, 1 - a.tp, 1, 1, false, NO_EX).nil();
        let sg = group::get_first(d, &sel).nil();
        // if sg:IsAbleToHand() then
        if card::is_able_to_hand(d, sg, None) {
            // Duel.SendtoHand(sg,1-tp,REASON_EFFECT)
            duel::send_to_hand(d, sg, Some(1 - a.tp), REASON_EFFECT, None);
            // Duel.ConfirmCards(tp,sg)
            duel::confirm_cards(d, a.tp, sg);
            // Duel.ShuffleHand(1-tp)
            duel::shuffle_hand(d, 1 - a.tp);
        } else {
            // Duel.SendtoGrave(sg,REASON_RULE)
            duel::send_to_grave(d, sg, REASON_RULE, None, None);
        }
        // g:RemoveCard(sg)
        let sg_set: CardSet = sg.into();
        group::remove_card(&mut g, &sg_set);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local tg=g:Select(tp,1,1,nil):GetFirst()
        let sel2 = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        let tg = group::get_first(d, &sel2).nil();
        // if tg:IsAbleToHand() then
        if card::is_able_to_hand(d, tg, None) {
            // Duel.SendtoHand(tg,tp,REASON_EFFECT)
            duel::send_to_hand(d, tg, Some(a.tp), REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,tg)
            duel::confirm_cards(d, 1 - a.tp, tg);
            // Duel.ShuffleHand(tp)
            duel::shuffle_hand(d, a.tp);
        } else {
            // Duel.SendtoGrave(tg,REASON_RULE)
            duel::send_to_grave(d, tg, REASON_RULE, None, None);
        }
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, a.tp);
    }
}
