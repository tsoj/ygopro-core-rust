//! Missus Radiant
use super::prelude::*;

pub const ID: u32 = 3987233;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    link::add_procedure(d, c, Some(mat_filter(|d, x, sc, st, tp| card::is_attribute_ctx(d, x, ATTRIBUTE_EARTH, sc, st, Some(tp)))), 2, Some(2), None, None, None);
    card::enable_revive_limit(d, c);
    // atk/def
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, x| card::is_attribute(d, x, ATTRIBUTE_EARTH))));
    effect::set_value(d, e1, 500);
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e1);
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, x| card::is_attribute(d, x, ATTRIBUTE_WIND))));
    effect::set_value(d, e3, -400);
    card::register_effect(d, c, e3, false);
    let e4 = effect::clone(d, e3);
    effect::set_code(d, e4, EFFECT_UPDATE_DEFENSE);
    card::register_effect(d, c, e4, false);
    // tohand
    let e5 = effect::create_effect(d, c);
    effect::set_description(d, e5, aux::stringid(ID, 0));
    effect::set_category(d, e5, CATEGORY_TOHAND);
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE | EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e5, EFFECT_FLAG_DELAY | EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e5, EVENT_DESTROYED);
    effect::set_count_limit(d, e5, 1, ID, 0, 0);
    effect::set_condition(d, e5, Some(condition(thcon)));
    effect::set_target(d, e5, Some(cost_target(thtg)));
    effect::set_operation(d, e5, Some(operation(thop)));
    card::register_effect(d, c, e5, false);
}
fn thcon(_d: &mut Duel, a: &EffArgs) -> bool {
    // (r & REASON_EFFECT + REASON_BATTLE) ~= 0: '+' binds tighter than '&' in Lua
    (a.r & (REASON_EFFECT + REASON_BATTLE)) != 0
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_monster(d, c) && card::is_attribute(d, c, ATTRIBUTE_EARTH) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && thfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_target(d, tp, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
    }
}
