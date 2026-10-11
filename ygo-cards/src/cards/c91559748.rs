//! Prickle Fairy
use super::prelude::*;

pub const ID: u32 = 91559748;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // change position
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_POSITION);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_DAMAGE_STEP_END);
    effect::set_condition(d, e1, Some(condition(cpcon)));
    effect::set_target(d, e1, Some(cost_target(cptg)));
    effect::set_operation(d, e1, Some(operation(cpop)));
    card::register_effect(d, c, e1, false);
    // cannot be battle target
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    effect::set_value_fn(d, e2, effect_card_fn(tg));
    card::register_effect(d, c, e2, false);
}
/// `s.tg(e,c)`
fn tg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_race(d, c, RACE_INSECT)
}
fn cpcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let t = card::get_battle_target(d, c);
    effect::set_label_object(d, a.e, t.map_or(Value::Nil, |t| Value::from(t)));
    match t {
        Some(t) => card::is_relate_to_battle(d, t),
        None => false,
    }
}
fn cptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let t = effect::get_label_object(d, a.e).as_card();
    duel::set_operation_info(d, 0, CATEGORY_POSITION, t.map(CardSet::from_card), 1, 0, 0);
    true
}
fn cpop(d: &mut Duel, a: &EffArgs) {
    let g = effect::get_label_object(d, a.e).as_card().nil();
    if card::is_relate_to_battle(d, g) && card::is_attack_pos(d, g) {
        duel::change_position(d, g, POS_FACEUP_DEFENSE as u8, None, None, None, false);
    }
}
