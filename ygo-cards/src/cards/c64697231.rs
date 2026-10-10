//! Trap Dustshoot
use super::prelude::*;

pub const ID: u32 = 64697231;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_TODECK);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_TOHAND));
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) > 3
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_target_player(d, a.tp);
    true
}
fn activate(d: &mut Duel, _a: &EffArgs) {
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    let g = duel::get_field_group(d, p, 0, LOCATION_HAND);
    if !g.is_empty() {
        duel::confirm_cards(d, p, &g);
        let tg = group::filter(d, &g, &card::is_monster, NO_EX);
        if !tg.is_empty() {
            duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_TODECK as u64);
            let sg = group::select(d, &tg, p, 1, 1, false, NO_EX).nil();
            duel::send_to_deck(d, sg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        }
        duel::shuffle_hand(d, 1 - p);
    }
}
