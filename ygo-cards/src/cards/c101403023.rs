//! The Secret Crane Princess Who Returns A Favor
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 101403023;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // During your opponent's End Phase: You can place this card from your GY on top of the Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e1, Some(condition(|d, a| duel::is_turn_player(d, 1 - a.tp))));
    // e1:SetTarget(s.tdtg)
    effect::set_target(d, e1, Some(cost_target(tdtg)));
    // e1:SetOperation(s.tdop)
    effect::set_operation(d, e1, Some(operation(tdop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // During the Main Phase (Quick Effect): You can return this card from the field to the hand, and if you do, each player draws 1 card, then your opponent can draw 1 card, and if they do, you must keep your hand revealed during this Main Phase
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_DRAW)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_DRAW);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return Duel.IsMainPhase() end)
    effect::set_condition(d, e2, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    // e2:SetTarget(s.rthdrawtg)
    effect::set_target(d, e2, Some(cost_target(rthdrawtg)));
    // e2:SetOperation(s.rthdrawop)
    effect::set_operation(d, e2, Some(operation(rthdrawop)));
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e2, 0, Some((TIMING_MAIN_END | TIMINGS_CHECK_MONSTER) as u32));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // (Quick Effect): You can banish this revealed card from your hand; destroy all face-up cards your opponent controls
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_HAND)
    effect::set_range(d, e3, LOCATION_HAND);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return e:GetHandler():IsPublic() end)
    effect::set_condition(
        d,
        e3,
        Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            card::is_public(d, h)
        })),
    );
    // e3:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e3, Some(cost::self_banish()));
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // e3:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e3, 0, Some((TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E) as u32));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToDeck() end
    if a.chk == Some(0) {
        return card::is_able_to_deck(d, c);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
// function s.tdop(e,tp,eg,ep,ev,re,r,rp)
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SendtoDeck(c,nil,SEQ_DECKTOP,REASON_EFFECT)>0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::send_to_deck(d, CardSet::from_card(c), None, SEQ_DECKTOP as i32, REASON_EFFECT, None) > 0 {
        // Duel.ConfirmDecktop(tp,1)
        duel::confirm_decktop(d, a.tp, 1);
    }
}
// function s.rthdrawtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn rthdrawtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHand() and Duel.IsPlayerCanDraw(tp,1)
    //     and Duel.IsPlayerCanDraw(1-tp,1) end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None) && duel::is_player_can_draw(d, a.tp, 1) && duel::is_player_can_draw(d, 1 - a.tp, 1);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,PLAYER_ALL,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, PLAYER_ALL as u8, 1);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DRAW,nil,0,1-tp,1)
    duel::set_possible_operation_info(d, 0, CATEGORY_DRAW, None, 0, 1 - a.tp, 1);
    true
}
// function s.rthdrawop(e,tp,eg,ep,ev,re,r,rp)
fn rthdrawop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SendtoHand(c,nil,REASON_EFFECT)>0 and c:IsLocation(LOCATION_HAND) then
    if card::is_relate_to_effect(d, c, a.e) && duel::send_to_hand(d, CardSet::from_card(c), None, REASON_EFFECT, None) > 0 && card::is_location(d, c, LOCATION_HAND) {
        // Duel.ShuffleHand(c:GetControler())
        let controler = card::get_controler(d, c);
        duel::shuffle_hand(d, controler);
        // local opp=1-tp
        let opp = 1 - a.tp;
        // local turn_player=Duel.GetTurnPlayer()
        let turn_player = duel::get_turn_player(d);
        // if Duel.Draw(turn_player,1,REASON_EFFECT)>0 and Duel.Draw(1-turn_player,1,REASON_EFFECT)>0
        //     and Duel.IsPlayerCanDraw(opp,1) and Duel.SelectYesNo(opp,aux.Stringid(id,3)) then
        if duel::draw(d, turn_player, 1, REASON_EFFECT) > 0
            && duel::draw(d, 1 - turn_player, 1, REASON_EFFECT) > 0
            && duel::is_player_can_draw(d, opp, 1)
            && duel::select_yes_no(d, opp, aux::stringid(ID, 3))
        {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // if Duel.Draw(opp,1,REASON_EFFECT)==0 then return end
            if duel::draw(d, opp, 1, REASON_EFFECT) == 0 {
                return;
            }
            // local reset_phase=Duel.IsMainPhase1() and PHASE_MAIN1 or PHASE_MAIN2
            let reset_phase = if duel::is_main_phase1(d, None) { PHASE_MAIN1 } else { PHASE_MAIN2 };
            // Then your opponent can draw 1 card, and if they do, you must keep your hand revealed during this Main Phase
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_FIELD)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD);
            // e1:SetCode(EFFECT_PUBLIC)
            effect::set_code(d, e1, EFFECT_PUBLIC);
            // e1:SetTargetRange(LOCATION_HAND,0)
            effect::set_target_range(d, e1, LOCATION_HAND, 0);
            // e1:SetReset(RESET_PHASE|reset_phase)
            effect::set_reset(d, e1, RESET_PHASE | reset_phase, 1);
            // Duel.RegisterEffect(e1,tp)
            duel::register_effect(d, e1, a.tp);
        }
    }
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, a.tp, 0);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
