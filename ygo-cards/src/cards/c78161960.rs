//! Reject Reborn
use super::prelude::*;

pub const ID: u32 = 78161960;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let Some(eg) = a.eg.clone() else {
        return false;
    };
    let first = group::get_first(d, &eg).nil();
    card::is_controler(d, first, 1 - tp) && duel::get_attack_target(d).is_none()
}
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_type(d, c, TYPE_TUNER)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && duel::is_existing_matching_card(d, Some(&|d, c2| filter2(d, c2, e, tp)), tp, LOCATION_GRAVE, 0, 1, c.into())
}
fn filter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_type(d, c, TYPE_SYNCHRO) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return true;
    }
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_BATTLE, 0, 1, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, tp, LOCATION_GRAVE as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    if duel::get_flag_effect(d, tp, ID) == 0 {
        return;
    }
    let c = effect::get_handler(d, e).nil();
    duel::skip_phase(d, 1 - tp, PHASE_BATTLE, RESET_PHASE | PHASE_BATTLE_STEP, 1, 0);
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 1
        && duel::is_existing_matching_card(d, Some(&|d, c2| filter(d, c2, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 0))
    {
        duel::break_effect(d);
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let g1 = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(|d, c2| filter(d, c2, e, tp))), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let ex = match group::get_first(d, &g1) {
            Some(first) => first.into(),
            None => NO_EX,
        };
        let g2 = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(|d, c2| filter2(d, c2, e, tp))), tp, LOCATION_GRAVE, 0, 1, 1, false, ex).nil();
        let mut g1 = g1;
        group::add_card(&mut g1, &g2);
        for tc in group::iter(d, &g1) {
            duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_DISABLE);
            effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, tc, e1, false);
            let e2 = effect::create_effect(d, c);
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, tc, e2, false);
        }
        duel::special_summon_complete(d);
    }
}
