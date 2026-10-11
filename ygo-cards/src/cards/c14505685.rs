//! Cyberse Converter
use super::prelude::*;

pub const ID: u32 = 14505685;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_condition(d, e1, Some(lua_fn(sprcon)));
    card::register_effect(d, c, e1, false);
    // race
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE | EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    effect::set_target(d, e2, Some(cost_target(rctg)));
    effect::set_operation(d, e2, Some(operation(rcop)));
    card::register_effect(d, c, e2, false);
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_facedown(d, c) || !card::is_race(d, c, RACE_CYBERSE)
}
fn sprcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    let tp = card::get_controler(d, c);
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::get_field_group_count(d, tp, LOCATION_MZONE, 0) > 0
        && !duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
fn rcfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && !card::is_race(d, c, RACE_CYBERSE)
}
fn rctg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && rcfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&rcfilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    let _g = duel::select_target(d, tp, Some(&rcfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
fn rcop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d);
    if let Some(tc) = tc {
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
            let c = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_CHANGE_RACE);
            effect::set_value(d, e1, RACE_CYBERSE as i64);
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            card::register_effect(d, tc, e1, false);
        }
    }
}
