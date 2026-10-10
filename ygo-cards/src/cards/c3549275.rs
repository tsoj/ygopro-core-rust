//! Dice Jar
use super::prelude::*;

pub const ID: u32 = 3549275;
pub const INFO: ScriptInfo = ScriptInfo { roll_dice: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Damage
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DICE)
    effect::set_category(d, e1, CATEGORY_DICE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_FLIP)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DICE,nil,0,PLAYER_ALL,1)
    duel::set_operation_info(d, 0, CATEGORY_DICE, None, 0, PLAYER_ALL as u8, 1);
    true
}
/// `s.operation(e,tp,...)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local d1=0
    let mut d1: i32 = 0;
    // local d2=0
    let mut d2: i32 = 0;
    // while d1==d2 do
    while d1 == d2 {
        // d1,d2=Duel.TossDice(tp,1,1)
        let r = duel::toss_dice(d, a.tp, 1, 1);
        d1 = r[0];
        d2 = r[1];
    }
    // if d1<d2 then
    if d1 < d2 {
        // if d2==6 then
        if d2 == 6 {
            // Duel.Damage(tp,6000,REASON_EFFECT)
            duel::damage(d, a.tp, 6000, REASON_EFFECT, false, None);
        } else {
            // Duel.Damage(tp,d2*500,REASON_EFFECT)
            duel::damage(d, a.tp, (d2 * 500) as i64, REASON_EFFECT, false, None);
        }
    } else {
        // if d1==6 then
        if d1 == 6 {
            // Duel.Damage(1-tp,6000,REASON_EFFECT)
            duel::damage(d, 1 - a.tp, 6000, REASON_EFFECT, false, None);
        } else {
            // Duel.Damage(1-tp,d1*500,REASON_EFFECT)
            duel::damage(d, 1 - a.tp, (d1 * 500) as i64, REASON_EFFECT, false, None);
        }
    }
}
