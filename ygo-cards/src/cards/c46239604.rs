//! Dupe Frog
use super::prelude::*;

pub const ID: u32 = 46239604;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID, 84451804], listed_series: &[SET_FROG], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // This card's name becomes "Des Frog" while on the field
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_CHANGE_CODE);
    effect::set_value(d, e1, 84451804);
    card::register_effect(d, c, e1, false);
    // Add 1 "Frog" monster from your Deck or GY to your hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_condition(d, e2, Some(condition(cond)));
    effect::set_target(d, e2, Some(cost_target(target)));
    effect::set_operation(d, e2, Some(operation(op)));
    card::register_effect(d, c, e2, false);
    // Monsters your opponent controls cannot target monsters for attacks, except this one
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_code(d, e3, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_target_range(d, e3, 0, LOCATION_MZONE);
    effect::set_value_fn(d, e3, lua_fn(atlimit));
    card::register_effect(d, c, e3, false);
}
/// `s.atlimit(e,c)`
fn atlimit(d: &mut Duel, a: Args) -> bool {
    // return c~=e:GetHandler()
    let h = effect::get_handler(d, a.e(0)).nil();
    a.card(1) != Some(h)
}
/// `s.condition(e,...)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD)
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, h, LOCATION_ONFIELD)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_FROG as u16]) && card::is_monster(d, c) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_GRAVE | LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_GRAVE | LOCATION_DECK) as i32);
    true
}
/// `s.operation(e,tp,...)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let nv = aux::necro_valley_filter(thfilter);
    let g = duel::select_matching_card(d, tp, Some(&nv), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
