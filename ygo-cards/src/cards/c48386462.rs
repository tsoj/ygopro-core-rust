//! The Cuckoo Commanded to Croon
use super::prelude::*;

pub const ID: u32 = 48386462;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Draw 1 card
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(drtg)));
    effect::set_operation(d, e1, Some(operation(drop_op)));
    card::register_effect(d, c, e1, false);
}
fn discardable(d: &mut Duel, c: CardId) -> bool {
    card::is_discardable(d, c, Some(REASON_EFFECT), None)
}
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let b1 = duel::is_player_can_draw(d, a.tp, 1);
    let b2 = b1 && duel::is_existing_matching_card(d, Some(&discardable), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2)), (true, aux::stringid(ID, 3))]).unwrap_or(0);
    if op == 1 {
        effect::set_category(d, a.e, CATEGORY_DESTROY + CATEGORY_DRAW);
        let h = effect::get_handler(d, a.e).nil();
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(h)), 1, 0, 0);
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    } else if op == 2 {
        effect::set_category(d, a.e, CATEGORY_HANDES + CATEGORY_DRAW);
        duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_HAND as i32);
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    } else if op == 3 {
        effect::set_category(d, a.e, CATEGORY_DRAW);
        duel::set_possible_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    }
    duel::set_target_param(d, op as u32);
    true
}
fn drop_op(d: &mut Duel, a: &EffArgs) {
    let op = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
    let c = effect::get_handler(d, a.e).nil();
    if op == 1 {
        // Destroy this card, then draw 1 card
        if card::is_relate_to_effect(d, c, a.e) && duel::destroy(d, c, REASON_EFFECT, None, None) > 0 {
            duel::break_effect(d);
            duel::draw(d, a.tp, 1, REASON_EFFECT);
        }
    } else if op == 2 {
        // Discard 1 card, then draw 1 card
        if duel::discard_hand(d, a.tp, Some(&discardable), 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX) > 0 {
            duel::break_effect(d);
            duel::draw(d, a.tp, 1, REASON_EFFECT);
        }
    } else if op == 3 {
        // During the End Phase of this turn, draw 1 card
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        effect::set_operation(d, e1, Some(lua_fn(|d, a: Args| {
            duel::draw(d, a.p(1), 1, REASON_EFFECT);
            Value::Nil
        })));
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        duel::register_effect(d, e1, a.tp);
    }
}
