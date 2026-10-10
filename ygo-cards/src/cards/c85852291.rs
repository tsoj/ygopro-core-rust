//! Magical Mallet
use super::prelude::*;

pub const ID: u32 = 85852291;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp) and Duel.IsExistingMatchingCard(Card.IsAbleToDeck,tp,LOCATION_HAND,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        if !duel::is_player_can_draw(d, a.tp, 1) {
            return false;
        }
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&card::is_able_to_deck), a.tp, LOCATION_HAND, 0, 1, h.into());
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
fn activate(d: &mut Duel, _a: &EffArgs) {
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER]);
    let p = info[0].int() as u8;
    // Duel.Hint(HINT_SELECTMSG,p,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(p,Card.IsAbleToDeck,p,LOCATION_HAND,0,1,63,nil)
    let g = duel::select_matching_card(d, p, Some(&card::is_able_to_deck), p, LOCATION_HAND, 0, 1, 63, false, NO_EX).nil();
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
    duel::send_to_deck(d, g.clone(), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    // Duel.ShuffleDeck(p)
    duel::shuffle_deck(d, p);
    // Duel.BreakEffect()
    duel::break_effect(d);
    // Duel.Draw(p,#g,REASON_EFFECT)
    duel::draw(d, p, g.len() as u32, REASON_EFFECT);
}
