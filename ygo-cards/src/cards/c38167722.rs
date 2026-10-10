//! Dark Spirit Art - Greed
use super::prelude::*;

pub const ID: u32 = 38167722;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn is_dark(d: &mut Duel, c: CardId) -> bool {
    card::is_attribute(d, c, ATTRIBUTE_DARK)
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,Card.IsAttribute,1,false,nil,nil,ATTRIBUTE_DARK) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&is_dark), 1, 1, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,Card.IsAttribute,1,1,false,nil,nil,ATTRIBUTE_DARK)
    let g = duel::select_release_group_cost(d, a.tp, Some(&is_dark), 1, 1, false, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,2) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 2);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(2)
    duel::set_target_param(d, 2);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,2)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 2);
    true
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return not c:IsPublic() and c:IsSpell()
    !card::is_public(d, c) && card::is_spell(d, c)
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let count = info[1].int() as u32;
    // if Duel.IsChainDisablable(0) then
    if duel::is_chain_disablable(d, 0) {
        // local g=Duel.GetMatchingGroup(s.cfilter,p,0,LOCATION_HAND,nil)
        let g = duel::get_matching_group(d, Some(&cfilter), p, 0, LOCATION_HAND, NO_EX);
        // if #g>0 and Duel.SelectYesNo(1-tp,aux.Stringid(id,0)) then
        if !g.is_empty() && duel::select_yes_no(d, 1 - tp, aux::stringid(ID, 0)) {
            // Duel.Hint(HINT_SELECTMSG,1-p,HINTMSG_CONFIRM)
            duel::hint(d, HINT_SELECTMSG as u8, 1 - p, HINTMSG_CONFIRM as u64);
            // local sg=g:Select(1-p,1,1,nil)
            let sg = group::select(d, &g, 1 - p, 1, 1, false, NO_EX).nil();
            // Duel.ConfirmCards(p,sg)
            duel::confirm_cards(d, p, &sg);
            // Duel.ShuffleHand(1-p)
            duel::shuffle_hand(d, 1 - p);
            // Duel.NegateEffect(0)
            duel::negate_effect(d, 0);
            return;
        }
    }
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, p, count, REASON_EFFECT);
}
