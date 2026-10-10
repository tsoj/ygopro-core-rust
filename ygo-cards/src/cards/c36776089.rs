//! Sky Cavalry Centaurea
use super::prelude::*;

pub const ID: u32 = 36776089;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    xyz::add_procedure(d, c, None, Some(2), 2, None, None, None, None, false, None);
    card::enable_revive_limit(d, c);
    // indestructable
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_condition(d, e1, Some(lua_fn(incon)));
    effect::set_value(d, e1, 1);
    card::register_effect(d, c, e1, false);
    // return
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_DAMAGE_STEP_END);
    effect::set_condition(d, e2, Some(condition(retcon)));
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    effect::set_target(d, e2, Some(cost_target(rettg)));
    effect::set_operation(d, e2, Some(operation(retop)));
    card::register_effect(d, c, e2, false);
}
/// `s.incon(e)`
fn incon(d: &mut Duel, a: Args) -> bool {
    let c = effect::get_handler(d, a.e(0)).nil();
    card::get_overlay_count(d, c) > 0
}
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    match card::get_battle_target(d, c) {
        None => false,
        Some(bc) => card::is_relate_to_battle(d, c) && card::is_relate_to_battle(d, bc),
    }
}
fn rettg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let c = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, c).map(CardSet::from);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, bc, 1, 0, 0);
    true
}
fn retop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, c).nil();
    if card::is_relate_to_battle(d, bc) {
        duel::send_to_hand(d, bc, None, REASON_EFFECT, None);
    }
}
