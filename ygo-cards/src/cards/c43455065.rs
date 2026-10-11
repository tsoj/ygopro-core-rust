//! Magical Spring
use super::prelude::*;

pub const ID: u32 = 43455065;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_spell_trap(d, c)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let ct = duel::get_matching_group_count(d, Some(&filter), a.tp, 0, LOCATION_ONFIELD, NO_EX) as i64;
    if a.chk == Some(0) {
        return ct > 0 && duel::is_player_can_draw(d, a.tp, ct as u32);
    }
    duel::set_target_player(d, a.tp);
    duel::set_target_param(d, ct as u32);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, ct as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    let mut ct = duel::get_matching_group_count(d, Some(&filter), a.tp, 0, LOCATION_ONFIELD, NO_EX) as i64;
    if duel::draw(d, p, ct as u32, REASON_EFFECT) != 0 {
        ct = duel::get_matching_group_count(d, Some(&filter), a.tp, LOCATION_ONFIELD, 0, NO_EX) as i64;
        if ct > 0 {
            duel::break_effect(d);
            duel::discard_hand(d, a.tp, None, ct as u16, ct as u16, REASON_EFFECT | REASON_DISCARD, NO_EX);
        }
    }
    let mut rct: u16 = 1;
    if duel::is_turn_player(d, 1 - a.tp) {
        rct = 2;
    }
    let c = effect::get_handler(d, a.e).nil();
    // indestructable
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    effect::set_target_range(d, e1, 0, LOCATION_ONFIELD);
    effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE, 0);
    effect::set_target(d, e1, Some(effect_card_fn(indtg)));
    effect::set_value(d, e1, 1);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, rct);
    duel::register_effect(d, e1, a.tp);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    duel::register_effect(d, e2, a.tp);
    // cannot inactivate
    let e5 = effect::create_effect(d, c);
    effect::set_type(d, e5, EFFECT_TYPE_FIELD);
    effect::set_code(d, e5, EFFECT_CANNOT_INACTIVATE);
    effect::set_value_fn(d, e5, lua_fn(efilter));
    effect::set_reset(d, e5, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, rct);
    duel::register_effect(d, e5, a.tp);
    // cannot diseffect
    let e6 = effect::create_effect(d, c);
    effect::set_type(d, e6, EFFECT_TYPE_FIELD);
    effect::set_code(d, e6, EFFECT_CANNOT_DISEFFECT);
    effect::set_value_fn(d, e6, lua_fn(efilter));
    effect::set_reset(d, e6, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, rct);
    duel::register_effect(d, e6, a.tp);
    // cannot disable
    let e7 = effect::create_effect(d, c);
    effect::set_type(d, e7, EFFECT_TYPE_FIELD);
    effect::set_code(d, e7, EFFECT_CANNOT_DISABLE);
    effect::set_target_range(d, e7, 0, LOCATION_ONFIELD);
    effect::set_property(d, e7, EFFECT_FLAG_SET_AVAILABLE, 0);
    effect::set_target(d, e7, Some(effect_card_fn(indtg)));
    effect::set_reset(d, e7, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, rct);
    duel::register_effect(d, e7, a.tp);
}
/// `function(e,c) return c:IsSpellTrap() end`
fn indtg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_spell_trap(d, c)
}
/// `function(e,ct)`: the triggering effect is a Spell/Trap of the opponent
fn efilter(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let ct = a.int(1);
    let info = duel::get_chain_info(d, ct as u8, &[CHAININFO_TRIGGERING_EFFECT, CHAININFO_TRIGGERING_PLAYER]);
    let te = info[0].as_effect().nil();
    let tp = info[1].int() as u8;
    let tc = effect::get_handler(d, te).nil();
    tp != effect::get_handler_player(d, e) && card::is_spell_trap(d, tc)
}
