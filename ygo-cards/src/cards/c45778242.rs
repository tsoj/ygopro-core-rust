//! Cliant
use super::prelude::*;

pub const ID: u32 = 45778242;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atk/def
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    effect::set_condition(d, e1, Some(condition(atkcon)));
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_race(d, c, RACE_CYBERSE as u64))));
    effect::set_value(d, e1, 500);
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    card::register_effect(d, c, e2, false);

    // destroy replace
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    effect::set_code(d, e3, EFFECT_DESTROY_REPLACE);
    effect::set_target(d, e3, Some(cost_target(reptg)));
    effect::set_operation(d, e3, Some(operation(repop)));
    card::register_effect(d, c, e3, false);
}

fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = effect::get_handler_player(d, a.e);
    let h = effect::get_handler(d, a.e).nil();
    duel::is_turn_player(d, tp) && card::is_normal_summoned(d, h)
}

fn repfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_HAND))
        && card::is_race(d, c, RACE_CYBERSE as u64)
        && card::is_destructable(d, c, Some(e))
        && !card::is_status(d, c, STATUS_DESTROY_CONFIRMED)
}

fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if a.chk == Some(0) {
        return !card::is_reason(d, c, REASON_REPLACE)
            && card::is_location(d, c, LOCATION_MZONE)
            && card::is_faceup(d, c)
            && duel::is_existing_matching_card(d, Some(&|d, c2| repfilter(d, c2, e)), tp, LOCATION_MZONE | LOCATION_HAND, 0, 1, c.into());
    }
    if duel::select_effect_yes_no(d, tp, c, Some(96)) {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESREPLACE as u64);
        let g = duel::select_matching_card(d, tp, Some(&|d, c2| repfilter(d, c2, e)), tp, LOCATION_MZONE | LOCATION_HAND, 0, 1, 1, false, c.into()).nil();
        let tc = group::get_first(d, &g).nil();
        effect::set_label_object(d, e, Value::Card(tc));
        card::set_status(d, tc, STATUS_DESTROY_CONFIRMED, true);
        true
    } else {
        false
    }
}

fn repop(d: &mut Duel, a: &EffArgs) {
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    card::set_status(d, tc, STATUS_DESTROY_CONFIRMED, false);
    duel::destroy(d, tc, REASON_EFFECT | REASON_REPLACE, None, None);
}
