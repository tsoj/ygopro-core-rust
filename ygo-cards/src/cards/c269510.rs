//! Fire Prison
use super::prelude::*;

pub const ID: u32 = 269510;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // defup
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_race(d, c, RACE_DRAGON))));
    effect::set_value(d, e2, 300);
    card::register_effect(d, c, e2, false);
    // cannot link summon
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e3, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_range(d, e3, LOCATION_FZONE);
    effect::set_target_range(d, e3, 1, 1);
    effect::set_target(d, e3, Some(lua_fn(splimit)));
    card::register_effect(d, c, e3, false);
    // cannot attack
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    effect::set_code(d, e4, EFFECT_CANNOT_ATTACK);
    effect::set_range(d, e4, LOCATION_FZONE);
    effect::set_target_range(d, e4, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e4, Some(effect_card_fn(atktg)));
    card::register_effect(d, c, e4, false);
    // disable
    let e5 = effect::create_effect(d, c);
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e5, EVENT_CHAIN_SOLVING);
    effect::set_range(d, e5, LOCATION_FZONE);
    effect::set_condition(d, e5, Some(condition(discon)));
    effect::set_operation(d, e5, Some(operation(disop)));
    card::register_effect(d, c, e5, false);
    // cannot attack
    let e6 = effect::create_effect(d, c);
    effect::set_type(d, e6, EFFECT_TYPE_FIELD);
    effect::set_code(d, e6, EFFECT_CANNOT_ATTACK);
    effect::set_range(d, e6, LOCATION_FZONE);
    effect::set_target_range(d, e6, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_condition(d, e6, Some(condition(limcon)));
    effect::set_target(d, e6, Some(effect_card_fn(atlimit)));
    card::register_effect(d, c, e6, false);
    // cannot be battle target
    let e7 = effect::create_effect(d, c);
    effect::set_type(d, e7, EFFECT_TYPE_FIELD);
    effect::set_code(d, e7, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    effect::set_range(d, e7, LOCATION_FZONE);
    effect::set_target_range(d, e7, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_condition(d, e7, Some(condition(limcon)));
    effect::set_value_fn(d, e7, effect_card_fn(atlimit));
    card::register_effect(d, c, e7, false);
    // cannot be effect target
    let e8 = effect::create_effect(d, c);
    effect::set_type(d, e8, EFFECT_TYPE_FIELD);
    effect::set_property(d, e8, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_code(d, e8, EFFECT_CANNOT_BE_EFFECT_TARGET);
    effect::set_range(d, e8, LOCATION_FZONE);
    effect::set_target_range(d, e8, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_condition(d, e8, Some(condition(limcon)));
    effect::set_target(d, e8, Some(effect_card_fn(|d, _e, c| card::is_race(d, c, RACE_CYBERSE))));
    effect::set_value(d, e8, 1);
    card::register_effect(d, c, e8, false);
}
/// `s.splimit(e,c,tp,sumtp,sumpos)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(2);
    let sumtp = a.int(3) as u64;
    let c = a.c(1);
    let g = duel::get_matching_group(d, Some(&aux::faceup_filter(|d, c| card::is_link_monster(d, c))), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    if g.is_empty() {
        return false;
    }
    let lk = group::get_max_group(d, &g, &|d, c| card::get_link(d, c) as i64).map(|(_, v)| v);
    let clink = card::get_link(d, c) as i64;
    (sumtp & SUMMON_TYPE_LINK as u64) == SUMMON_TYPE_LINK as u64 && lk.is_some_and(|v| v > clink)
}
fn atktg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    !card::is_link_monster(d, c)
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_race(d, c, RACE_CYBERSE) && card::is_link_monster(d, c)
}
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    if duel::get_matching_group_count(d, Some(&cfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX) <= 1 {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    let rh = effect::get_handler(d, re).nil();
    card::is_race(d, rh, RACE_CYBERSE)
}
fn disop(d: &mut Duel, a: &EffArgs) {
    duel::negate_effect(d, a.ev as u8);
}
fn limcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    let p = card::get_controler(d, h);
    duel::get_matching_group_count(d, Some(&cfilter), p, LOCATION_MZONE, LOCATION_MZONE, NO_EX) > 1
}
fn atlimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_race(d, c, RACE_CYBERSE)
}
