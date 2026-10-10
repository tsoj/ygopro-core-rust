//! Poison Chain
use super::prelude::*;

pub const ID: u32 = 33302407;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_IRON_CHAIN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // discard deck
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_category(d, e2, CATEGORY_DECKDES);
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(discon)));
    effect::set_target(d, e2, Some(cost_target(distg)));
    effect::set_operation(d, e2, Some(operation(disop)));
    card::register_effect(d, c, e2, false);
}
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    a.tp == duel::get_turn_player(d) && duel::get_activity_count(d, a.tp, &[ACTIVITY_ATTACK])[0] == 0
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_IRON_CHAIN as u16])
}
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    let ct = duel::get_matching_group_count(d, Some(&filter), a.tp, LOCATION_MZONE, 0, NO_EX);
    if a.chk == Some(0) {
        return ct > 0 && duel::is_player_can_discard_deck(d, 1 - a.tp, ct as u32);
    }
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, 1 - a.tp, ct as i32);
    true
}
fn disop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let ct = duel::get_matching_group_count(d, Some(&filter), a.tp, LOCATION_MZONE, 0, NO_EX);
    if ct > 0 {
        duel::discard_deck(d, 1 - a.tp, ct as u16, REASON_EFFECT);
    }
}
