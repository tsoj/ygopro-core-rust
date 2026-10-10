//! Cup of Ace
use super::prelude::*;

pub const ID: u32 = 37812118;
pub const INFO: ScriptInfo = ScriptInfo { toss_coin: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_COIN + CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,2) or Duel.IsPlayerCanDraw(1-tp,2) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 2) || duel::is_player_can_draw(d, 1 - a.tp, 2);
    }
    // Duel.SetOperationInfo(0,CATEGORY_COIN,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_COIN, None, 0, a.tp, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,PLAYER_EITHER,2)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, PLAYER_EITHER as u8, 2);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local coin=Duel.TossCoin(tp,1)
    let coin = duel::toss_coin(d, a.tp, 1)[0];
    // local player=(coin==COIN_HEADS and tp) or (coin==COIN_TAILS and 1-tp) or nil
    let player = if coin == COIN_HEADS as i32 {
        Some(a.tp)
    } else if coin == COIN_TAILS as i32 {
        Some(1 - a.tp)
    } else {
        None
    };
    // if player then Duel.Draw(player,2,REASON_EFFECT) end
    if let Some(player) = player {
        duel::draw(d, player, 2, REASON_EFFECT);
    }
}
