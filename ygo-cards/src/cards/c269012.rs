//! Mound of the Bound Creator
use super::prelude::*;

pub const ID: u32 = 269012;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Cannot be target
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, x| card::is_level_above(d, x, 10))));
    effect::set_value(d, e2, 1);
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EFFECT_CANNOT_BE_EFFECT_TARGET);
    effect::set_property(d, e3, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    card::register_effect(d, c, e3, false);
    // Damage
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 0));
    effect::set_category(d, e4, CATEGORY_DAMAGE);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD | EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e4, EVENT_BATTLE_DESTROYED);
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_range(d, e4, LOCATION_FZONE);
    effect::set_condition(d, e4, Some(condition(damcon)));
    effect::set_target(d, e4, Some(cost_target(damtg)));
    effect::set_operation(d, e4, Some(operation(damop)));
    card::register_effect(d, c, e4, false);
    // Search
    let e5 = effect::create_effect(d, c);
    effect::set_description(d, e5, aux::stringid(ID, 1));
    effect::set_category(d, e5, CATEGORY_TOHAND | CATEGORY_SEARCH);
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE | EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e5, EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e5, EVENT_TO_GRAVE);
    effect::set_condition(d, e5, Some(condition(thcon)));
    effect::set_target(d, e5, Some(cost_target(thtg)));
    effect::set_operation(d, e5, Some(operation(thop)));
    card::register_effect(d, c, e5, false);
}
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    let des = group::get_first(d, &eg).nil();
    let rc = card::get_reason_card(d, des);
    card::is_location(d, des, LOCATION_GRAVE)
        && card::is_monster(d, des)
        && card::is_relate_to_battle(d, rc.nil())
        && card::is_level_above(d, rc.nil(), 10)
}
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let eg = a.eg.clone().nil();
    let des = group::get_first(d, &eg).nil();
    let damp = card::get_previous_controler(d, des);
    duel::set_target_player(d, damp);
    duel::set_target_param(d, 1000);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, damp, 1000);
    true
}
fn damop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    (a.r & (REASON_EFFECT + REASON_DESTROY)) == (REASON_EFFECT + REASON_DESTROY) && card::is_previous_location(d, h, LOCATION_ONFIELD)
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_attribute(d, c, ATTRIBUTE_DIVINE) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&filter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, g);
    }
}
