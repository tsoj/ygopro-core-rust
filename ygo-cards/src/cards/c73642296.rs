//! Ghost Belle & Haunted Mansion
use super::prelude::*;

pub const ID: u32 = 73642296;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate the activation of a card or effect that includes adding a card(s) to the hand, Deck, and/or Extra Deck, Special Summoning a Monster Card, or banishing a card(s), from the GY
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_NEGATE);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(negcon)));
    effect::set_cost(d, e1, Some(cost::self_discard()));
    effect::set_target(d, e1, Some(cost_target(negtg)));
    effect::set_operation(d, e1, Some(operation(|d, a| {
        duel::negate_activation(d, a.ev as u8);
    })));
    card::register_effect(d, c, e1, false);
}
/// `s.check(ev,category)`
fn check(d: &mut Duel, ev: u32, category: u64) -> bool {
    let op1 = duel::get_operation_info(d, ev, category);
    let op2 = duel::get_possible_operation_info(d, ev, category);
    if op1.is_none() && op2.is_none() {
        return false;
    }
    let loc1 = op1.as_ref().map_or(0, |o| o.op_param);
    let loc2 = op2.as_ref().map_or(0, |o| o.op_param);
    let mut g = CardSet::new();
    if let Some(g1) = op1.as_ref().and_then(|o| o.op_cards.as_ref()) {
        group::add_card(&mut g, g1);
    }
    if let Some(g2) = op2.as_ref().and_then(|o| o.op_cards.as_ref()) {
        group::add_card(&mut g, g2);
    }
    if ((loc1 | loc2) & LOCATION_GRAVE as i32) > 0 {
        return true;
    }
    !g.is_empty() && group::is_exists(d, &g, &|d, c| card::is_location(d, c, LOCATION_GRAVE) && (category != CATEGORY_SPECIAL_SUMMON || card::is_monster(d, c)), 1, NO_EX)
}
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    if !duel::is_chain_negatable(d, a.ev as u8) {
        return false;
    }
    check(d, a.ev, CATEGORY_SPECIAL_SUMMON)
        || check(d, a.ev, CATEGORY_REMOVE)
        || check(d, a.ev, CATEGORY_TOHAND)
        || check(d, a.ev, CATEGORY_TODECK)
        || check(d, a.ev, CATEGORY_TOEXTRA)
}
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, a.tp, 0);
    true
}
