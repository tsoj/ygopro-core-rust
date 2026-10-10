//! The Seal of Orichalcos
use super::prelude::*;

pub const ID: u32 = 48179391;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, (EFFECT_COUNT_CODE_OATH + EFFECT_COUNT_CODE_DUEL) as u8);
    effect::set_target(d, e1, Some(cost_target(acttg)));
    effect::set_operation(d, e1, Some(operation(actop)));
    card::register_effect(d, c, e1, false);
    // You cannot Special Summon monsters from the Extra Deck
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e2, 1, 0);
    effect::set_target(d, e2, Some(effect_card_fn(sumlimit)));
    card::register_effect(d, c, e2, false);
    // All monsters you control gain 500 ATK
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_range(d, e3, LOCATION_FZONE);
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    effect::set_target_range(d, e3, LOCATION_MZONE, 0);
    effect::set_value(d, e3, 500);
    card::register_effect(d, c, e3, false);
    // Cannot be destroyed by effects once per turn
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e4, LOCATION_FZONE);
    effect::set_code(d, e4, EFFECT_INDESTRUCTABLE_COUNT);
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    effect::set_value_fn(d, e4, lua_fn(valcon));
    card::register_effect(d, c, e4, false);
    // Monsters with the lowest attacks cannot be targeted by attacks
    let e5 = effect::create_effect(d, c);
    effect::set_type(d, e5, EFFECT_TYPE_FIELD);
    effect::set_code(d, e5, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    effect::set_range(d, e5, LOCATION_FZONE);
    effect::set_target_range(d, e5, 0, LOCATION_MZONE);
    effect::set_condition(d, e5, Some(condition(atkcon)));
    effect::set_value_fn(d, e5, effect_card_fn(atlimit));
    card::register_effect(d, c, e5, false);
    // clock lizard
    aux::add_continuous_lizard_check(d, c, LOCATION_FZONE, None, None, None);
}
/// `s.desfilter(c)`
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_special_summoned(d, c)
}
fn acttg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let g = duel::get_matching_group(d, Some(&desfilter), a.tp, LOCATION_MZONE, 0, NO_EX);
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
fn actop(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_matching_group(d, Some(&desfilter), a.tp, LOCATION_MZONE, 0, NO_EX);
    if !g.is_empty() {
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
/// `s.sumlimit(e,c,sump,sumtype,sumpos,targetp)`
fn sumlimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_location(d, c, LOCATION_EXTRA)
}
/// `s.valcon(e,re,r,rp)`
fn valcon(_d: &mut Duel, a: Args) -> bool {
    (a.int(2) & REASON_EFFECT as i64) != 0
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_position(d, c, POS_FACEUP_ATTACK)), p, LOCATION_MZONE, 0, 2, NO_EX)
}
/// `s.atkfilter(c,atk)`
fn atkfilter(d: &mut Duel, c: CardId, atk: i32) -> bool {
    card::is_faceup(d, c) && card::get_attack(d, c) < atk
}
/// `s.atlimit(e,c)`
fn atlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    if !card::is_faceup(d, c) {
        return false;
    }
    let p = effect::get_handler_player(d, e);
    let atk = card::get_attack(d, c);
    !duel::is_existing_matching_card(d, Some(&|d, x| atkfilter(d, x, atk)), p, LOCATION_MZONE, 0, 1, c.into())
}
