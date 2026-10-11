//! Battlin' Boxer Rabbit Puncher
use super::prelude::*;

pub const ID: u32 = 61679541;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // destroy
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_BATTLE_START);
    effect::set_target(d, e1, Some(cost_target(targ)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn targ(d: &mut Duel, a: &EffArgs) -> bool {
    let dt = duel::get_attack_target(d);
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        if duel::get_attacker(d) != Some(h) {
            return false;
        }
        return match dt {
            Some(x) => card::is_defense_pos(d, x) && card::is_relate_to_battle(d, x),
            None => false,
        };
    }
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, dt.map(CardSet::from_card), 1, 0, 0);
    true
}
fn op(d: &mut Duel, _a: &EffArgs) {
    let dt = duel::get_attack_target(d);
    if let Some(x) = dt {
        if card::is_relate_to_battle(d, x) && card::is_defense_pos(d, x) {
            duel::destroy(d, x, REASON_EFFECT, None, None);
        }
    }
}
