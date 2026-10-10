//! Confiscation
use super::prelude::*;

pub const ID: u32 = 17375316;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_HANDES);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(1000))));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) > 0;
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,1-tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, 1 - a.tp, 1);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, _a: &EffArgs) {
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER]);
    let p = info[0].int() as u8;
    // local g=Duel.GetFieldGroup(p,0,LOCATION_HAND)
    let g = duel::get_field_group(d, p, 0, LOCATION_HAND);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.ConfirmCards(p,g)
        duel::confirm_cards(d, p, g.clone());
        // Duel.Hint(HINT_SELECTMSG,p,HINTMSG_DISCARD)
        duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_DISCARD as u64);
        // local sg=g:Select(p,1,1,nil)
        let sg = group::select(d, &g, p, 1, 1, false, NO_EX).nil();
        // Duel.SendtoGrave(sg,REASON_EFFECT|REASON_DISCARD)
        duel::send_to_grave(d, sg, REASON_EFFECT | REASON_DISCARD, None, None);
        // Duel.ShuffleHand(1-p)
        duel::shuffle_hand(d, 1 - p);
    }
}
