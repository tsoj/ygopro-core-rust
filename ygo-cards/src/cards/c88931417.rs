//! Gatling Ogre
use super::prelude::*;

pub const ID: u32 = 88931417;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Inflict 800 damage to your damage for each facedow Spell/Trap Card sent to the GY
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 3, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(damcost)));
    effect::set_target(d, e1, Some(cost_target(damtg)));
    effect::set_operation(d, e1, Some(operation(damop)));
    card::register_effect(d, c, e1, false);
    // When this card is targeted for an attack: Inflict 500 damage for each Attack Position monster they control, then end the Battle Phase
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE | EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_BE_BATTLE_TARGET);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_target(d, e2, Some(cost_target(damendbptg)));
    effect::set_operation(d, e2, Some(operation(damendbpop)));
    card::register_effect(d, c, e2, false);
}
fn cost_filter(d: &mut Duel, c: CardId) -> bool {
    card::is_facedown(d, c) && card::is_able_to_grave_as_cost(d, c)
}
fn is_attack_pos_filter(d: &mut Duel, c: CardId) -> bool {
    card::is_attack_pos(d, c)
}
fn damcost(d: &mut Duel, a: &EffArgs) -> bool {
    let g = duel::get_matching_group(d, Some(&cost_filter), a.tp, LOCATION_STZONE, 0, NO_EX);
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&cost_filter), a.tp, LOCATION_STZONE, 0, 1, g.len() as u16, false, NO_EX).nil();
    effect::set_label(d, a.e, &[g.len() as i64]);
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let dam = effect::get_label(d, a.e) * 800;
    duel::set_target_player(d, 1 - a.tp);
    duel::set_target_param(d, dam as u32);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, dam as i32);
    true
}
fn damop(d: &mut Duel, a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
    // Your opponent takes no damage from non-Fiend monsters' effects for the rest of this turn
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EFFECT_CHANGE_DAMAGE);
    effect::set_target_range(d, e1, 0, 1);
    effect::set_value_fn(d, e1, lua_fn(damval));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
}
fn damval(d: &mut Duel, a: Args) -> i64 {
    if let Some(re) = a.v(1).as_effect() {
        if (a.int(3) as u32) & REASON_EFFECT > 0 && effect::is_monster_effect(d, re) {
            let rc = effect::get_handler(d, re).nil();
            if card::is_race_except(d, rc, RACE_FIEND, None, 0, None) {
                return 0;
            }
        }
    }
    a.int(2)
}
fn damendbptg(d: &mut Duel, a: &EffArgs) -> bool {
    let ct = duel::get_matching_group_count(d, Some(&is_attack_pos_filter), a.tp, 0, LOCATION_MZONE, NO_EX);
    if a.chk == Some(0) {
        return ct > 0;
    }
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, (ct * 500) as i32);
    true
}
fn damendbpop(d: &mut Duel, a: &EffArgs) {
    let dam = 500 * duel::get_matching_group_count(d, Some(&is_attack_pos_filter), a.tp, 0, LOCATION_MZONE, NO_EX) as i64;
    if dam > 0 && duel::damage(d, 1 - a.tp, dam, REASON_EFFECT, false, None) > 0 {
        duel::break_effect(d);
        duel::skip_phase(d, 1 - a.tp, PHASE_BATTLE, RESET_PHASE | PHASE_BATTLE_STEP, 1, 0);
    }
}
