//! Card Advance
use super::prelude::*;

pub const ID: u32 = 52112003;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Look at up to 5 cards from the top of your Deck, then place them on the top of the Deck in any order
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_player_can_summon(d, a.tp, None) && duel::is_player_can_additional_tribute_summon(d, a.tp)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) > 0;
    }
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let ct = duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0).min(5);
    if ct > 0 {
        let ac = if ct == 1 {
            ct
        } else {
            duel::announce_number_range(d, a.tp, Some(1), Some(ct as u32), &[]).nil().0 as i32
        };
        duel::sort_decktop(d, a.tp, a.tp, ac as u16);
    }
    if !duel::is_player_can_additional_tribute_summon(d, a.tp) {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
    // You can Tribute Summon 1 monster in addition to your Normal Summon/Set this turn
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 2));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_EXTRA_SUMMON_COUNT);
    effect::set_target_range(d, e1, LOCATION_HAND, 0);
    effect::set_value(d, e1, 0x1);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_EXTRA_SET_COUNT);
    duel::register_effect(d, e2, a.tp);
}
