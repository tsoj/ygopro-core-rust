//! Parallel eXceed
use super::prelude::*;

pub const ID: u32 = 71278040;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card to a zone a Link monster points to
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(handspcon)));
    effect::set_target(d, e1, Some(cost_target(handsptg)));
    effect::set_operation(d, e1, Some(operation(handspop)));
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Parallel eXceed" from your Deck
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_TRIGGER_O + EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e3, false);
    // Change its level, ATK and DEF if it is summoned by the effect of "Parallel eXceed"
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e4, Some(condition(cond)));
    effect::set_operation(d, e4, Some(operation(op)));
    card::register_effect(d, c, e4, false);
}
fn linkfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_link_summoned(d, c) && card::is_summon_player(d, c, tp)
}
fn handspcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| linkfilter(d, c, tp), 1, NO_EX)
}
fn link_zones(d: &mut Duel, eg: &CardSet, tp: u8) -> u32 {
    let linked = group::filter(d, eg, &|d, c| card::is_summon_type(d, c, &[SUMMON_TYPE_LINK]), NO_EX);
    group::get_linked_zone(d, &linked, tp) & ZONES_MMZ
}
fn handsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    let eg = a.eg.clone().nil();
    let zones = link_zones(d, &eg, tp);
    if a.chk == Some(0) {
        return zones != 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(tp), Some(zones));
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
fn handspop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    let eg = a.eg.clone().nil();
    let zones = link_zones(d, &eg, tp);
    if card::is_relate_to_effect(d, c, e) && zones != 0 {
        duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, Some(zones));
    }
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_code(d, c, &[ID]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    match a.re {
        Some(re) => {
            let h = effect::get_handler(d, re).nil();
            card::is_code(d, h, &[ID])
        }
        None => false,
    }
}
fn op(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    // Level becomes 4
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_value(d, e1, 4);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
    card::register_effect(d, c, e1, false);
    // ATK/DEF becomes halved
    let e2 = effect::clone(d, e1);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e2, EFFECT_SET_BASE_ATTACK);
    let atk = lua_int(card::get_base_attack(d, c) as f64 / 2.0);
    effect::set_value(d, e2, atk);
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EFFECT_SET_BASE_DEFENSE);
    let def = lua_int(card::get_base_defense(d, c) as f64 / 2.0);
    effect::set_value(d, e3, def);
    card::register_effect(d, c, e3, false);
}
