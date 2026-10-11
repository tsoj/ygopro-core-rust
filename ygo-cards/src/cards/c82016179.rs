//! Sylvan Charity
use super::prelude::*;

pub const ID: u32 = 82016179;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SYLVAN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DRAW+CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_DRAW + CATEGORY_TODECK);
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
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,3) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 3);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(3)
    duel::set_target_param(d, 3);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,3)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 3);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,0,tp,2)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 0, a.tp, 2);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let cnt = info[1].int() as u32;
    // if Duel.Draw(p,d,REASON_EFFECT)==3 then
    if duel::draw(d, p, cnt, REASON_EFFECT) == 3 {
        // Duel.ShuffleHand(p)
        duel::shuffle_hand(d, p);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // local g=Duel.GetMatchingGroup(Card.IsAbleToDeck,p,LOCATION_HAND,0,nil)
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_deck(d, c)), p, LOCATION_HAND, 0, NO_EX);
        // if #g>1 and g:IsExists(Card.IsSetCard,1,nil,SET_SYLVAN) then
        if g.len() > 1 && group::is_exists(d, &g, &|d, c| card::is_set_card(d, c, &[SET_SYLVAN as u16]), 1, NO_EX) {
            // Duel.Hint(HINT_SELECTMSG,p,HINTMSG_TODECK)
            duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_TODECK as u64);
            // local sg1=g:FilterSelect(p,Card.IsSetCard,1,1,nil,SET_SYLVAN)
            let mut sg1 = group::filter_select(d, &g, p, &|d, c| card::is_set_card(d, c, &[SET_SYLVAN as u16]), 1, 1, false, NO_EX).nil();
            // Duel.Hint(HINT_SELECTMSG,p,HINTMSG_TODECK)
            duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_TODECK as u64);
            // local sg2=g:Select(p,1,1,sg1:GetFirst())
            let first = group::get_first(d, &sg1).nil();
            let sg2 = group::select(d, &g, p, 1, 1, false, first.into()).nil();
            // sg1:Merge(sg2)
            group::add_card(&mut sg1, &sg2);
            // Duel.ConfirmCards(1-p,sg1)
            duel::confirm_cards(d, 1 - p, sg1.clone());
            // Duel.SendtoDeck(sg1,nil,SEQ_DECKTOP,REASON_EFFECT)
            duel::send_to_deck(d, sg1, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
            // Duel.SortDecktop(p,p,2)
            duel::sort_decktop(d, p, p, 2);
        } else {
            // local hg=Duel.GetFieldGroup(p,LOCATION_HAND,0)
            let hg = duel::get_field_group(d, p, LOCATION_HAND, 0);
            // Duel.ConfirmCards(1-p,hg)
            duel::confirm_cards(d, 1 - p, hg.clone());
            // local ct=Duel.SendtoDeck(hg,nil,SEQ_DECKTOP,REASON_EFFECT)
            let ct = duel::send_to_deck(d, hg, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
            // Duel.SortDecktop(p,p,ct)
            duel::sort_decktop(d, p, p, ct as u16);
        }
    }
}
