//! Infernity Destroyer
use super::prelude::*;

pub const ID: u32 = 98954375;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // damage
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_condition(d, e1, Some(condition(damcon)));
    effect::set_target(d, e1, Some(cost_target(damtg)));
    effect::set_operation(d, e1, Some(operation(damop)));
    card::register_effect(d, c, e1, false);
}
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let mut dd = duel::get_attacker(d);
    if dd == Some(c) {
        dd = duel::get_attack_target(d);
    }
    card::is_relate_to_battle(d, c)
        && duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0) == 0
        && {
            let dd = dd.nil();
            card::is_location(d, dd, LOCATION_GRAVE) && card::is_reason(d, dd, REASON_BATTLE) && card::is_monster(d, dd)
        }
}
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_target_player(d, 1 - a.tp);
    duel::set_target_param(d, 1600);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 1600);
    true
}
fn damop(d: &mut Duel, a: &EffArgs) {
    if duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0) > 0 {
        return;
    }
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
