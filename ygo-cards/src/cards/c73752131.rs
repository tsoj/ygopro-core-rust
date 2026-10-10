//! Skilled Dark Magician
use super::prelude::*;

pub const ID: u32 = 73752131;

pub const INFO: ScriptInfo = ScriptInfo {
    counter_place_list: &[COUNTER_SPELL],
    listed_names: &[CARD_DARK_MAGICIAN],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_counter_permit(d, c, COUNTER_SPELL as u16, None, None);
    card::set_counter_limit(d, c, COUNTER_SPELL as u16, 3);
    // add counter
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e0, EVENT_CHAINING);
    effect::set_range(d, e0, LOCATION_MZONE);
    effect::set_operation(d, e0, Some(aux::chainreg()));
    card::register_effect(d, c, e0, false);
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EVENT_CHAIN_SOLVED);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_operation(d, e1, Some(operation(acop)));
    card::register_effect(d, c, e1, false);
    // special summon
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_cost(d, e2, Some(cost_target(spcost)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}
fn acop(d: &mut Duel, a: &EffArgs) {
    let re = a.re.nil();
    let c = effect::get_handler(d, a.e).nil();
    if effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && effect::is_spell_effect(d, re) && card::get_flag_effect(d, c, 1) > 0 {
        card::add_counter(d, c, COUNTER_SPELL as u16, 1, false);
    }
}
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::get_counter(d, c, COUNTER_SPELL as u16) == 3 && card::is_releasable(d, c, None, None);
    }
    duel::release(d, c, REASON_COST, None);
    true
}
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_code(d, c, &[CARD_DARK_MAGICIAN]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    let c = effect::get_handler(d, a.e).nil();
    if card::get_sequence(d, c) < 5 {
        ft += 1;
    }
    if a.chk == Some(0) {
        return ft > 0 && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK | LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 0, a.tp, (LOCATION_DECK | LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&aux::necro_valley_filter(|d, c| filter(d, c, a.e, a.tp))), a.tp, LOCATION_DECK | LOCATION_HAND | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
