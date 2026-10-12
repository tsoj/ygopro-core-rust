//! Flashforce Sword
use super::prelude::*;

pub const ID: u32 = 100459021;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,0,LOCATION_HAND,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)), tp, 0, LOCATION_HAND, 1, NO_EX);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local cd=e:GetChainData()
    // if e:IsHasType(EFFECT_TYPE_ACTIVATE) and not c:IsStatus(STATUS_ACT_FROM_HAND) and c:IsPreviousPosition(POS_FACEDOWN) then
    if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE)
        && !card::is_status(d, c, STATUS_ACT_FROM_HAND)
        && card::is_previous_position(d, c, POS_FACEDOWN)
    {
        // e:SetCategory(CATEGORY_REMOVE+CATEGORY_TOHAND+CATEGORY_SEARCH)
        effect::set_category(d, a.e, CATEGORY_REMOVE + CATEGORY_TOHAND + CATEGORY_SEARCH);
        // cd.set_before_activation=true
        chain::set_effect_data(d, a.e, None, chain::field::CARD, Value::Bool(true));
        // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,1-tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE)
        duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
        duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    } else {
        // e:SetCategory(CATEGORY_REMOVE)
        effect::set_category(d, a.e, CATEGORY_REMOVE);
        // cd.set_before_activation=false
        chain::set_effect_data(d, a.e, None, chain::field::CARD, Value::Bool(false));
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,1-tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - tp, LOCATION_HAND as i32);
    true
}
/// `s.thfilter(c,race,attr,lv)`
fn thfilter(d: &mut Duel, c: CardId, race: u64, attr: u32, lv: u32) -> bool {
    // return c:IsLevelAbove(5) and (c:IsRace(race) or c:IsAttribute(attr) or c:IsLevel(lv)) and c:IsAbleToHand()
    card::is_level_above(d, c, 5)
        && (card::is_race(d, c, race) || card::is_attribute(d, c, attr) || card::is_level(d, c, &[lv]))
        && card::is_able_to_hand(d, c, None)
}
/// `s.banfilter(c,code,opp)`
fn banfilter(d: &mut Duel, c: CardId, code: u32, opp: u8) -> bool {
    // return c:IsMonster() and c:IsCode(code) and c:IsAbleToRemove(opp)
    card::is_monster(d, c) && card::is_code(d, c, &[code]) && card::is_able_to_remove(d, c, Some(opp), None, None)
}
/// The `oper` of aux.RemoveUntil: `Duel.Hint(HINT_CARD,0,id)` and return the banished card to the hand.
fn remove_until_oper(d: &mut Duel, rg: &CardSet, _a: &EffArgs) -> bool {
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    let first = group::get_first(d, rg).nil();
    let prev = card::get_previous_controler(d, first);
    duel::send_to_hand(d, rg.clone(), Some(prev), REASON_EFFECT, None);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local sc=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_HAND,nil):RandomSelect(tp,1):GetFirst()
    let hand = duel::get_matching_group(d, Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)), tp, 0, LOCATION_HAND, NO_EX);
    let picked = group::random_select(d, &hand, tp, 1);
    // if not sc then return end
    let Some(sc) = group::get_first(d, &picked) else {
        return;
    };
    // local turn_count=Duel.GetTurnCount()
    let turn_count = duel::get_turn_count(d, None);
    // local reset_count=Duel.GetCurrentPhase()<=PHASE_STANDBY and 2 or 1
    let reset_count: u16 = if duel::get_current_phase(d) <= PHASE_STANDBY { 2 } else { 1 };
    // Banish (face-up) 1 random card from your opponent's hand (until the Standby Phase of the next turn)
    let oper: aux::DelayedFn = std::sync::Arc::new(remove_until_oper);
    let cond: aux::DelayedFn = std::sync::Arc::new(move |d: &mut Duel, _rg: &CardSet, _a: &EffArgs| duel::get_turn_count(d, None) == turn_count + 1);
    if aux::remove_until(
        d,
        CardSet::from_card(sc),
        Some(POS_FACEUP as u8),
        REASON_EFFECT,
        PHASE_STANDBY,
        ID,
        a.e,
        tp,
        Some(oper),
        Some(cond),
        Some(RESET_PHASE | PHASE_STANDBY),
        Some(reset_count),
        None,
        None,
    )
    .is_some()
        && card::is_location(d, sc, LOCATION_REMOVED)
        && card::is_monster(d, sc)
        && chain::effect_data(d, a.e, None, chain::field::CARD).truthy()
    {
        // local opp=1-tp
        let opp = 1 - tp;
        // local apply_banish_chk=sc:IsLevelBelow(4) and Duel.IsPlayerCanRemove(opp)
        //     and Duel.GetMatchingGroupCount(nil,opp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,nil)>0
        let apply_banish_chk = card::is_level_below(d, sc, 4)
            && duel::is_player_can_remove(d, opp, None)
            && duel::get_matching_group_count(d, None, opp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, NO_EX) > 0;
        // local race,attr,lv=sc:GetRace(),sc:GetAttribute(),sc:GetLevel()
        let race = card::get_race(d, sc);
        let attr = card::get_attribute(d, sc);
        let lv = card::get_level(d, sc);
        // local apply_search_chk=sc:IsLevelAbove(5) and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil,race,attr,lv)
        let apply_search_chk = card::is_level_above(d, sc, 5)
            && duel::is_existing_matching_card(d, Some(&|d, x| thfilter(d, x, race, attr, lv)), tp, LOCATION_DECK, 0, 1, NO_EX);
        // if not (apply_banish_chk or apply_search_chk) then return end
        if !(apply_banish_chk || apply_search_chk) {
            return;
        }
        // local hint_offset=apply_banish_chk and 1 or 2
        let hint_offset = if apply_banish_chk { 1 } else { 2 };
        // if not Duel.SelectYesNo(tp,aux.Stringid(id,hint_offset)) then return end
        if !duel::select_yes_no(d, tp, aux::stringid(ID, hint_offset)) {
            return;
        }
        if apply_banish_chk {
            // local g=Duel.GetMatchingGroup(s.banfilter,opp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,nil,sc:GetCode(),opp)
            let code = card::get_code(d, sc)[0];
            let g = duel::get_matching_group(
                d,
                Some(&|d, x| banfilter(d, x, code, opp)),
                opp,
                LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE,
                0,
                NO_EX,
            );
            // if #g>0 then
            if !g.is_empty() {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Remove(g,POS_FACEUP,REASON_EFFECT,nil,opp)
                duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, Some(opp));
            }
        } else if apply_search_chk {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
            // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil,race,attr,lv)
            let g = duel::select_matching_card(
                d,
                tp,
                Some(&|d, x| thfilter(d, x, race, attr, lv)),
                tp,
                LOCATION_DECK,
                0,
                1,
                1,
                false,
                NO_EX,
            )
            .nil();
            // if #g>0 then
            if !g.is_empty() {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.SendtoHand(g,nil,REASON_EFFECT)
                duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
                // Duel.ConfirmCards(opp,g)
                duel::confirm_cards(d, opp, g);
            }
        }
    }
}
