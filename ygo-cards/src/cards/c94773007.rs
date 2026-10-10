//! Jirai Gumo
use super::prelude::*;

pub const ID: u32 = 94773007;

pub const INFO: ScriptInfo = ScriptInfo { toss_coin: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Toss a coin and halve the player's LP
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_COIN);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    effect::set_target(d, e1, Some(cost_target(attg)));
    effect::set_operation(d, e1, Some(operation(atop)));
    card::register_effect(d, c, e1, false);
}
fn attg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_COIN, None, 0, a.tp, 1);
    true
}
fn atop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if !duel::call_coin(d, tp) {
        let lp = duel::get_lp(d, tp) as f64;
        duel::set_lp(d, tp, (lp / 2.0).ceil() as i32);
    }
}
