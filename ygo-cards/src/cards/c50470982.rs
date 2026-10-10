//! The Paths of Destiny
use super::prelude::*;

pub const ID: u32 = 50470982;
pub const INFO: ScriptInfo = ScriptInfo { toss_coin: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_COIN)
    effect::set_category(d, e1, CATEGORY_COIN);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
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
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_COIN,nil,0,PLAYER_ALL,2)
    duel::set_operation_info(d, 0, CATEGORY_COIN, None, 0, PLAYER_ALL as u8, 2);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local res=Duel.TossCoin(tp,1)
    let res = duel::toss_coin(d, a.tp, 1)[0];
    // if res==COIN_HEADS then Duel.Recover(tp,2000,REASON_EFFECT)
    if res == COIN_HEADS as i32 {
        duel::recover(d, a.tp, 2000, REASON_EFFECT, false, None);
    // elseif res==COIN_TAILS then Duel.Damage(tp,2000,REASON_EFFECT) end
    } else if res == COIN_TAILS as i32 {
        duel::damage(d, a.tp, 2000, REASON_EFFECT, false, None);
    }
    // res=Duel.TossCoin(1-tp,1)
    let res = duel::toss_coin(d, 1 - a.tp, 1)[0];
    // if res==COIN_HEADS then Duel.Recover(1-tp,2000,REASON_EFFECT)
    if res == COIN_HEADS as i32 {
        duel::recover(d, 1 - a.tp, 2000, REASON_EFFECT, false, None);
    // elseif res==COIN_TAILS then Duel.Damage(1-tp,2000,REASON_EFFECT) end
    } else if res == COIN_TAILS as i32 {
        duel::damage(d, 1 - a.tp, 2000, REASON_EFFECT, false, None);
    }
}
