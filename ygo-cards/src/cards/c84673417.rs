//! Jain, Twilightsworn General
use super::prelude::*;

pub const ID: u32 = 84673417;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LIGHTSWORN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // reduce atk
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(atkcost)));
    effect::set_target(d, e1, Some(cost_target(atktg)));
    effect::set_operation(d, e1, Some(operation(atkop)));
    card::register_effect(d, c, e1, false);
    // mill
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DECKDES);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_CHAINING);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(ddcon)));
    effect::set_target(d, e2, Some(cost_target(ddtg)));
    effect::set_operation(d, e2, Some(operation(ddop)));
    card::register_effect(d, c, e2, false);
}
fn atkcfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_LIGHTSWORN as u16])
        && card::is_level_above(d, c, 1)
        && card::is_able_to_remove_as_cost(d, c, None)
        && (card::is_location(d, c, LOCATION_HAND) || aux::sp_elim_filter(d, c, true, false))
        && duel::is_existing_target(d, Some(&|d, x| card::is_faceup(d, x)), 0, LOCATION_MZONE, LOCATION_MZONE, 1, c.into())
}
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&atkcfilter), a.tp, LOCATION_HAND | LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&atkcfilter), a.tp, LOCATION_HAND | LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let first = group::get_first(d, &g).nil();
    let lv = card::get_level(d, first);
    effect::set_label(d, a.e, &[lv as i64]);
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    let _g = duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    if let Some(tc) = duel::first_target(d) {
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
            let lv = effect::get_label(d, a.e);
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            effect::set_value(d, e1, -lv * 300);
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            card::register_effect(d, tc, e1, false);
            let e2 = effect::clone(d, e1);
            effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
            card::register_effect(d, tc, e2, false);
        }
    }
}
fn ddcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let rc = effect::get_handler(d, a.re.nil()).nil();
    effect::is_monster_effect(d, a.re.nil()) && rc != c && card::is_set_card(d, rc, &[SET_LIGHTSWORN as u16]) && card::is_controler(d, rc, a.tp)
}
fn ddtg(_d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(_d, 0, CATEGORY_DECKDES, None, 0, a.tp, 2);
    true
}
fn ddop(d: &mut Duel, a: &EffArgs) {
    duel::discard_deck(d, a.tp, 2, REASON_EFFECT);
}
