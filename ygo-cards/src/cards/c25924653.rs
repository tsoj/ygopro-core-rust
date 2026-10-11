//! Gladiator Beast Darius
use super::prelude::*;

pub const ID: u32 = 25924653;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GLADIATOR_BEAST], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Gladiator Beast" monster from your GY
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e1, Some(aux::gbspcon()));
    effect::set_target(d, e1, Some(cost_target(gysptg)));
    effect::set_operation(d, e1, Some(operation(gyspop)));
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Gladiator Beast" monster from your Deck
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_PHASE | PHASE_BATTLE);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_condition(d, e2, Some(condition(battled)));
    effect::set_cost(d, e2, Some(cost::self_to_deck()));
    effect::set_target(d, e2, Some(cost_target(dsptg)));
    effect::set_operation(d, e2, Some(operation(dspop)));
    card::register_effect(d, c, e2, false);
}
fn battled(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::get_battled_group_count(d, h) > 0
}
fn gyspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_GLADIATOR_BEAST as u16])
        && card::is_can_be_special_summoned(d, c, e, 104, tp, false, false, None, None, None)
}
fn gysptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && gyspfilter(d, chkc, a.e, tp);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| gyspfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| gyspfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
fn gyspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && duel::special_summon_step(d, tc, 104, tp, tp, false, false, POS_FACEUP as u8, None) {
        card::negate_effects(d, tc, c, None, false, None);
        if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
            card::set_card_target(d, c, tc);
            // Register when this card is about the leave the field
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
            effect::set_code(d, e1, EVENT_LEAVE_FIELD_P);
            effect::set_operation(d, e1, Some(operation(regop)));
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, c, e1, false);
        }
    }
    duel::special_summon_complete(d);
}
fn regop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tc = match card::get_first_card_target(d, c) {
        Some(tc) => tc,
        None => return,
    };
    if !card::is_location(d, tc, LOCATION_MZONE) {
        return;
    }
    // Shuffle it into the Deck when this card leaves the field
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_LEAVE_FIELD);
    effect::set_operation(d, e1, Some(operation(tdop)));
    card::register_effect(d, c, e1, false);
}
fn tdop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if let Some(tc) = card::get_first_card_target(d, c) {
        if card::is_location(d, tc, LOCATION_MZONE) {
            duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        }
    }
    effect::reset(d, a.e);
}
fn dspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_GLADIATOR_BEAST as u16])
        && card::is_can_be_special_summoned(d, c, e, 104, tp, false, false, None, None, None)
        && !card::is_code(d, c, &[ID])
}
fn dsptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return duel::get_m_zone_count(d, a.tp, c.into(), None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| dspfilter(d, x, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn dspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, x| dspfilter(d, x, a.e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    if let Some(sc) = sc {
        if duel::special_summon(d, sc, 104, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
            let code = card::get_original_code(d, sc);
            card::register_flag_effect(d, sc, code, RESET_EVENT | RESETS_STANDARD_DISABLE, 0, 0, 0, 0);
        }
    }
}
