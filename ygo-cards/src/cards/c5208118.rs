//! Solving for Pendulum
use super::prelude::*;

pub const ID: u32 = 5208118;
const COUNTER_T: u16 = 0x218;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_counter_permit(d, c, COUNTER_T, None, None);
    // Activate
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e0, false);
    // Once per turn, during the Standby Phase: Place 1 T Counter on this card
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_COUNTER);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_PHASE + PHASE_STANDBY);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_operation(d, e1, Some(operation(|d, a| {
        let c = effect::get_handler(d, a.e).nil();
        card::add_counter(d, c, COUNTER_T, 1, false);
    })));
    card::register_effect(d, c, e1, false);
    // Once per turn: You can target 1 Pendulum Monster in your Main Monster Zone; move that Pendulum Monster to an unused adjacent Main Monster Zone a number of times equal to the number of T Counters on this card, then if your opponent controls in the targeted monster's column a Fusion and/or Xyz Monster(s) with a Level/Rank equal to or lower than the targeted monster's Pendulum Scale, destroy as many of them as possible, and if you do, inflict damage to your opponent equal to the total ATK the destroyed monsters had on the field
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let c = effect::get_handler(d, a.e).nil();
        card::has_counter(d, c, COUNTER_T)
    })));
    effect::set_target(d, e2, Some(cost_target(movetg)));
    effect::set_operation(d, e2, Some(operation(moveop)));
    effect::set_hint_timing(d, e2, TIMING_STANDBY_PHASE, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    card::register_effect(d, c, e2, false);
}
fn movefilter(d: &mut Duel, c: CardId) -> bool {
    card::is_pendulum_monster(d, c) && card::is_faceup(d, c) && card::check_adjacent(d, c)
}
fn movetg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&movefilter), a.tp, LOCATION_MMZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    duel::select_target(d, a.tp, Some(&movefilter), a.tp, LOCATION_MMZONE, 0, 1, 1, false, NO_EX);
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, 1 - a.tp, LOCATION_MZONE as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 0);
    true
}
/// `s.desfilter(c, opp, scale)`
fn desfilter(d: &mut Duel, c: CardId, opp: u8, scale: u32) -> bool {
    if !((card::is_fusion_monster(d, c) || card::is_xyz_monster(d, c)) && card::is_faceup(d, c) && card::is_controler(d, c, opp)) {
        return false;
    }
    if card::has_level(d, c) {
        card::is_level_below(d, c, scale)
    } else if card::has_rank(d, c) {
        card::is_rank_below(d, c, scale)
    } else {
        false
    }
}
fn moveop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if !(card::is_relate_to_effect(d, c, a.e) && card::has_counter(d, c, COUNTER_T)) {
        return;
    }
    let tc = duel::first_target(d).nil();
    let move_count = card::get_counter(d, c, COUNTER_T);
    if card::is_relate_to_effect(d, tc, a.e) && card::is_pendulum_monster(d, tc) && card::is_faceup(d, tc) {
        for _ in 0..move_count {
            card::move_adjacent(d, tc, None);
        }
    }
    let opp = 1 - tp;
    let scale = card::get_scale(d, tc);
    let mut column_g = card::get_column_group(d, tc, 0, 0);
    group::match_(d, &mut column_g, &|d, cc| desfilter(d, cc, opp, scale), NO_EX);
    if column_g.is_empty() {
        return;
    }
    duel::break_effect(d);
    if duel::destroy(d, column_g, REASON_EFFECT, None, None) == 0 {
        return;
    }
    let operated = duel::get_operated_group(d);
    let total_atk = group::get_sum(d, &operated, &|d, cc| card::get_previous_attack_on_field(d, cc) as i64);
    if total_atk > 0 {
        duel::damage(d, opp, total_atk, REASON_EFFECT, false, None);
    }
}
