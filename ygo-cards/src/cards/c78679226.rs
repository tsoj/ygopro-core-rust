//! Future Silence
//! 未来への沈黙
use super::prelude::*;

pub const ID: u32 = 78679226;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_SHINING_SARCOPHAGUS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMING_BATTLE_START|TIMING_BATTLE_END|TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some((TIMING_BATTLE_START | TIMING_BATTLE_END | TIMING_END_PHASE) as u32));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:ListsCode(CARD_SHINING_SARCOPHAGUS) and c:IsAbleToHand()
    card::is_monster(d, c) && card::lists_code(d, c, &[CARD_SHINING_SARCOPHAGUS as u32]) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ct1=6-Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)
    let ct1 = 6 - duel::get_field_group_count(d, tp, LOCATION_HAND, 0);
    // local ct2=6-Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)
    let ct2 = 6 - duel::get_field_group_count(d, tp, 0, LOCATION_HAND);
    // local draw=Duel.IsBattlePhase() and e:IsHasType(EFFECT_TYPE_ACTIVATE)
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_SHINING_SARCOPHAGUS),tp,LOCATION_ONFIELD,0,1,nil)
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.ListsCode,CARD_SHINING_SARCOPHAGUS),tp,LOCATION_MZONE,0,1,nil)
    let draw = duel::is_battle_phase(d, None)
        && effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE)
        && duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[CARD_SHINING_SARCOPHAGUS as u32]))), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::lists_code(d, c, &[CARD_SHINING_SARCOPHAGUS as u32]))), tp, LOCATION_MZONE, 0, 1, NO_EX);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil)
    //     and (not draw or (ct1>0 and Duel.IsPlayerCanDraw(tp,ct1) and ct2>0 and Duel.IsPlayerCanDraw(1-tp,ct2))) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX)
            && (!draw || (ct1 > 0 && duel::is_player_can_draw(d, tp, ct1 as u32) && ct2 > 0 && duel::is_player_can_draw(d, 1 - tp, ct2 as u32)));
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // if draw then
    if draw {
        // e:SetLabel(1)
        effect::set_label(d, a.e, &[1]);
        // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,ct1)
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, ct1);
        // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,1-tp,ct2)
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, 1 - tp, ct2);
    } else {
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g==0 or Duel.SendtoHand(g,nil,REASON_EFFECT)==0 then return end
    if g.is_empty() || duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) == 0 {
        return;
    }
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - tp, g);
    // if e:GetLabel()==0 then return end
    if effect::get_label(d, a.e) == 0 {
        return;
    }
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, tp);
    // Duel.ShuffleDeck(tp)
    duel::shuffle_deck(d, tp);
    // local turn_p=Duel.GetTurnPlayer()
    let turn_p = duel::get_turn_player(d);
    // local ct1=6-Duel.GetFieldGroupCount(turn_p,LOCATION_HAND,0)
    let ct1 = 6 - duel::get_field_group_count(d, turn_p, LOCATION_HAND, 0);
    // local ct2=6-Duel.GetFieldGroupCount(turn_p,0,LOCATION_HAND)
    let ct2 = 6 - duel::get_field_group_count(d, turn_p, 0, LOCATION_HAND);
    // if ct1>0 or ct2>0 then Duel.BreakEffect() end
    if ct1 > 0 || ct2 > 0 {
        duel::break_effect(d);
    }
    // if ct1>0 then
    if ct1 > 0 {
        // Duel.Draw(turn_p,ct1,REASON_EFFECT)
        duel::draw(d, turn_p, ct1 as u32, REASON_EFFECT);
    }
    // if ct2>0 then
    if ct2 > 0 {
        // Duel.Draw(1-turn_p,ct2,REASON_EFFECT)
        duel::draw(d, 1 - turn_p, ct2 as u32, REASON_EFFECT);
    }
}
