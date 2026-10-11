//! H-C Night Watchman
//! Heroic Challenger - Night Watchman
use super::prelude::*;

pub const ID: u32 = 61132951;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // destroy
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_BATTLE_CONFIRM);
    effect::set_target(d, e1, Some(cost_target(destg)));
    effect::set_operation(d, e1, Some(operation(desop)));
    card::register_effect(d, c, e1, false);
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let dc = duel::get_attack_target(d);
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::get_attacker(d) == Some(h)
            && dc.is_some_and(|x| card::is_defense_pos(d, x) && card::is_relate_to_battle(d, x));
    }
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, dc.map(|x| x.into()), 1, 0, 0);
    true
}
fn desop(d: &mut Duel, _a: &EffArgs) {
    let dc = duel::get_attack_target(d);
    if let Some(dc) = dc {
        if card::is_relate_to_battle(d, dc) && card::is_defense_pos(d, dc) {
            duel::destroy(d, dc, REASON_EFFECT, None, None);
        }
    }
}
