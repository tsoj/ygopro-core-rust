//! Silent Burning
use super::prelude::*;

pub const ID: u32 = 44968459;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SILENT_MAGICIAN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Each player draws until they have 6 cards in their hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CANNOT_NEGATE+EFFECT_FLAG_CANNOT_INACTIVATE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CANNOT_NEGATE + EFFECT_FLAG_CANNOT_INACTIVATE, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetHintTiming(TIMING_BATTLE_PHASE,TIMING_BATTLE_START|TIMING_BATTLE_PHASE|TIMING_BATTLE_STEP_END|TIMING_BATTLE_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, TIMING_BATTLE_PHASE, Some(TIMING_BATTLE_START | TIMING_BATTLE_PHASE | TIMING_BATTLE_STEP_END | TIMING_BATTLE_END | TIMINGS_CHECK_MONSTER));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add 1 "Silent Magician" monster from your Deck to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.condition(e,tp,...)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsBattlePhase() and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_SILENT_MAGICIAN),tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)>Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)
    duel::is_battle_phase(d, None)
        && duel::is_existing_matching_card(
            d,
            Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_SILENT_MAGICIAN as u16])),
            a.tp,
            LOCATION_MZONE,
            0,
            1,
            NO_EX,
        )
        && duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0) > duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND)
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local ct1=6-Duel.GetMatchingGroupCount(nil,tp,LOCATION_HAND,0,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let ct1 = 6 - duel::get_matching_group_count(d, None, a.tp, LOCATION_HAND, 0, h.into()) as i64;
    // local ct2=6-Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)
    let ct2 = 6 - duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) as i64;
    // if chk==0 then return ct1>0 and Duel.IsPlayerCanDraw(tp,ct1)
    //     and ct2>0 and Duel.IsPlayerCanDraw(1-tp,ct2) end
    if a.chk == Some(0) {
        return ct1 > 0 && duel::is_player_can_draw(d, a.tp, ct1 as u32) && ct2 > 0 && duel::is_player_can_draw(d, 1 - a.tp, ct2 as u32);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,PLAYER_ALL,ct1+ct2)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, PLAYER_ALL as u8, (ct1 + ct2) as i32);
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local turn_player=Duel.GetTurnPlayer()
    let turn_player = duel::get_turn_player(d);
    // local ct1=6-Duel.GetFieldGroupCount(turn_player,LOCATION_HAND,0)
    let ct1 = 6 - duel::get_field_group_count(d, turn_player, LOCATION_HAND, 0);
    // local ct2=6-Duel.GetFieldGroupCount(turn_player,0,LOCATION_HAND)
    let ct2 = 6 - duel::get_field_group_count(d, turn_player, 0, LOCATION_HAND);
    // if ct1>0 then
    if ct1 > 0 {
        // Duel.Draw(turn_player,ct1,REASON_EFFECT)
        duel::draw(d, turn_player, ct1 as u32, REASON_EFFECT);
    }
    // if ct2>0 then
    if ct2 > 0 {
        // Duel.Draw(1-turn_player,ct2,REASON_EFFECT)
        duel::draw(d, 1 - turn_player, ct2 as u32, REASON_EFFECT);
    }
    let _ = a;
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SILENT_MAGICIAN) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_SILENT_MAGICIAN as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,...,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,...)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
