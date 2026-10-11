//! Evo-Karma
use super::prelude::*;

pub const ID: u32 = 8632967;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // act limit
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_operation(d, e2, Some(operation(sucop)));
    card::register_effect(d, c, e2, false);
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_code(d, e3, EVENT_CHAIN_END);
    effect::set_operation(d, e3, Some(operation(cedop)));
    effect::set_label_object(d, e3, Value::Effect(e2));
    card::register_effect(d, c, e3, false);
}
fn sucfilter(d: &mut Duel, c: CardId) -> bool {
    let st = card::get_summon_type(d, c);
    card::is_set_card(d, c, &[SET_EVOLTILE as u16]) && st >= SUMMON_TYPE_SPECIAL + 150 && st < SUMMON_TYPE_SPECIAL + 180
}
fn sucop(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg.clone().nil();
    if group::is_exists(d, &eg, &sucfilter, 1, NO_EX) {
        effect::set_label(d, a.e, &[1]);
    } else {
        effect::set_label(d, a.e, &[0]);
    }
}
fn cedop(d: &mut Duel, a: &EffArgs) {
    if duel::check_event(d, EVENT_SPSUMMON_SUCCESS).is_some() {
        let lo = effect::get_label_object(d, a.e).as_effect().nil();
        if effect::get_label(d, lo) == 1 {
            duel::set_chain_limit_till_chain_end(d, lua_fn(|_d, a| a.int(2) == a.int(1)));
        }
    }
}
