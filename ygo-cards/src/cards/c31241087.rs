//! Transcendosaurus Meteorus
//! Scripted by Satella
use super::prelude::*;

pub const ID: u32 = 31241087;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_unsummonable(d, c);
    // Must be Special Summoned by a card effect
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    effect::set_value_fn(d, e1, lua_fn(value1));
    card::register_effect(d, c, e1, false);
    // During your opponent's turn (Quick Effect): You can destroy 2 other Dinosaur monsters in your hand and/or field, and if you do, Special Summon this card from your hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER_E));
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(cond2)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
    // If this card is Special Summoned: You can send 1 Dinosaur monster from your Deck to the GY
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_TOGRAVE);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_target(d, e3, Some(cost_target(tgtg)));
    effect::set_operation(d, e3, Some(operation(tgop)));
    card::register_effect(d, c, e3, false);
    // If this card is destroyed: You can shuffle 1 Normal Monster from your GY into the Deck, then you can Special Summon this card
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 2));
    effect::set_category(d, e4, CATEGORY_TODECK + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e4, EVENT_DESTROYED);
    effect::set_count_limit(d, e4, 1, ID, 2, 0);
    effect::set_target(d, e4, Some(cost_target(tdtg)));
    effect::set_operation(d, e4, Some(operation(tdop)));
    card::register_effect(d, c, e4, false);
}
/// `function(e,se) return se:IsHasType(EFFECT_TYPE_ACTIONS) end`
fn value1(d: &mut Duel, a: Args) -> bool {
    let se = a.e(1);
    effect::is_has_type(d, se, EFFECT_TYPE_ACTIONS)
}
/// `function(_,tp) return Duel.IsTurnPlayer(1-tp) end`
fn cond2(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_race(d, c, RACE_DINOSAUR) && (card::is_location(d, c, LOCATION_HAND) || card::is_faceup(d, c))
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, Some(&desfilter), a.tp, LOCATION_MZONE | LOCATION_HAND, 0, c.into());
    if a.chk == Some(0) {
        return group::get_count(&g) >= 2
            && matches!(aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&aux::chkf_mmz(1)), 0, a.tp, None, None, None, false), Err(true))
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 2, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, Some(&desfilter), a.tp, LOCATION_MZONE | LOCATION_HAND, 0, c.into());
    if group::get_count(&g) < 2 {
        return;
    }
    let dg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&aux::chkf_mmz(1)), 1, a.tp, Some(HINTMSG_DESTROY as u64), None, None, false).unwrap_or_default();
    if group::get_count(&dg) == 2 && duel::destroy(d, dg, REASON_EFFECT, None, None) == 2 && card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, CardSet::from_card(c), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        card::complete_procedure(d, c);
    }
}
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_race(d, c, RACE_DINOSAUR) && card::is_able_to_grave(d, c, None)
}
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn tgop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if group::get_count(&g) > 0 {
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_type(d, c, TYPE_NORMAL) && card::is_able_to_deck(d, c)
}
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tdfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_GRAVE as i32);
    let h = effect::get_handler(d, a.e).nil();
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, a.tp, 0);
    true
}
fn tdop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&tdfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    if group::get_count(&g) == 0 {
        return;
    }
    duel::hint_selection(d, g.clone(), true);
    let c = effect::get_handler(d, a.e).nil();
    if duel::send_to_deck(d, g.clone(), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0
        && card::is_location(d, group::get_first(d, &g).nil(), LOCATION_DECK)
        && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && card::is_relate_to_effect(d, c, a.e)
        && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None)
        && aux::nvfilter(d, c)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 3))
    {
        duel::break_effect(d);
        duel::special_summon(d, CardSet::from_card(c), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        card::complete_procedure(d, c);
    }
}
