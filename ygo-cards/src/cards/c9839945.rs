//! Lyna the Light Charmer, Lustrous
use super::prelude::*;

pub const ID: u32 = 9839945;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Link summon
    link::add_procedure(d, c, None, 2, Some(2), Some(group_check(lcheck)), None, None);
    card::enable_revive_limit(d, c);
    // Special Summon
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
    // Add to hand
    let e3 = effect::create_effect(d, c);
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e3, EVENT_DESTROYED);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_condition(d, e3, Some(condition(thcon)));
    effect::set_target(d, e3, Some(cost_target(thtg)));
    effect::set_operation(d, e3, Some(operation(thop)));
    card::register_effect(d, c, e3, false);
}
/// `s.lcheck(g,lc,sumtype,tp)`
fn lcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    group::is_exists(d, g, &|d, c| card::is_attribute_ctx(d, c, ATTRIBUTE_LIGHT, Some(lc), sumtype, Some(tp)), 1, NO_EX)
}
/// `s.spfilter(c,e,tp,zone)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, zone: u32) -> bool {
    card::is_attribute(d, c, ATTRIBUTE_LIGHT) && (zone != 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(tp), Some(zone)))
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    let _lg = card::get_linked_group(d, c);
    let zone = card::get_free_linked_zone(d, c, None) & ZONES_MMZ;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, 1 - tp) && spfilter(d, chkc, a.e, tp, zone);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, x| spfilter(d, x, a.e, tp, zone)), tp, 0, LOCATION_GRAVE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, tp, Some(&|d, x| spfilter(d, x, a.e, tp, zone)), tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, tp, LOCATION_GRAVE as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let zone = card::get_free_linked_zone(d, effect::get_handler(d, a.e).nil(), None) & ZONES_MMZ;
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) && zone != 0 {
            duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, Some(zone));
        }
    }
}
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT) && a.rp == 1 - a.tp && card::is_link_summoned(d, c) && card::is_previous_location(d, c, LOCATION_MZONE)
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_defense_below(d, c, 1500) && card::is_monster(d, c) && card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
