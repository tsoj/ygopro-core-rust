//! Joker's Straight
use super::prelude::*;

pub const ID: u32 = 92067220;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_JACK_KNIGHT, CARD_QUEEN_KNIGHT, CARD_KING_KNIGHT],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SUMMON + CATEGORY_HANDES);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // to hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_TODECK);
    effect::set_type(d, e2, EFFECT_TYPE_TRIGGER_O + EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    effect::set_operation(d, e2, Some(operation(tdop)));
    card::register_effect(d, c, e2, false);
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_code(d, c, &[CARD_QUEEN_KNIGHT]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(tp), Some(0xff))
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[CARD_KING_KNIGHT, CARD_JACK_KNIGHT]) && card::is_able_to_hand(d, c, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| card::is_discardable(d, x, Some(REASON_EFFECT), None)), a.tp, LOCATION_HAND, 0, 1, h.into())
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // Cannot Special Summon from the Extra Deck, except LIGHT Warriors
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 3));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_target(d, e1, Some(effect_card_fn(splimit)));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
    if duel::discard_hand(d, a.tp, Some(&|d, x| card::is_discardable(d, x, Some(REASON_EFFECT), None)), 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX) == 0
        || duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 1
    {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let sg = duel::select_matching_card(d, a.tp, Some(&|d, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if sg.is_empty() || duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) == 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let hg = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if hg.is_empty() || duel::send_to_hand(d, hg.clone(), None, REASON_EFFECT, None) == 0 {
        return;
    }
    duel::confirm_cards(d, 1 - a.tp, hg);
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_summonable(d, x, true, None, 0, None)), a.tp, LOCATION_HAND, 0, NO_EX);
    if !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
        duel::break_effect(d);
        duel::shuffle_hand(d, a.tp);
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SUMMON as u64);
        let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sg).nil();
        duel::summon(d, a.tp, sc, true, None, 0, None);
    }
}
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    !(card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_race(d, c, RACE_WARRIOR)) && card::is_location(d, c, LOCATION_EXTRA)
}
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_race(d, c, RACE_WARRIOR) && card::is_able_to_deck(d, c)
}
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && tdfilter(d, chkc);
    }
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_able_to_hand(d, h, None) && duel::is_existing_target(d, Some(&tdfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    let g = duel::select_target(d, a.tp, Some(&tdfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_TODECK, g, 1, 0, 0);
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d);
    let c = effect::get_handler(d, a.e).nil();
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e)
            && duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) != 0
            && card::is_location(d, tc, LOCATION_DECK | LOCATION_EXTRA)
            && card::is_relate_to_effect(d, c, a.e)
        {
            duel::send_to_hand(d, c, None, REASON_EFFECT, None);
        }
    }
}
