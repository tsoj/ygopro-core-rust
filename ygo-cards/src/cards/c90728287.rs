//! Ars Magna - "Philosophirum"
use super::prelude::*;

pub const ID: u32 = 90728287;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ARS_MAGNA, SET_POWER_PATRON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Target up to 3 "Ars Magna" monsters in your field and/or GY; banish them, then you can negate the effects of up to the same number of face-up cards on the field until the end of this turn
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_DISABLE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(bantg)));
    effect::set_operation(d, e1, Some(operation(banop)));
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    card::register_effect(d, c, e1, false);
    // You can banish this card from your GY; return 1 "Power Patron" Link Monster from your GY into the Extra Deck, then you can Special Summon it
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOEXTRA + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(retsptg)));
    effect::set_operation(d, e2, Some(operation(retspop)));
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    card::register_effect(d, c, e2, false);
}
fn banfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_ARS_MAGNA as u16]) && card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None) && card::is_faceup(d, c)
}
fn bantg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_MZONE | LOCATION_GRAVE) && banfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&banfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    let g = duel::select_target(d, a.tp, Some(&banfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 3, false, NO_EX).nil();
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n, a.tp, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_DISABLE, None, n, PLAYER_EITHER as u8, LOCATION_ONFIELD as i32);
    true
}
fn banop(d: &mut Duel, a: &EffArgs) {
    let tg = duel::get_target_cards(d, a.e).nil();
    if tg.len() > 0 && duel::remove(d, tg, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0 {
        let operated = duel::get_operated_group(d);
        let max_negate_count = group::filter_count(d, &operated, &|d, c| card::is_location(d, c, LOCATION_REMOVED), NO_EX);
        if max_negate_count == 0 {
            return;
        }
        let c = effect::get_handler(d, a.e).nil();
        let exc: Exclude = if card::is_relate_to_effect(d, c, a.e) { c.into() } else { NO_EX };
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_negatable(d, c)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, exc);
        if !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_NEGATE as u64);
            let dg = group::select(d, &g, a.tp, 1, max_negate_count as u16, false, NO_EX).nil();
            if dg.is_empty() {
                return;
            }
            duel::hint_selection(d, dg.clone(), true);
            duel::break_effect(d);
            for dc in group::iter(d, &dg) {
                // Negate the effects of up to the same number of face-up cards on the field until the end of this turn
                card::negate_effects(d, dc, c, Some(RESET_PHASE | PHASE_END), true, None);
            }
        }
    }
}
fn retfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_POWER_PATRON as u16]) && card::is_link_monster(d, c) && card::is_able_to_extra(d, c)
}
fn retsptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&retfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOEXTRA, None, 1, a.tp, LOCATION_GRAVE as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
fn retspop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&retfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let Some(sc) = group::get_first(d, &g) else { return };
    duel::hint_selection(d, CardSet::from_card(sc), true);
    if duel::send_to_deck(d, sc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0
        && card::is_location(d, sc, LOCATION_EXTRA)
        && duel::get_location_count_from_ex(d, a.tp, Some(a.tp), NO_EX, duel::ExtraCard::Card(sc), None).0 > 0
        && card::is_can_be_special_summoned(d, sc, a.e, 0, a.tp, false, false, None, None, None)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 3))
    {
        duel::break_effect(d);
        duel::special_summon(d, sc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
