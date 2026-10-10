//! Pot of Extravagance
use super::prelude::*;

pub const ID: u32 = 49238328;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_phase(d, PHASE_MAIN1) && !duel::check_phase_activity(d)
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    effect::set_label(d, a.e, &[100]);
    true
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_facedown(d, c) && card::is_able_to_remove_as_cost(d, c, Some(POS_FACEDOWN as u8))
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let g = duel::get_matching_group(d, Some(&cfilter), a.tp, LOCATION_EXTRA, 0, NO_EX);
    if a.chk == Some(0) {
        if effect::get_label(d, a.e) != 100 {
            return false;
        }
        effect::set_label(d, a.e, &[0]);
        return duel::is_player_can_draw(d, a.tp, 1) && g.len() >= 3;
    }
    let op: i32 = if duel::is_player_can_draw(d, a.tp, 2) && g.len() >= 6 {
        duel::select_option(d, a.tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1)])
    } else {
        duel::select_option(d, a.tp, true, &[aux::stringid(ID, 0)])
    };
    let rg = group::random_select(d, &g, a.tp, (3 + op * 3) as usize);
    duel::remove(d, rg, POS_FACEDOWN as u8, REASON_COST, None, None);
    duel::set_target_player(d, a.tp);
    duel::set_target_param(d, (op + 1) as u32);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, op + 1);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CANNOT_DRAW);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_description(d, e1, aux::stringid(ID, 2));
    effect::set_target_range(d, e1, 1, 0);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
}
