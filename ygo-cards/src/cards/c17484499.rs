//! Exchange of the Spirit
use super::prelude::*;

pub const ID: u32 = 17484499;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Each player swaps the cards in their GY with the cards in their Deck, then shuffles their Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_DUEL)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_DUEL as u8);
    // e1:SetCondition(function(e,tp) return Duel.GetFieldGroupCount(tp,LOCATION_GRAVE,0)>=15
    //     and Duel.GetFieldGroupCount(tp,0,LOCATION_GRAVE)>=15 end)
    effect::set_condition(d, e1, Some(condition(e1_condition)));
    // e1:SetCost(Cost.PayLP(1000))
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(1000))));
    // e1:SetOperation(function(e,tp,eg,ep,ev,re,r,rp)
    //     local turn_player=Duel.GetTurnPlayer()
    //     Duel.SwapDeckAndGrave(turn_player)
    //     Duel.SwapDeckAndGrave(1-turn_player)
    // end)
    effect::set_operation(d, e1, Some(operation(e1_operation)));
    // e1:SetHintTiming(0,TIMING_DRAW_PHASE|TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMING_BATTLE_START|TIMING_BATTLE_END|TIMINGS_CHECK_MONSTER_E|TIMING_CHAIN_END)
    effect::set_hint_timing(
        d,
        e1,
        0,
        Some(TIMING_DRAW_PHASE | TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMING_BATTLE_START | TIMING_BATTLE_END | TIMINGS_CHECK_MONSTER_E | TIMING_CHAIN_END),
    );
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `function(e,tp) return Duel.GetFieldGroupCount(tp,LOCATION_GRAVE,0)>=15 and Duel.GetFieldGroupCount(tp,0,LOCATION_GRAVE)>=15 end`
fn e1_condition(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_field_group_count(d, a.tp, LOCATION_GRAVE, 0) >= 15 && duel::get_field_group_count(d, a.tp, 0, LOCATION_GRAVE) >= 15
}
/// The operation of e1
fn e1_operation(d: &mut Duel, _a: &EffArgs) {
    // local turn_player=Duel.GetTurnPlayer()
    let turn_player = duel::get_turn_player(d);
    // Duel.SwapDeckAndGrave(turn_player)
    duel::swap_deck_and_grave(d, turn_player);
    // Duel.SwapDeckAndGrave(1-turn_player)
    duel::swap_deck_and_grave(d, 1 - turn_player);
}
