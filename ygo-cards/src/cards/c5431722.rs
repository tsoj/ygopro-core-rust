//! Dracotail Flame
use super::prelude::*;

pub const ID: u32 = 5431722;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DRACOTAIL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate the effects of 1 face-up Spell, then you can place 1 "Dragontail" card from your GY or banishment on the bottom of the Deck, then draw 1 card
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DISABLE + CATEGORY_TODECK + CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn disfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_spell(d, c) && card::is_negatable_spell_trap(d, c)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && disfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&disfilter), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_NEGATE as u64);
    let g = duel::select_target(d, a.tp, Some(&disfilter), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, g, 1, a.tp, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, (LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_DRAW, None, 1, a.tp, 1);
    true
}
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_DRACOTAIL as u16]) && card::is_faceup(d, c) && card::is_able_to_deck(d, c) && !card::is_code(d, c, &[ID])
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d).nil();
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) && card::is_can_be_disabled_by_effect(d, tc, a.e, true) {
        card::negate_effects(d, tc, c, Some(RESETS_STANDARD_PHASE_END), false, None);
        duel::adjust_instantly(d, Some(tc));
        let f = aux::necro_valley_filter(tdfilter);
        if card::is_disabled(d, tc)
            && duel::is_existing_matching_card(d, Some(&f), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX)
            && duel::is_player_can_draw(d, tp, 1)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
        {
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
            let g = duel::select_matching_card(d, tp, Some(&f), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
            if g.is_empty() {
                return;
            }
            duel::hint_selection(d, &g, true);
            duel::break_effect(d);
            if duel::send_to_deck(d, &g, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None) > 0 {
                duel::break_effect(d);
                duel::draw(d, tp, 1, REASON_EFFECT);
            }
        }
    }
}
