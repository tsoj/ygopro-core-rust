//! Danger! Zone
use super::prelude::*;

pub const ID: u32 = 15083304;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DANGER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DRAW+CATEGORY_HANDES)
    effect::set_category(d, e1, CATEGORY_DRAW + CATEGORY_HANDES);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
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
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,3) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 3);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, tp);
    // Duel.SetTargetParam(3)
    duel::set_target_param(d, 3);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,3)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 3);
    true
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:IsExists(Card.IsSetCard,1,nil,SET_DANGER)
    (group::is_exists(d, sg, &|d, c| card::is_set_card(d, c, &[SET_DANGER as u16]), 1, NO_EX), false)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let (p, dc) = (info[0].int() as u8, info[1].int() as u32);
    // if Duel.Draw(p,d,REASON_EFFECT)==3 then
    if duel::draw(d, p, dc, REASON_EFFECT) == 3 {
        // Duel.ShuffleHand(p)
        duel::shuffle_hand(d, p);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // local g=Duel.GetFieldGroup(p,LOCATION_HAND,0)
        let g = duel::get_field_group(d, p, LOCATION_HAND, 0);
        // if g:IsExists(Card.IsSetCard,1,nil,SET_DANGER) then
        if group::is_exists(d, &g, &|d, c| card::is_set_card(d, c, &[SET_DANGER as u16]), 1, NO_EX) {
            // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_DISCARD)
            let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_DISCARD as u64), None, None, false).unwrap_or_default();
            // Duel.SendtoGrave(sg,REASON_EFFECT|REASON_DISCARD)
            duel::send_to_grave(d, sg, REASON_EFFECT | REASON_DISCARD, None, None);
        } else {
            // Duel.ConfirmCards(1-p,g)
            duel::confirm_cards(d, 1 - p, g.clone());
            // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
            duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        }
    }
}
