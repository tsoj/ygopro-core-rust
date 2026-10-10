//! Sangan
use super::prelude::*;

pub const ID: u32 = 26202165;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Search
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, c, LOCATION_ONFIELD)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_attack_below(d, c, 1500) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&filter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, &g);
        let tc = group::get_first(d, &g).nil();
        if card::is_location(d, tc, LOCATION_HAND) {
            let e1 = effect::create_effect(d, effect::get_handler(d, a.e).nil());
            effect::set_type(d, e1, EFFECT_TYPE_FIELD);
            effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
            effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
            effect::set_target_range(d, e1, 1, 0);
            effect::set_value_fn(d, e1, lua_fn(aclimit));
            let codes: Vec<i64> = card::get_code(d, tc).into_iter().map(|x| x as i64).collect();
            effect::set_label(d, e1, &codes);
            effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
            duel::register_effect(d, e1, tp);
        }
    }
}
/// `function(e, re, tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    let (e, re) = (a.e(0), a.e(1));
    let h = effect::get_handler(d, re).nil();
    let codes: Vec<u32> = effect::get_labels(d, e).into_iter().map(|x| x as u32).collect();
    card::is_code(d, h, &codes)
}
