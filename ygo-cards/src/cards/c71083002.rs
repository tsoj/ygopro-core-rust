//! Tachyon Spiral of Destruction
use super::prelude::*;

pub const ID: u32 = 71083002;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    listed_series: &[SET_GALAXY, SET_TACHYON, SET_NUMBER],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMING_BATTLE_START | TIMING_BATTLE_END | TIMINGS_CHECK_MONSTER_E));
    effect::set_target(d, e1, Some(cost_target(efftg)));
    effect::set_operation(d, e1, Some(operation(effop)));
    card::register_effect(d, c, e1, false);
}
fn galaxyfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_GALAXY as u16]) && card::is_type(d, c, TYPE_XYZ) && card::is_faceup(d, c)
}
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_disabled(d, c) && card::is_faceup(d, c) && !card::is_type(d, c, TYPE_NORMAL)
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_TACHYON as u16]) && card::is_able_to_hand(d, c, None) && !card::is_code(d, c, &[ID])
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_NUMBER as u16])
        && card::is_race(d, c, RACE_DRAGON)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if let Some(chkc) = a.chkc() {
        let op = effect::get_label(d, e);
        if op == 1 || !(card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE)) {
            return false;
        }
        return (op == 2 && thfilter(d, chkc)) || (op == 3 && spfilter(d, chkc, e, tp));
    }
    let b1 = duel::is_existing_matching_card(d, Some(&galaxyfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&desfilter), tp, 0, LOCATION_ONFIELD, 1, NO_EX)
        && !duel::has_flag_effect(d, tp, ID, None);
    let b2 = duel::is_existing_target(d, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX) && !duel::has_flag_effect(d, tp, ID + 1, None);
    let b3 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
        && !duel::has_flag_effect(d, tp, ID + 2, None);
    if a.chk == Some(0) {
        return b1 || b2 || b3;
    }
    let op = duel::select_effect(
        d,
        tp,
        &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2)), (b3, aux::stringid(ID, 3))],
    )
    .map_or(0, |x| x as i64);
    effect::set_label(d, e, &[op]);
    if op == 1 {
        duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        effect::set_category(d, e, CATEGORY_DESTROY);
        effect::set_property(d, e, 0, 0);
        let g = duel::get_matching_group(d, Some(&desfilter), tp, 0, LOCATION_ONFIELD, NO_EX);
        let n = g.len() as u8;
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, tp, 0);
    } else if op == 2 {
        duel::register_flag_effect(d, tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
        effect::set_category(d, e, CATEGORY_TOHAND);
        effect::set_property(d, e, EFFECT_FLAG_CARD_TARGET, 0);
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        let g = duel::select_target(d, tp, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, tp, 0);
    } else if op == 3 {
        duel::register_flag_effect(d, tp, ID + 2, RESET_PHASE | PHASE_END, 0, 1, 0);
        effect::set_category(d, e, CATEGORY_SPECIAL_SUMMON);
        effect::set_property(d, e, EFFECT_FLAG_CARD_TARGET, 0);
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, tp, 0);
    }
    true
}
fn effop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let op = effect::get_label(d, e);
    if op == 1 {
        // Destroy all face-up cards your opponent controls whose effects are negated
        let g = duel::get_matching_group(d, Some(&desfilter), tp, 0, LOCATION_ONFIELD, NO_EX);
        if !g.is_empty() {
            duel::destroy(d, g, REASON_EFFECT, None, None);
        }
    } else if op == 2 {
        // Add 1 "Tachyon" card from your GY to your hand
        let tc = duel::first_target(d).nil();
        if card::is_relate_to_effect(d, tc, e) {
            duel::send_to_hand(d, tc, Some(tp), REASON_EFFECT, None);
        }
    } else if op == 3 {
        // Special Summon 1 "Number" monster from your GY in Defense Position
        let tc = duel::first_target(d).nil();
        if card::is_relate_to_effect(d, tc, e) {
            duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
        }
    }
}
