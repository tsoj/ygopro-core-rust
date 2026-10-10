//! Don Zaloog
use super::prelude::*;

pub const ID: u32 = 76922029;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_BATTLE_DAMAGE);
    effect::set_condition(d, e1, Some(condition(|_d, a| a.ep == 1 - a.tp)));
    effect::set_target(d, e1, Some(cost_target(efftg)));
    effect::set_operation(d, e1, Some(operation(effop)));
    card::register_effect(d, c, e1, false);
}
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let b1 = duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) > 0;
    let b2 = duel::is_player_can_discard_deck(d, 1 - a.tp, 2);
    if a.chk == Some(0) {
        return b1 || b2;
    }
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]).nil();
    effect::set_label(d, a.e, &[op as i64]);
    if op == 1 {
        effect::set_category(d, a.e, CATEGORY_HANDES);
        duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, 1 - a.tp, 1);
    } else {
        effect::set_category(d, a.e, CATEGORY_DECKDES);
        duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, 1 - a.tp, 2);
    }
    true
}
fn effop(d: &mut Duel, a: &EffArgs) {
    if effect::get_label(d, a.e) == 1 {
        // Discard 1 random card from your opponent's hand
        let g = duel::get_field_group(d, a.tp, 0, LOCATION_HAND);
        if g.is_empty() {
            return;
        }
        let sg = group::random_select(d, &g, a.tp, 1);
        duel::send_to_grave(d, sg, REASON_DISCARD | REASON_EFFECT, None, None);
    } else {
        // Send the top 2 cards of your opponent's Deck to the GY
        duel::discard_deck(d, 1 - a.tp, 2, REASON_EFFECT);
    }
}
