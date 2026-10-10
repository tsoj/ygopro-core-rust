//! Illusion of Chaos
use super::prelude::*;

pub const ID: u32 = 12266229;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[21082832, CARD_DARK_MAGICIAN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Search
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_TODECK);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(thcost)));
    effect::set_target(d, e1, Some(cost_target(thtg)));
    effect::set_operation(d, e1, Some(operation(thop)));
    card::register_effect(d, c, e1, false);
    // Disable
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON + CATEGORY_DISABLE);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 1, 0);
    effect::set_condition(d, e1, Some(condition(discon)));
    effect::set_target(d, e1, Some(cost_target(distg)));
    effect::set_operation(d, e1, Some(operation(disop)));
    card::register_effect(d, c, e1, false);
}
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return !card::is_public(d, c);
    }
    duel::confirm_cards(d, 1 - a.tp, c);
    true
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    ((card::lists_code(d, c, &[CARD_DARK_MAGICIAN]) && card::is_monster(d, c) && !card::is_type(d, c, TYPE_RITUAL)) || card::is_code(d, c, &[CARD_DARK_MAGICIAN]))
        && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() && duel::send_to_hand(d, &g, None, REASON_EFFECT, None) > 0 && card::is_location(d, group::get_first(d, &g).nil(), LOCATION_HAND) {
        duel::confirm_cards(d, 1 - tp, &g);
        duel::shuffle_hand(d, tp);
        duel::shuffle_deck(d, tp);
        duel::break_effect(d);
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
        let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_able_to_deck(d, c)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        duel::send_to_deck(d, &g, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
    }
}
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    a.ep == 1 - a.tp && effect::is_monster_effect(d, a.re.nil()) && duel::is_chain_disablable(d, a.ev as u8)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_code(d, c, &[CARD_DARK_MAGICIAN]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None)
            && (duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 || card::is_in_main_m_zone(d, c, None))
            && duel::is_existing_matching_card(d, Some(&|d, tc| spfilter(d, tc, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    true
}
fn disop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if card::is_relate_to_effect(d, c, e)
        && duel::send_to_hand(d, c, None, REASON_EFFECT, None) > 0
        && card::is_location(d, c, LOCATION_HAND)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
    {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let g = duel::select_matching_card(d, tp, Some(&|d, tc| spfilter(d, tc, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        if !g.is_empty() && duel::special_summon(d, &g, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
            duel::negate_effect(d, a.ev as u8);
        }
    }
}
