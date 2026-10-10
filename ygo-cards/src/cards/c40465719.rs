//! Common Charity
use super::prelude::*;

pub const ID: u32 = 40465719;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DRAW+CATEGORY_REMOVE+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_DRAW + CATEGORY_REMOVE + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return not Duel.IsPlayerAffectedByEffect(tp,30459350) and Duel.IsPlayerCanDraw(tp,2) end
    if a.chk == Some(0) {
        return duel::is_player_affected_by_effect(d, tp, 30459350).is_none() && duel::is_player_can_draw(d, tp, 2);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, tp);
    // Duel.SetTargetParam(2)
    duel::set_target_param(d, 2);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,2)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 2);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, p, info[1].int() as u32, REASON_EFFECT);
    // Duel.ShuffleHand(p)
    duel::shuffle_hand(d, p);
    // Duel.BreakEffect()
    duel::break_effect(d);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(p,Card.IsType,p,LOCATION_HAND,0,1,1,nil,TYPE_NORMAL)
    let g = duel::select_matching_card(d, p, Some(&|d, c| card::is_type(d, c, TYPE_NORMAL)), p, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // local tg=g:GetFirst()
    let tg = group::get_first(d, &g);
    // if tg then
    if let Some(tg) = tg {
        // if Duel.Remove(tg,POS_FACEUP,REASON_EFFECT)==0 then
        if duel::remove(d, CardSet::from_card(tg), POS_FACEUP as u8, REASON_EFFECT, None, None) == 0 {
            // Duel.ConfirmCards(1-p,tg)
            duel::confirm_cards(d, 1 - p, CardSet::from_card(tg));
            // Duel.ShuffleHand(p)
            duel::shuffle_hand(d, p);
        }
    } else {
        // local sg=Duel.GetFieldGroup(p,LOCATION_HAND,0)
        let sg = duel::get_field_group(d, p, LOCATION_HAND, 0);
        // Duel.SendtoGrave(sg,REASON_EFFECT)
        duel::send_to_grave(d, sg, REASON_EFFECT, None, None);
    }
}
