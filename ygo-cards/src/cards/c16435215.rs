//! Dragged Down into the Grave
use super::prelude::*;

pub const ID: u32 = 16435215;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOGRAVE+CATEGORY_HANDES+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_HANDES + CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)<=0 then return false end
    if duel::get_field_group_count(d, tp, 0, LOCATION_HAND) <= 0 {
        return false;
    }
    // local ct=0
    let mut ct = 0;
    // if e:GetHandler():IsLocation(LOCATION_HAND) then ct=1 end
    let h = effect::get_handler(d, a.e).nil();
    if card::is_location(d, h, LOCATION_HAND) {
        ct = 1;
    }
    // return Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)>ct
    duel::get_field_group_count(d, tp, LOCATION_HAND, 0) > ct
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) and Duel.IsPlayerCanDraw(1-tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1) && duel::is_player_can_draw(d, 1 - a.tp, 1);
    }
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,PLAYER_ALL,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, PLAYER_ALL as u8, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,PLAYER_ALL,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, PLAYER_ALL as u8, 1);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)==0 or Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)==0 then return end
    if duel::get_field_group_count(d, tp, 0, LOCATION_HAND) == 0 || duel::get_field_group_count(d, tp, LOCATION_HAND, 0) == 0 {
        return;
    }
    // local g1=Duel.GetFieldGroup(tp,0,LOCATION_HAND)
    let g1 = duel::get_field_group(d, tp, 0, LOCATION_HAND);
    // local g2=Duel.GetFieldGroup(tp,LOCATION_HAND,0)
    let g2 = duel::get_field_group(d, tp, LOCATION_HAND, 0);
    // Duel.ConfirmCards(tp,g1)
    duel::confirm_cards(d, tp, &g1);
    // Duel.ConfirmCards(1-tp,g2)
    duel::confirm_cards(d, 1 - tp, &g2);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DISCARD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DISCARD as u64);
    // local sg1=g1:Select(tp,1,1,nil)
    let mut sg1 = group::select(d, &g1, tp, 1, 1, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_DISCARD)
    duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_DISCARD as u64);
    // local sg2=g2:Select(1-tp,1,1,nil)
    let sg2 = group::select(d, &g2, 1 - tp, 1, 1, false, NO_EX).nil();
    // sg1:Merge(sg2)
    group::add_card(&mut sg1, &sg2);
    // Duel.SendtoGrave(sg1,REASON_EFFECT|REASON_DISCARD)
    duel::send_to_grave(d, sg1, REASON_EFFECT | REASON_DISCARD, None, None);
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, tp);
    // Duel.ShuffleHand(1-tp)
    duel::shuffle_hand(d, 1 - tp);
    // Duel.BreakEffect()
    duel::break_effect(d);
    // Duel.Draw(tp,1,REASON_EFFECT)
    duel::draw(d, tp, 1, REASON_EFFECT);
    // Duel.Draw(1-tp,1,REASON_EFFECT)
    duel::draw(d, 1 - tp, 1, REASON_EFFECT);
}
