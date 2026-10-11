//! Returned Dino Daneen
use super::prelude::*;

pub const ID: u32 = 29927283;

// The script's `s.listeds_series={SET_GMX}` (sic) is not read by any script or the core, so it is left out.

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If this card is Special Summoned by a monster effect: You can add 1 "GMX" monster from your Deck to your hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(|d, a| match a.re {
        None => false,
        Some(re) => effect::is_monster_effect(d, re),
    })));
    effect::set_target(d, e1, Some(cost_target(thtg)));
    effect::set_operation(d, e1, Some(operation(thop)));
    card::register_effect(d, c, e1, false);
    // During your Main Phase: You can Fusion Summon 1 Dinosaur Fusion Monster from your Extra Deck, using monsters from your hand or field
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    let tg = fusion::summon_eff_tg(d, fusion_params());
    effect::set_target(d, e2, Some(tg));
    let op = fusion::summon_eff_op(d, fusion_params());
    effect::set_operation(d, e2, Some(op));
    card::register_effect(d, c, e2, false);
    // If this card is sent to the GY: You can make all Dinosaur monsters you currently control gain 400 ATK
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_category(d, e3, CATEGORY_ATKCHANGE);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    effect::set_target(d, e3, Some(cost_target(atktg)));
    effect::set_operation(d, e3, Some(operation(atkop)));
    card::register_effect(d, c, e3, false);
}
fn fusion_params() -> fusion::SummonEff {
    fusion::SummonEff {
        fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_race(d, c, RACE_DINOSAUR))),
        ..Default::default()
    }
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_GMX as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
fn dino_faceup(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_race(d, c, RACE_DINOSAUR)
}
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&dino_faceup), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    true
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, Some(&dino_faceup), a.tp, LOCATION_MZONE, 0, NO_EX);
    for tc in group::iter(d, &g) {
        // All Dinosaur monsters you currently control gain 400 ATK
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_value(d, e1, 400);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e1, false);
    }
}
