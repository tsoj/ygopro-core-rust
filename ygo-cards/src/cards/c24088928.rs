//! Skull Archfiend of Chaos
use super::prelude::*;

pub const ID: u32 = 24088928;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_LIGHT_AND_DARKNESS_RITUAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If this card is in your hand or GY: You can target 3 other cards in the GYs and/or face-up banishment, including a card that mentions "Ritual of Light and Darkness"; place them on the bottom of the Deck in any order, and if you do, Special Summon this card
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_GRAVE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(tdsptg)));
    effect::set_operation(d, e1, Some(operation(tdspop)));
    card::register_effect(d, c, e1, false);
    // If this card is sent to the GY: You can send 1 Ritual Spell from your hand or Deck to the GY; add 1 Ritual Monster mentioned on that card from your Deck to your hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_cost(d, e2, Some(cost_target(thcost)));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
}
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let res = group::is_exists(d, sg, &|d, c| card::lists_code(d, c, &[CARD_LIGHT_AND_DARKNESS_RITUAL]), 1, NO_EX);
    (res, false)
}
fn tdsptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chkc().is_some() {
        return false;
    }
    let c = effect::get_handler(d, a.e).nil();
    let g = duel::get_target_group(
        d,
        Some(&|d, cc| card::is_faceup(d, cc) && card::is_able_to_deck(d, cc)),
        a.tp,
        LOCATION_GRAVE | LOCATION_REMOVED,
        LOCATION_GRAVE | LOCATION_REMOVED,
        c.into(),
    );
    if a.chk == Some(0) {
        return g.len() >= 3
            && matches!(
                aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(3), Some(3), Some(&rescon), 0, a.tp, None, None, None, false),
                Err(true)
            )
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    let tg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(3), Some(3), Some(&rescon), 1, a.tp, Some(HINTMSG_TODECK as u64), None, None, false)
        .unwrap_or_default();
    duel::set_target_card(d, tg.clone());
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(tg), 3, a.tp, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
fn tdspop(d: &mut Duel, a: &EffArgs) {
    let tg = duel::get_target_cards(d, a.e).nil();
    if !tg.is_empty()
        && duel::send_to_deck(d, tg.clone(), None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None) > 0
        && group::is_exists(d, &tg, &|d, c| card::is_location(d, c, LOCATION_DECK | LOCATION_EXTRA), 1, NO_EX)
    {
        let tp = a.tp;
        let (tg_your, tg_opp) = group::split(d, &tg, &|d, c| card::is_controler(d, c, tp), NO_EX);
        let your_count = group::filter_count(d, &tg_your, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX);
        let opp_count = group::filter_count(d, &tg_opp, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX);
        if your_count >= 2 {
            duel::sort_deckbottom(d, tp, tp, your_count as u16);
        }
        if opp_count >= 2 {
            duel::sort_deckbottom(d, tp, 1 - tp, opp_count as u16);
        }
        let c = effect::get_handler(d, a.e).nil();
        if card::is_relate_to_effect(d, c, a.e) {
            duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
fn thcostfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_ritual_spell(d, c) && card::is_able_to_grave_as_cost(d, c) && duel::is_existing_matching_card(d, Some(&|d, cc| thfilter(d, cc, c)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
fn thfilter(d: &mut Duel, c: CardId, rit_spell: CardId) -> bool {
    let code = card::get_code(d, c)[0];
    card::is_ritual_monster(d, c) && card::lists_code(d, rit_spell, &[code]) && card::is_able_to_hand(d, c, None)
}
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let tp = a.tp;
        return duel::is_existing_matching_card(d, Some(&|d, c| thcostfilter(d, c, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    let tp = a.tp;
    let g = duel::select_matching_card(d, tp, Some(&|d, c| thcostfilter(d, c, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    if let Some(sc) = sc {
        duel::send_to_grave(d, sc, REASON_COST, None, None);
    }
    effect::set_label_object(d, a.e, sc.map_or(Value::Nil, Value::Card));
    true
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let rit_spell = effect::get_label_object(d, a.e).as_card();
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| thfilter(d, c, rit_spell.nil())), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
