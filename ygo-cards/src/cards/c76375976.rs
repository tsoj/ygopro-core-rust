//! Mystic Mine
use super::prelude::*;

pub const ID: u32 = 76375976;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Cannot activate effects
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_target_range(d, e2, 1, 0);
    effect::set_condition(d, e2, Some(lua_fn(conself)));
    effect::set_value_fn(d, e2, lua_fn(aclimit));
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_target_range(d, e3, 0, 1);
    effect::set_condition(d, e3, Some(lua_fn(conopp)));
    card::register_effect(d, c, e3, false);
    // Cannot declare attack
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    effect::set_code(d, e4, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_range(d, e4, LOCATION_FZONE);
    effect::set_target_range(d, e4, 1, 0);
    effect::set_condition(d, e4, Some(lua_fn(conself)));
    card::register_effect(d, c, e4, false);
    let e5 = effect::clone(d, e4);
    effect::set_target_range(d, e5, 0, 1);
    effect::set_condition(d, e5, Some(lua_fn(conopp)));
    card::register_effect(d, c, e5, false);
    // Destroy itself
    let e6 = effect::create_effect(d, c);
    effect::set_category(d, e6, CATEGORY_DESTROY);
    effect::set_type(d, e6, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e6, EVENT_PHASE + PHASE_END);
    effect::set_range(d, e6, LOCATION_FZONE);
    effect::set_count_limit(d, e6, 1, 0, 0, 0);
    effect::set_condition(d, e6, Some(condition(descon)));
    effect::set_target(d, e6, Some(cost_target(destg)));
    effect::set_operation(d, e6, Some(operation(desop)));
    card::register_effect(d, c, e6, false);
}
/// `s.conself(e)`: `tp=e:GetHandlerPlayer()`, more monsters for `tp` than for the opponent
fn conself(d: &mut Duel, a: Args) -> bool {
    let tp = effect::get_handler_player(d, a.e(0));
    duel::get_field_group_count(d, tp, LOCATION_MZONE, 0) > duel::get_field_group_count(d, tp, 0, LOCATION_MZONE)
}
/// `s.conopp(e)`: `tp=e:GetHandlerPlayer()`, fewer monsters for `tp` than for the opponent
fn conopp(d: &mut Duel, a: Args) -> bool {
    let tp = effect::get_handler_player(d, a.e(0));
    duel::get_field_group_count(d, tp, LOCATION_MZONE, 0) < duel::get_field_group_count(d, tp, 0, LOCATION_MZONE)
}
/// `s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    effect::is_monster_effect(d, a.e(1))
}
/// `s.descon(e,tp,...)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    duel::get_field_group_count(d, tp, LOCATION_MZONE, 0) == duel::get_field_group_count(d, tp, 0, LOCATION_MZONE)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::destroy(d, c, REASON_EFFECT, None, None);
    }
}
