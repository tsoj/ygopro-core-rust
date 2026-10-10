//! Goblin Thief
use super::prelude::*;

pub const ID: u32 = 45311864;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // recover&damage
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_category(d, e1, CATEGORY_RECOVER + CATEGORY_DAMAGE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,500)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, 500);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,500)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 500);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    // Duel.Damage(1-tp,500,REASON_EFFECT,true)
    duel::damage(d, 1 - a.tp, 500, REASON_EFFECT, true, None);
    // Duel.Recover(tp,500,REASON_EFFECT,true)
    duel::recover(d, a.tp, 500, REASON_EFFECT, true, None);
    // Duel.RDComplete()
    duel::rd_complete(d);
}
