//! Slip Summon
use super::prelude::*;

pub const ID: u32 = 94484482;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e2, Some(condition(condition2)));
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e1);
    effect::set_code(d, e3, EVENT_FLIP_SUMMON_SUCCESS);
    card::register_effect(d, c, e3, false);
}
fn condition_fn(_d: &mut Duel, a: &EffArgs) -> bool {
    a.ep != a.tp
}
fn condition2(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    let opp = 1 - a.tp;
    group::is_exists(d, &eg, &|d, c| card::is_summon_player(d, c, opp), 1, NO_EX)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_level_below(d, c, 4) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    if let Some(tc) = tc {
        if duel::special_summon(d, g.clone(), 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None) != 0 {
            let h = effect::get_handler(d, a.e).nil();
            let fid = card::get_field_id(d, h);
            card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, fid as i64, 0);
            let e1 = effect::create_effect(d, h);
            effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
            effect::set_count_limit(d, e1, 1, 0, 0, 0);
            effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
            effect::set_label(d, e1, &[fid as i64]);
            effect::set_label_object(d, e1, Value::Card(tc));
            effect::set_condition(d, e1, Some(condition(retcon)));
            effect::set_operation(d, e1, Some(operation(retop)));
            duel::register_effect(d, e1, tp);
        }
    }
}
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tc = effect::get_label_object(d, e).as_card().nil();
    let flag = card::get_flag_effect_label(d, tc, ID).first().copied();
    if flag != Some(effect::get_label(d, e)) {
        effect::reset(d, e);
        false
    } else {
        true
    }
}
fn retop(d: &mut Duel, a: &EffArgs) {
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
}
