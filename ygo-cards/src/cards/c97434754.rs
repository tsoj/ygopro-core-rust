//! Artmage Litera
use super::prelude::*;

pub const ID: u32 = 97434754;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_ARTMAGE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You take no battle damage from battles involving this card
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_AVOID_BATTLE_DAMAGE);
    effect::set_value(d, e1, 1);
    card::register_effect(d, c, e1, false);
    // Special Summon this card from your hand, then you can add 1 "Artmage" card from your GY to your hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    effect::set_target(d, e2, Some(cost_target(selfsptg)));
    effect::set_operation(d, e2, Some(operation(selfspop)));
    card::register_effect(d, c, e2, false);
    // Special Summon 1 "Artmage" monster from your hand or GY, except "Artmage Litera", then return this card from the field to the hand
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_hint_timing(d, e3, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    effect::set_condition(d, e3, Some(condition(e3_cond)));
    effect::set_target(d, e3, Some(cost_target(sptg)));
    effect::set_operation(d, e3, Some(operation(spop)));
    card::register_effect(d, c, e3, false);
}
fn is_artmage_faceup(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_ARTMAGE as u16])
}
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&is_artmage_faceup), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_ARTMAGE as u16]) && card::is_able_to_hand(d, c, None)
}
fn selfspop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) || duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) == 0 {
        return;
    }
    let f = aux::necro_valley_filter(|d, c| thfilter(d, c));
    let g = duel::get_matching_group(d, Some(&f), a.tp, LOCATION_GRAVE, 0, NO_EX);
    if g.is_empty() || !duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
    if !sg.is_empty() {
        duel::hint_selection(d, sg.clone(), true);
        duel::break_effect(d);
        duel::send_to_hand(d, sg, None, REASON_EFFECT, None);
    }
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_ARTMAGE as u16]) && !card::is_code(d, c, &[ID]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn e3_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_main_phase(d, None) && duel::is_turn_player(d, 1 - a.tp)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX)
            && card::is_able_to_hand(d, c, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 == 0 {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let f = aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp));
    let g = duel::select_matching_card(d, a.tp, Some(&f), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() && duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 && card::is_relate_to_effect(d, c, a.e) {
        duel::break_effect(d);
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
    }
}
