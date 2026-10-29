//! Abduction
use super::prelude::*;

pub const ID: u32 = 21846145;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Banish 1 monster from your Deck or Extra Deck and take control of opponent's monster
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_CONTROL);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    effect::set_operation(d, e1, Some(operation(rmop)));
    card::register_effect(d, c, e1, false);
}
/// `original_lvrklnk` of the Lua script: the Level, Rank or Link of a card (`None` for Lua `false`)
fn orig_lvrklnk(d: &mut Duel, c: CardId) -> Option<u32> {
    if card::has_level(d, c) {
        return Some(card::get_original_level(d, c));
    }
    if card::has_rank(d, c) {
        return Some(card::get_original_rank(d, c));
    }
    if card::is_link_monster(d, c) {
        return Some(card::get_link(d, c));
    }
    None
}
/// `s.ctfilter(c,tp)`
fn ctfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    if !(card::is_faceup(d, c) && card::is_able_to_change_controler(d, c)) {
        return false;
    }
    let lvrklnk = orig_lvrklnk(d, c);
    let race = card::get_original_race(d, c);
    let attr = card::get_original_attribute(d, c);
    duel::is_existing_matching_card(
        d,
        Some(&|d, cc| rmfilter(d, cc, race, attr, c, lvrklnk)),
        tp,
        LOCATION_DECK | LOCATION_EXTRA,
        0,
        1,
        NO_EX,
    )
}
/// `s.rmfilter(c,race,attr,tc,lvrklnk)`
fn rmfilter(d: &mut Duel, c: CardId, race: u64, attr: u32, tc: CardId, lvrklnk: Option<u32>) -> bool {
    // a branch below only reads lvrklnk when tc has that Level/Rank/Link, which makes it a number
    let v = lvrklnk.unwrap_or(0);
    card::is_able_to_remove(d, c, None, None, None)
        && card::is_original_race(d, c, race)
        && card::is_original_attribute(d, c, attr)
        && ((card::has_level(d, tc) && card::is_original_level(d, c, v))
            || (card::has_rank(d, tc) && card::is_original_rank(d, c, v))
            || (card::is_link_monster(d, tc) && card::is_link(d, c, &[v])))
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, Some(tp), Some(LOCATION_REASON_CONTROL), None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| ctfilter(d, c, tp)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| ctfilter(d, c, tp)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, tp, (LOCATION_DECK | LOCATION_EXTRA) as i32);
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, g, 1, 0, 0);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let tc = duel::first_target(d).nil();
    if !(card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc)) {
        return;
    }
    let lvrklnk = orig_lvrklnk(d, tc);
    let race = card::get_original_race(d, tc);
    let attr = card::get_original_attribute(d, tc);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let sel = duel::select_matching_card(
        d,
        tp,
        Some(&|d, c| rmfilter(d, c, race, attr, tc, lvrklnk)),
        tp,
        LOCATION_DECK | LOCATION_EXTRA,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    if let Some(rc) = group::get_first(d, &sel) {
        if duel::remove(d, CardSet::from_card(rc), POS_FACEUP as u8, REASON_EFFECT, None, None) > 0
            && card::is_location(d, rc, LOCATION_REMOVED)
        {
            // Duel.GetControl returns a number, which is always true in Lua
            duel::get_control(d, CardSet::from_card(tc), tp, 0, 0, None, None);
            let codes = card::get_original_code_rule(d, tc);
            if card::is_original_code_rule(d, rc, &codes) {
                let h = effect::get_handler(d, a.e).nil();
                card::negate_effects(d, tc, h, Some(RESET_CONTROL), false, None);
            }
        }
    }
}
