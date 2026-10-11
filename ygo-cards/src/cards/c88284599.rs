//! Sauravis, Dragon Sage of the Voiceless Voice
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 88284599;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon procedure
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_condition(d, e1, Some(lua_fn(selfspcon)));
    effect::set_target(d, e1, Some(cost_target(selfsptg)));
    effect::set_operation(d, e1, Some(operation(selfspop)));
    card::register_effect(d, c, e1, false);
    // Special Summon 1 LIGHT Warrior or Dragon Ritual Monster from your hand or Deck
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TODECK);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_CHAINING);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    effect::set_cost(d, e2, Some(cost::self_to_hand()));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_spell(d, c) && card::is_able_to_deck_as_cost(d, c)
}
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    (group::is_exists(d, sg, &|d, c| card::is_ritual_spell(d, c), 1, NO_EX), false)
}
fn selfspcon(d: &mut Duel, a: Args) -> bool {
    let Some(c) = a.card(1) else { return true };
    let e = a.e(0);
    let tp = effect::get_handler_player(d, e);
    let rg = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_HAND | LOCATION_GRAVE, 0, c.into());
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && matches!(
            aux::select_unselect_group(d, &rg, Some(e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false),
            Err(true)
        )
}
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let rg = duel::get_matching_group(d, Some(&cfilter), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, c.into());
    let g = aux::select_unselect_group(
        d,
        &rg,
        Some(a.e),
        a.tp,
        Some(2),
        Some(2),
        Some(&rescon),
        1,
        a.tp,
        Some(HINTMSG_TODECK as u64),
        None,
        None,
        true,
    )
    .unwrap_or_default();
    if !g.is_empty() {
        effect::set_label_object(d, a.e, Value::Group(g));
        return true;
    }
    false
}
fn selfspop(d: &mut Duel, a: &EffArgs) {
    let Some(g) = effect::get_label_object(d, a.e).as_group().cloned() else { return };
    duel::hint_selection(d, g.clone(), true);
    duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_COST, None);
}
fn e2_cond(_d: &mut Duel, a: &EffArgs) -> bool {
    a.ep == 1 - a.tp
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_ritual_monster(d, c)
        && card::is_attribute(d, c, ATTRIBUTE_LIGHT)
        && card::is_race(d, c, RACE_WARRIOR | RACE_DRAGON)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, true, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return duel::get_m_zone_count(d, a.tp, c.into(), None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_MZONE as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let Some(sc) = group::get_first(d, &g) else { return };
    if duel::special_summon(d, sc, 0, a.tp, a.tp, false, true, POS_FACEUP as u8, None) > 0 {
        let turn_count = duel::get_turn_count(d, None);
        // Shuffle it into the Deck during the End Phase of the next turn
        let oper: aux::DelayedFn = Arc::new(|d: &mut Duel, ag: &CardSet, _e: &EffArgs| {
            duel::send_to_deck(d, ag.clone(), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
            true
        });
        let cond: aux::DelayedFn = Arc::new(move |d: &mut Duel, _ag: &CardSet, _e: &EffArgs| duel::get_turn_count(d, None) == turn_count + 1);
        aux::delayed_operation(d, sc, PHASE_END, ID, a.e, a.tp, Some(oper), Some(cond), None, Some(2), Some(aux::stringid(ID, 2)), None);
    }
}
