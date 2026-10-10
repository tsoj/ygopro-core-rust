//! Card of Demise
use super::prelude::*;

pub const ID: u32 = 59750328;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Draw up to 3 cards
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_activity_count(d, a.tp, &[ACTIVITY_SPSUMMON])[0] == 0;
    }
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_description(d, e1, aux::stringid(ID, 1));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    effect::set_target_range(d, e1, 1, 0);
    duel::register_effect(d, e1, a.tp);
    true
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    let ct = 3 - duel::get_matching_group_count(d, None, a.tp, LOCATION_HAND, 0, h.into()) as i64;
    if a.chk == Some(0) {
        return ct > 0 && duel::is_player_can_draw(d, a.tp, ct as u32);
    }
    duel::set_target_player(d, a.tp);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, ct as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    let ct = 3 - duel::get_field_group_count(d, p, LOCATION_HAND, 0) as i64;
    if ct > 0 {
        duel::draw(d, p, ct as u32, REASON_EFFECT);
    }
    if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        effect::set_code(d, e1, EFFECT_CHANGE_DAMAGE);
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        effect::set_target_range(d, e1, 0, 1);
        effect::set_value(d, e1, 0);
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        duel::register_effect(d, e1, p);
        let e2 = effect::clone(d, e1);
        effect::set_code(d, e2, EFFECT_NO_EFFECT_DAMAGE);
        effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
        duel::register_effect(d, e2, p);
    }
    //Send your hand to the GY during the End Phase
    let h = effect::get_handler(d, a.e).nil();
    let e3 = effect::create_effect(d, h);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 1);
    effect::set_operation(d, e3, Some(operation(tgop)));
    duel::register_effect(d, e3, p);
}
fn tgop(d: &mut Duel, a: &EffArgs) {
    let owner = effect::get_owner_player(d, a.e);
    let g = duel::get_field_group(d, owner, LOCATION_HAND, 0);
    duel::send_to_grave(d, g, REASON_EFFECT, None, None);
}
