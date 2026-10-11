//! Fifty Fifty?
use super::prelude::*;

pub const ID: u32 = 71275181;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:IsExists(Card.IsMonster,1,nil) and sg:IsExists(Card.IsTrap,2,nil)
    let res = group::is_exists(d, sg, &|d, c| card::is_monster(d, c), 1, NO_EX) && group::is_exists(d, sg, &|d, c| card::is_trap(d, c), 2, NO_EX);
    (res, false)
}
/// `Duel.GetMatchingGroup(aux.OR(Card.IsMonster,Card.IsTrap),tp,LOCATION_DECK,0,nil)`
fn monster_or_trap_group(d: &mut Duel, tp: u8) -> CardSet {
    duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c) || card::is_trap(d, c)), tp, LOCATION_DECK, 0, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(aux.OR(Card.IsMonster,Card.IsTrap),tp,LOCATION_DECK,0,nil)
    let g = monster_or_trap_group(d, a.tp);
    // if chk==0 then return #g>=3 and Duel.IsPlayerCanRemove(tp) and aux.SelectUnselectGroup(g,e,tp,3,3,s.rescon,0) end
    if a.chk == Some(0) {
        return g.len() >= 3
            && duel::is_player_can_remove(d, a.tp, None)
            && matches!(aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(3), Some(3), Some(&rescon), 0, a.tp, None, None, None, false), Err(true));
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if not Duel.IsPlayerCanRemove(tp) then return end
    if !duel::is_player_can_remove(d, a.tp, None) {
        return;
    }
    // local g=Duel.GetMatchingGroup(aux.OR(Card.IsMonster,Card.IsTrap),tp,LOCATION_DECK,0,nil)
    let g = monster_or_trap_group(d, a.tp);
    // if #g<3 then return end
    if g.len() < 3 {
        return;
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,3,3,s.rescon,1,tp,HINTMSG_CONFIRM)
    let mut sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(3), Some(3), Some(&rescon), 1, a.tp, Some(HINTMSG_CONFIRM as u64), None, None, false).unwrap_or_default();
    // if #sg==0 then return end
    if sg.is_empty() {
        return;
    }
    // local opp=1-tp
    let opp = 1 - a.tp;
    // Duel.ConfirmCards(opp,sg)
    duel::confirm_cards(d, opp, sg.clone());
    // local picked_card=sg:RandomSelect(opp,1):GetFirst()
    let picked_group = group::random_select(d, &sg, opp, 1);
    let picked_card = group::get_first(d, &picked_group).nil();
    // sg:RemoveCard(picked_card)
    group::remove_card(&mut sg, &CardSet::from_card(picked_card));
    // Duel.ConfirmCards(tp,sg)
    duel::confirm_cards(d, a.tp, sg.clone());
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local banished_card=sg:FilterSelect(tp,aux.AND(Card.IsTrap,Card.IsAbleToRemove),1,1,nil)
    let banished = group::filter_select(d, &sg, a.tp, &|d, c| card::is_trap(d, c) && card::is_able_to_remove(d, c, None, None, None), 1, 1, false, NO_EX).nil();
    // if #banished_card==0 or Duel.Remove(banished_card,POS_FACEUP,REASON_EFFECT)==0 then return end
    if banished.is_empty() || duel::remove(d, banished.clone(), POS_FACEUP as u8, REASON_EFFECT, None, None) == 0 {
        return;
    }
    // sg:RemoveCard(banished_card)
    group::remove_card(&mut sg, &banished);
    // local op=Duel.SelectEffect(opp,{true,aux.Stringid(id,1)},{true,aux.Stringid(id,2)})
    let op = duel::select_effect(d, opp, &[(true, aux::stringid(ID, 1)), (true, aux::stringid(ID, 2))]);
    // Duel.BreakEffect()
    duel::break_effect(d);
    // if op==1 then
    if op == Some(1) {
        // Duel.ConfirmCards(opp,picked_card)
        duel::confirm_cards(d, opp, CardSet::from_card(picked_card));
        // if picked_card:IsMonster() then
        if card::is_monster(d, picked_card) {
            // aux.ToHandOrElse(picked_card,tp, function() return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and picked_card:IsCanBeSpecialSummoned(e,0,tp,false,false) end,
            //     function() Duel.SpecialSummon(picked_card,0,tp,tp,false,false,POS_FACEUP) end, aux.Stringid(id,3))
            let check = |d: &mut Duel, _c: CardId| {
                duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && card::is_can_be_special_summoned(d, picked_card, a.e, 0, a.tp, false, false, None, None, None)
            };
            let oper = |d: &mut Duel, _g: &CardSet| duel::special_summon(d, CardSet::from_card(picked_card), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
            aux::to_hand_or_else(d, picked_card, a.tp, Some(&check), Some(&oper), Some(aux::stringid(ID, 3)));
        } else {
            // Duel.Remove(picked_card,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, CardSet::from_card(picked_card), POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    } else if op == Some(2) {
        // local other_card=sg:GetFirst()
        let other_card = group::get_first(d, &sg).nil();
        // Duel.ConfirmCards(opp,other_card)
        duel::confirm_cards(d, opp, CardSet::from_card(other_card));
        // if other_card:IsMonster() then
        if card::is_monster(d, other_card) {
            // Duel.SendtoHand(other_card,nil,REASON_EFFECT)
            duel::send_to_hand(d, CardSet::from_card(other_card), None, REASON_EFFECT, None);
        } else {
            // Duel.Remove(other_card,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, CardSet::from_card(other_card), POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
    // Duel.ShuffleDeck(tp)
    duel::shuffle_deck(d, a.tp);
}
