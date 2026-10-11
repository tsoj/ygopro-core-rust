//! Underworld Circle
use super::prelude::*;

pub const ID: u32 = 73443672;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_DUEL as u8);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Allow each player to Special Summon 1 monster from their GY during the Standby Phase
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_PHASE + PHASE_STANDBY);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    duel::is_existing_matching_card(d, Some(&card::is_monster), tp, LOCATION_GRAVE, 0, 5, NO_EX)
        && duel::is_existing_matching_card(d, Some(&card::is_monster), tp, 0, LOCATION_GRAVE, 5, NO_EX)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let g = duel::get_matching_group(d, None, tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), g.len() as u8, tp, 0);
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, PLAYER_ALL as u8, LOCATION_DECK as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
fn spnormalfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_type(d, c, TYPE_NORMAL) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let g = duel::get_matching_group(d, None, tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    if !g.is_empty() && duel::destroy(d, g, REASON_EFFECT, None, None) > 0 {
        let rg = duel::get_matching_group(
            d,
            Some(&|d, c| card::is_able_to_remove(d, c, None, None, None) && card::is_monster(d, c)),
            tp,
            LOCATION_DECK,
            LOCATION_DECK,
            NO_EX,
        );
        if rg.is_empty() {
            return;
        }
        duel::break_effect(d);
        if duel::remove(d, rg, POS_FACEDOWN as u8, REASON_EFFECT, None, None) > 0
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&aux::necro_valley_filter(|d, c| spnormalfilter(d, c, e, tp))), tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
        {
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            let sg = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(|d, c| spnormalfilter(d, c, e, tp))), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
            if !sg.is_empty() {
                duel::break_effect(d);
                duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            }
        }
    }
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, PLAYER_ALL as u8, LOCATION_GRAVE as i32);
    true
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_monster(d, c) && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
fn ignoresumconsp(d: &mut Duel, e: EffectId, p: u8) {
    if duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0 == 0 {
        return;
    }
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, p)), p, LOCATION_GRAVE, 0, NO_EX);
    if g.is_empty() || !duel::select_yes_no(d, p, aux::stringid(ID, 3)) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_SPSUMMON as u64);
    let sel = group::select(d, &g, p, 1, 1, false, NO_EX).nil();
    if let Some(tc) = group::get_first(d, &sel) {
        if duel::special_summon_step(d, tc, 0, p, p, true, false, POS_FACEUP as u8, None) {
            // Banish it if it leaves the field
            let h = effect::get_handler(d, e).nil();
            let e1 = effect::create_effect(d, h);
            effect::set_description(d, e1, 3300);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE | EFFECT_FLAG_CLIENT_HINT, 0);
            effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
            effect::set_value(d, e1, LOCATION_REMOVED as i64);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
            card::register_effect(d, tc, e1, true);
        }
    }
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let turn_p = duel::get_turn_player(d);
    ignoresumconsp(d, a.e, turn_p);
    ignoresumconsp(d, a.e, 1 - turn_p);
    duel::special_summon_complete(d);
}
