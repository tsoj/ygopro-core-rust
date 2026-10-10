//! Sky Striker Mecha - Shark Cannon
use super::prelude::*;

pub const ID: u32 = 51227866;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Banish 1 monster from opponent's GY, or if 3+ spells in your GY, special summon it
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE as u32));
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::get_sequence(d, c) < 5
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    !duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, spchk: bool) -> bool {
    card::is_monster(d, c)
        && (card::is_able_to_remove(d, c, None, None, None)
            || (spchk && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let mut spchk = false;
    if duel::get_matching_group_count(d, Some(&|d, c| card::is_spell(d, c)), tp, LOCATION_GRAVE, 0, NO_EX) >= 3 {
        spchk = true;
    }
    if let Some(chkc) = a.chkc() {
        // `c` is an undefined global in the Lua script (nil)
        let c_nil: Option<CardId> = None;
        return card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_GRAVE) && filter(d, c_nil.nil(), a.e, tp, spchk);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, a.e, tp, spchk)), tp, 0, LOCATION_GRAVE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| filter(d, c, a.e, tp, spchk)), tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, g.clone(), 1, tp, LOCATION_GRAVE as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, g.clone(), 1, tp, LOCATION_GRAVE as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, tp, LOCATION_GRAVE as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        if duel::get_matching_group_count(d, Some(&|d, c| card::is_spell(d, c)), tp, LOCATION_GRAVE, 0, NO_EX) >= 3
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, tc, a.e, 0, tp, false, false, None, None, None)
            && (!card::is_able_to_remove(d, tc, None, None, None) || duel::select_yes_no(d, tp, aux::stringid(ID, 0)))
        {
            duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            // Cannot attack
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            effect::set_description(d, e1, 3206);
            effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, tc, e1, true);
        } else {
            duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
