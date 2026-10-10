//! Cyber Phoenix
use super::prelude::*;

pub const ID: u32 = 3370104;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_DISABLE);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, LOCATION_SZONE, LOCATION_SZONE);
    effect::set_condition(d, e1, Some(condition(discon)));
    effect::set_target(d, e1, Some(effect_card_fn(distg)));
    card::register_effect(d, c, e1, false);
    // Negate effect
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_CHAIN_SOLVING);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_operation(d, e2, Some(operation(disop)));
    card::register_effect(d, c, e2, false);
    // Draw
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_DRAW);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e3, EVENT_BATTLE_DESTROYED);
    effect::set_condition(d, e3, Some(condition(condition_fn)));
    effect::set_target(d, e3, Some(cost_target(target)));
    effect::set_operation(d, e3, Some(operation(operation_fn)));
    card::register_effect(d, c, e3, false);
    aux::double_snare_validity(d, Some(c), LOCATION_MZONE, None);
}
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_attack_pos(d, h)
}
fn distg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    if card::get_card_target_count(d, c) != 1 {
        return false;
    }
    let tc = card::get_first_card_target(d, c).nil();
    card::is_controler(d, tc, effect::get_handler_player(d, e)) && card::is_faceup(d, tc) && card::is_race(d, tc, RACE_MACHINE)
}
fn disop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_attack_pos(d, h) || effect::is_monster_effect(d, a.re.nil()) {
        return;
    }
    if !effect::is_has_property(d, a.re.nil(), EFFECT_FLAG_CARD_TARGET, 0) {
        return;
    }
    let Some(g) = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]).first().and_then(|v| v.as_group().cloned()) else {
        return;
    };
    if g.len() != 1 {
        return;
    }
    let tc = group::get_first(d, &g).nil();
    if card::is_controler(d, tc, a.tp) && card::is_location(d, tc, LOCATION_MZONE) && card::is_faceup(d, tc) && card::is_race(d, tc, RACE_MACHINE) {
        duel::negate_effect(d, a.ev as u8);
    }
}
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_location(d, c, LOCATION_GRAVE) && card::is_reason(d, c, REASON_BATTLE) && card::is_previous_position(d, c, POS_FACEUP as u32)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1);
    }
    duel::set_target_player(d, a.tp);
    duel::set_target_param(d, 1);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
fn operation_fn(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
