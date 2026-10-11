//! Millennium Seeker
use super::prelude::*;

pub const ID: u32 = 5130393;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon itself from hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_code(d, e1, EVENT_DAMAGE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Special summon 1 monster from opponent's GY, or opponent adds 1 monster from deck
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_SEARCH + CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_target(d, e2, Some(cost_target(vartg)));
    effect::set_operation(d, e2, Some(operation(varop)));
    card::register_effect(d, c, e2, false);
}
// If you took battle or effect damage
fn spcon(_d: &mut Duel, a: &EffArgs) -> bool {
    a.ep == a.tp && (a.r & (REASON_BATTLE | REASON_EFFECT)) != 0 && a.ev >= 1000
}
// Activation legality
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, Some(POS_FACEUP as u8), None, None);
    }
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
// Special summon this card from hand
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
}
// Check for a non ? ATK monster to special summon
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None) && card::get_text_attack(d, c) >= 0
}
// Activation legality
fn vartg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, 1 - a.tp) && filter(d, chkc, a.e, a.tp);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, 0, LOCATION_GRAVE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, a.tp, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX);
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, a.tp, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, 1 - a.tp, LOCATION_DECK as i32);
    true
}
// Check for a non ? ATK monster to compare
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_monster(d, c) && card::is_able_to_hand(d, c, None) && card::get_text_attack(d, c) >= 0
}
// Special summon 1 monster from opponent's GY, or opponent adds 1 monster from deck
fn varop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    let vc = card::get_text_attack(d, tc);
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    let g = duel::get_matching_group(d, Some(&thfilter), a.tp, 0, LOCATION_DECK, NO_EX);
    if !g.is_empty() && duel::select_yes_no(d, 1 - a.tp, aux::stringid(ID, 2)) {
        duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_CONFIRM as u64);
        let sel = group::select(d, &g, 1 - a.tp, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sel).nil();
        duel::confirm_cards(d, a.tp, sc);
        if card::get_text_attack(d, sc) < vc {
            duel::shuffle_deck(d, 1 - a.tp);
            duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        } else {
            duel::send_to_hand(d, sc, None, REASON_EFFECT, None);
            duel::confirm_cards(d, a.tp, sc);
            duel::shuffle_hand(d, 1 - a.tp);
        }
    } else {
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
