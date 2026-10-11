//! Red Screen
use super::prelude::*;

pub const ID: u32 = 18634367;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_RED_DRAGON_ARCHFIEND], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE | TIMING_BATTLE_START));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Your opponent's monsters cannot declare attacks
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    card::register_effect(d, c, e2, false);
    // Maintenance cost
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_condition(d, e3, Some(condition(mt_cond)));
    effect::set_operation(d, e3, Some(operation(mtop)));
    card::register_effect(d, c, e3, false);
    // Special Summon 1 Level 1 Tuner monster from your Graveyard
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 0));
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON);
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e4, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e4, 0, Some(TIMING_END_PHASE));
    effect::set_range(d, e4, LOCATION_SZONE);
    effect::set_condition(d, e4, Some(condition(spcon)));
    effect::set_target(d, e4, Some(cost_target(sptg)));
    effect::set_operation(d, e4, Some(operation(spop)));
    effect::set_label(d, e4, &[1]);
    card::register_effect(d, c, e4, false);
}
fn red_archfiend_faceup(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_code(d, c, &[CARD_RED_DRAGON_ARCHFIEND])
}
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_level(d, c, &[1]) && card::is_type(d, c, TYPE_TUNER) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && filter(d, chkc, e, tp);
    }
    if a.chk == Some(0) {
        return true;
    }
    if duel::is_existing_matching_card(d, Some(&red_archfiend_faceup), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX)
        && card::is_destructable(d, c, None)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_target(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, 94)
    {
        effect::set_category(d, e, CATEGORY_DESTROY | CATEGORY_SPECIAL_SUMMON);
        effect::set_property(d, e, EFFECT_FLAG_CARD_TARGET, 0);
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let g = duel::select_target(d, tp, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(c)), 1, tp, 0);
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, tp, 0);
        effect::set_label(d, e, &[1]);
    } else {
        effect::set_category(d, e, 0);
        effect::set_property(d, e, 0, 0);
        effect::set_label(d, e, &[0]);
    }
    true
}
fn mt_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, a.tp)
}
fn mtop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::check_lp_cost(d, tp, 1000) {
        duel::pay_lp_cost(d, tp, 1000);
    } else {
        duel::destroy(d, effect::get_handler(d, a.e).nil(), REASON_COST, None, None);
    }
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    !card::is_status(d, c, STATUS_CHAINING) && duel::is_existing_matching_card(d, Some(&red_archfiend_faceup), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && filter(d, chkc, e, tp);
    }
    if a.chk == Some(0) {
        return card::is_destructable(d, c, None)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(c)), 1, tp, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    if effect::get_label(d, e) != 1 || !duel::is_existing_matching_card(d, Some(&red_archfiend_faceup), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX) {
        return;
    }
    let c = effect::get_handler(d, e).nil();
    let tc = duel::first_target(d);
    if card::is_relate_to_effect(d, c, e) && duel::destroy(d, c, REASON_EFFECT, None, None) > 0 && card::is_relate_to_effect(d, tc.nil(), e) {
        duel::special_summon(d, tc.nil(), 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
