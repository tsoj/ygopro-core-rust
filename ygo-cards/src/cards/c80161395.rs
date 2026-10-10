//! Mystik Wok
use super::prelude::*;

pub const ID: u32 = 80161395;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_RECOVER);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    effect::set_label(d, a.e, &[1]);
    true
}
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    card::get_attack(d, c) > 0 || card::get_defense(d, c) > 0
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        if effect::get_label(d, a.e) != 1 {
            return false;
        }
        effect::set_label(d, a.e, &[0]);
        return duel::check_release_group_cost(d, a.tp, Some(&costfilter), 1, 1, false, None, NO_EX);
    }
    let sc = duel::select_release_group_cost(d, a.tp, Some(&costfilter), 1, 1, false, None, NO_EX);
    let sc = group::get_first(d, &sc).nil();
    let atk = card::get_attack(d, sc);
    let def = card::get_defense(d, sc);
    duel::release(d, sc, REASON_COST, None);
    let b1 = atk > 0;
    let b2 = def > 0;
    let op: usize = if b1 && b2 {
        let r = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
        if r == Some(1) {
            1
        } else {
            2
        }
    } else if b1 {
        1
    } else {
        2
    };
    let val = if op == 1 { atk } else { def };
    duel::set_target_player(d, a.tp);
    duel::set_target_param(d, val as u32);
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, val);
    true
}
fn activate(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::recover(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
