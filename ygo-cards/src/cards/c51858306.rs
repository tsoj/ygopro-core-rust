//! Eclipse Wyvern
use super::prelude::*;

pub const ID: u32 = 51858306;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // remove
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_REMOVE);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    card::register_effect(d, c, e1, false);
    // remove
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e2, EVENT_REMOVE);
    effect::set_condition(d, e2, Some(condition(thcon)));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    effect::set_label_object(d, e2, Value::Effect(e1));
    card::register_effect(d, c, e2, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_level_above(d, c, 7) && card::is_race(d, c, RACE_DRAGON) && card::is_attribute(d, c, ATTRIBUTE_LIGHT | ATTRIBUTE_DARK) && card::is_able_to_remove(d, c, None, None, None)
}
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::is_player_affected_by_effect(d, tp, 30459350).is_some() {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&filter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    if let Some(tc) = tc {
        duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
        effect::set_label_object(d, a.e, Value::Card(tc));
        let h = effect::get_handler(d, a.e).nil();
        card::register_flag_effect(
            d,
            h,
            ID,
            RESET_EVENT | RESET_TURN_SET | RESET_TOGRAVE | RESET_TOHAND | RESET_TODECK | RESET_LEAVE | RESET_TOFIELD,
            0,
            1,
            0,
            0,
        );
        card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
    }
}
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_faceup(d, h) && card::get_flag_effect(d, h, ID) != 0
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let e1 = effect::get_label_object(d, a.e).as_effect().nil();
    let tc = effect::get_label_object(d, e1).as_card();
    if a.chk == Some(0) {
        return match tc {
            Some(tc) => card::get_flag_effect(d, tc, ID) != 0 && card::is_able_to_hand(d, tc, None),
            None => false,
        };
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, tc.map(CardSet::from_card), 1, 0, 0);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e1 = effect::get_label_object(d, a.e).as_effect().nil();
    let tc = effect::get_label_object(d, e1).as_card().nil();
    if card::get_flag_effect(d, tc, ID) != 0 && duel::send_to_hand(d, tc, None, REASON_EFFECT, None) != 0 {
        duel::confirm_cards(d, 1 - tp, tc);
    }
}
