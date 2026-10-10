//! Magical Merchant
use super::prelude::*;

pub const ID: u32 = 32362575;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Excavate cards until you find a Spell/Trap
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_DECKDES);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let g = duel::get_matching_group(d, Some(&card::is_spell_trap), tp, LOCATION_DECK, 0, NO_EX);
    let dcount = duel::get_field_group_count(d, tp, LOCATION_DECK, 0) as i64;
    if dcount == 0 {
        return;
    }
    if g.is_empty() {
        duel::confirm_decktop(d, tp, dcount as u32);
        duel::shuffle_deck(d, tp);
        return;
    }
    let mut seq: i64 = -1;
    let mut spcard: Option<CardId> = None;
    for tc in group::iter(d, &g) {
        if card::get_sequence(d, tc) as i64 > seq {
            seq = card::get_sequence(d, tc) as i64;
            spcard = Some(tc);
        }
    }
    duel::confirm_decktop(d, tp, (dcount - seq) as u32);
    let spcard = spcard.nil();
    if card::is_able_to_hand(d, spcard, None) {
        duel::disable_shuffle_check(d, true);
        duel::send_to_hand(d, spcard, None, REASON_EFFECT, None);
        duel::discard_deck(d, tp, (dcount - seq - 1) as u16, REASON_EFFECT | REASON_EXCAVATE);
        duel::confirm_cards(d, 1 - tp, spcard);
        duel::shuffle_hand(d, tp);
    } else {
        duel::discard_deck(d, tp, (dcount - seq) as u16, REASON_EFFECT | REASON_EXCAVATE);
    }
}
