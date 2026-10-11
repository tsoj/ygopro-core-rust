//! Tri-Brigade Roar
use super::prelude::*;

pub const ID: u32 = 10793085;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TRI_BRIGADE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Send 1 "Tri-Brigade" card to the GY and apply a matching effect
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, Some(TIMING_DAMAGE_STEP | TIMINGS_CHECK_MONSTER_E));
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let _ = a;
    duel::is_existing_matching_card(
        d,
        Some(&|d, c| card::is_faceup(d, c) && card::is_type(d, c, TYPE_LINK)),
        a.tp,
        LOCATION_MZONE,
        0,
        1,
        NO_EX,
    )
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_monster(d, c) && card::is_type(d, c, TYPE_EFFECT) && card::is_faceup(d, c)
}
fn cfilter1(d: &mut Duel, c: CardId) -> bool {
    cfilter(d, c) && card::get_attack(d, c) > 0
}
fn cfilter2(d: &mut Duel, c: CardId) -> bool {
    cfilter(d, c) && card::is_negatable_monster(d, c)
}
fn cfilter3(d: &mut Duel, c: CardId) -> bool {
    cfilter(d, c) && card::is_able_to_hand(d, c, None)
}
fn costfilter(d: &mut Duel, c: CardId, op1: bool, op2: bool, op3: bool) -> bool {
    card::is_set_card(d, c, &[SET_TRI_BRIGADE as u16])
        && card::is_able_to_grave_as_cost(d, c)
        && ((card::is_monster(d, c) && op1) || (card::is_spell(d, c) && op2) || (card::is_trap(d, c) && op3))
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let phase = duel::get_current_phase(d);
    let op1 = duel::is_existing_target(d, Some(&cfilter1), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
        && aux::stat_change_damage_step_condition(d);
    let op2 = duel::is_existing_target(d, Some(&cfilter2), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX) && phase != PHASE_DAMAGE;
    let op3 = duel::is_existing_target(d, Some(&cfilter3), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX) && phase != PHASE_DAMAGE;
    if a.chk == Some(0) {
        return (op1 || op2 || op3)
            && duel::is_existing_matching_card(
                d,
                Some(&|d, c| costfilter(d, c, op1, op2, op3)),
                tp,
                LOCATION_DECK | LOCATION_EXTRA,
                0,
                1,
                NO_EX,
            );
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&|d, c| costfilter(d, c, op1, op2, op3)),
        tp,
        LOCATION_DECK | LOCATION_EXTRA,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    let sc = group::get_first(d, &g).nil();
    duel::send_to_grave(d, sc, REASON_COST, None, None);
    if card::is_monster(d, sc) {
        effect::set_label(d, a.e, &[1]);
        effect::set_category(d, a.e, CATEGORY_ATKCHANGE);
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATKDEF as u64);
        let g2 = duel::select_target(d, tp, Some(&cfilter1), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &g2).nil();
        let atk = card::get_attack(d, tc);
        duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(tc.into()), 1, tp, -atk);
    } else if card::is_spell(d, sc) {
        effect::set_label(d, a.e, &[2]);
        effect::set_category(d, a.e, CATEGORY_DISABLE);
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NEGATE as u64);
        let g2 = duel::select_target(d, tp, Some(&cfilter2), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(g2.clone()), 1, 0, 0);
    } else if card::is_trap(d, sc) {
        effect::set_label(d, a.e, &[3]);
        effect::set_category(d, a.e, CATEGORY_TOHAND);
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
        let g2 = duel::select_target(d, tp, Some(&cfilter3), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g2.clone()), 1, 0, 0);
    }
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    let label = effect::get_label(d, a.e);
    if label == 1 && card::is_faceup(d, tc) {
        // Change its ATK to 0 until the end of this turn
        let c = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        effect::set_value(d, e1, 0);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e1, false);
    } else if label == 2 && card::is_faceup(d, tc) {
        duel::negate_related_chain(d, tc, RESET_TURN_SET);
        // Negate its effects until the end of this turn
        let c = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_DISABLE);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e1, false);
        let e2 = effect::clone(d, e1);
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        effect::set_value(d, e2, RESET_TURN_SET as i64);
        card::register_effect(d, tc, e2, false);
    } else if label == 3 {
        // Return it to the hand
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
    }
}
