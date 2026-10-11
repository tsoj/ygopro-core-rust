//! Altergeist Pixiel
use super::prelude::*;

pub const ID: u32 = 57769391;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ALTERGEIST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Excavate the top 3 cards from your Deck
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND | CATEGORY_DECKDES);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost::self_tribute()));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}

fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck(d, a.tp, 3);
    }
    true
}

fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_able_to_hand(d, c, None) && card::is_set_card(d, c, &[SET_ALTERGEIST as u16])
}

fn op(d: &mut Duel, a: &EffArgs) {
    if !duel::is_player_can_discard_deck(d, a.tp, 3) {
        return;
    }
    duel::confirm_decktop(d, a.tp, 3);
    let mut g = duel::get_decktop_group(d, a.tp, 3);
    if !g.is_empty() {
        duel::disable_shuffle_check(d, true);
        if group::is_exists(d, &g, &filter, 1, NO_EX) {
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
            let sg = group::filter_select(d, &g, a.tp, &filter, 1, 1, false, NO_EX).nil();
            duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
            duel::confirm_cards(d, 1 - a.tp, sg.clone());
            duel::shuffle_hand(d, a.tp);
            group::remove_card(&mut g, &sg);
        }
        duel::send_to_grave(d, g, REASON_EFFECT | REASON_EXCAVATE, None, None);
    }
}
