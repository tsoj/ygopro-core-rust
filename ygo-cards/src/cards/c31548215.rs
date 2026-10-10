//! Gravedigger's Trap Hole
use super::prelude::*;

pub const ID: u32 = 31548215;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // negate effect + damage
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DISABLE + CATEGORY_DAMAGE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_condition(d, e1, Some(condition(negcon)));
    effect::set_target(d, e1, Some(cost_target(negtg)));
    effect::set_operation(d, e1, Some(operation(negop)));
    card::register_effect(d, c, e1, false);
}
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    let activate_location = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION])[0].int();
    let h = effect::get_handler(d, a.e).nil();
    !card::is_status(d, h, STATUS_BATTLE_DESTROYED)
        && a.ep != a.tp
        && effect::is_monster_effect(d, a.re.nil())
        && duel::is_chain_disablable(d, a.ev as u8)
        && (activate_location == LOCATION_GRAVE as i64
            || activate_location == LOCATION_HAND as i64
            || activate_location == LOCATION_REMOVED as i64)
}
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let rh = effect::get_handler(d, a.re.nil()).nil();
        return !card::is_status(d, rh, STATUS_DISABLED);
    }
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    duel::set_target_player(d, 1 - a.tp);
    duel::set_target_param(d, 2000);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 2000);
    true
}
fn negop(d: &mut Duel, a: &EffArgs) {
    if duel::negate_effect(d, a.ev as u8) {
        duel::damage(d, 1 - a.tp, 2000, REASON_EFFECT, false, None);
    }
}
