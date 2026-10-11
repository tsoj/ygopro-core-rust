//! Mimighoul Dungeon
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 86809440;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MIMIGHOUL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e0, false);
    // "Mimighoul" monsters you control that were not Normal or Special Summoned this turn gain ATK equal to their original DEF
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e1, LOCATION_FZONE);
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    effect::set_target(d, e1, Some(effect_card_fn(e1_target)));
    effect::set_value_fn(d, e1, effect_card_value(e1_value));
    card::register_effect(d, c, e1, false);
    // Any player who controls a face-down monster cannot Normal Summon monsters
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e2, EFFECT_CANNOT_SUMMON);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_target_range(d, e2, 1, 1);
    effect::set_target(d, e2, Some(lua_fn(e2_target)));
    card::register_effect(d, c, e2, false);
    // Any player who controls a face-down monster cannot declare an attack with monsters that were Special Summoned this turn
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_property(d, e3, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_code(d, e3, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    effect::set_range(d, e3, LOCATION_FZONE);
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e3, Some(effect_card_fn(e3_target)));
    card::register_effect(d, c, e3, false);
    // Add 1 "Mimighoul" monster from your Deck or GY to your hand
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 0));
    effect::set_category(d, e4, CATEGORY_TOHAND | CATEGORY_SEARCH);
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e4, LOCATION_FZONE);
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    effect::set_target(d, e4, Some(cost_target(thtg)));
    effect::set_operation(d, e4, Some(operation(thop)));
    card::register_effect(d, c, e4, false);
}
fn e1_target(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_MIMIGHOUL as u16]) && !card::is_status(d, c, STATUS_SUMMON_TURN | STATUS_SPSUMMON_TURN)
}
fn e1_value(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    card::get_base_defense(d, c) as i64
}
fn e2_target(d: &mut Duel, a: Args<'_>) -> bool {
    let sump = a.p(2);
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_facedown(d, c)), sump, LOCATION_MZONE, 0, 1, NO_EX)
}
fn e3_target(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    if !card::is_status(d, c, STATUS_SPSUMMON_TURN) {
        return false;
    }
    let p = card::get_controler(d, c);
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_facedown(d, c)), p, LOCATION_MZONE, 0, 1, NO_EX)
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_MIMIGHOUL as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(thfilter)), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, g);
    }
}
