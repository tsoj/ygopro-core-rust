//! Tremendous Fire
use super::prelude::*;

pub const ID: u32 = 46918794;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,PLAYER_ALL,500)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, PLAYER_ALL as u8, 500);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.Damage(1-tp,1000,REASON_EFFECT,true)
    duel::damage(d, 1 - a.tp, 1000, REASON_EFFECT, true, None);
    // Duel.Damage(tp,500,REASON_EFFECT,true)
    duel::damage(d, a.tp, 500, REASON_EFFECT, true, None);
    // Duel.RDComplete()
    duel::rd_complete(d);
}
