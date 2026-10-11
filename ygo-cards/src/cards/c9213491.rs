//! Serene Psychic Girl
use super::prelude::*;

pub const ID: u32 = 9213491;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Banish 1 card from your hand, and if you do, Special Summon 1 Level 4 or lower Psychic monster from your Deck
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(rmvsptg)));
    effect::set_operation(d, e1, Some(operation(rmvspop)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e2, false);
    // Increase the Level of face-up monster(s) by 1
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_LVCHANGE);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_target(d, e3, Some(cost_target(lvtg)));
    effect::set_operation(d, e3, Some(operation(lvop)));
    card::register_effect(d, c, e3, false);
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_level_below(d, c, 4)
        && card::is_race(d, c, RACE_PSYCHIC)
        && !card::is_code(d, c, &[ID])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn rmvsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, LOCATION_HAND, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, tp, LOCATION_HAND as i32);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
fn rmvspop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let rg = duel::select_matching_card(d, tp, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    if !rg.is_empty()
        && duel::remove(d, rg.clone(), POS_FACEUP as u8, REASON_EFFECT, None, None) > 0
        && card::is_location(d, group::get_first(d, &rg).nil(), LOCATION_REMOVED)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
    {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        if !g.is_empty() {
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
    let c = effect::get_handler(d, e).nil();
    // You cannot Special Summon from the Extra Deck for the rest of this turn, except Psychic monsters
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 2));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA) && !card::is_race(d, c, RACE_PSYCHIC))));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, tp);
    // "Clock Lizard" check
    aux::add_temp_lizard_check(d, c, tp, Some(effect_card_fn(|d, _e, c| !card::is_original_race(d, c, RACE_PSYCHIC))), None, None, None, None);
}
fn lvtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc) && card::has_level(d, chkc);
    }
    let ct = duel::get_matching_group_count(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_PSYCHIC)), tp, LOCATION_MZONE, 0, NO_EX);
    if a.chk == Some(0) {
        return ct > 0 && duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c) && card::has_level(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_faceup(d, c) && card::has_level(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, ct as u16, false, NO_EX).nil();
    let n = g.len();
    duel::set_operation_info(d, 0, CATEGORY_LVCHANGE, Some(g), n as u8, tp, 1);
    true
}
fn lvop(d: &mut Duel, a: &EffArgs) {
    let mut tg = duel::get_target_cards(d, a.e).nil();
    group::match_(d, &mut tg, &|d, c| card::is_faceup(d, c), NO_EX);
    if tg.is_empty() {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    for tc in group::iter(d, &tg) {
        // Increase their Levels by 1
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
        effect::set_value(d, e1, 1);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e1, false);
    }
}
