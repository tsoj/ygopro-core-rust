//! Layer 19 "Sudden Incursion! Super Quantum Black!!"
use super::prelude::*;

pub const ID: u32 = 98349765;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SUPER_QUANT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMING_BATTLE_START | TIMINGS_CHECK_MONSTER_E));
    effect::set_cost(d, e1, Some(cost_target(effcost)));
    effect::set_target(d, e1, Some(cost_target(efftg)));
    effect::set_operation(d, e1, Some(operation(effop)));
    card::register_effect(d, c, e1, false);
}
/// `Card.IsAttackPos` and `Card.IsCanChangePosition` (`aux.AND`)
fn attack_change(d: &mut Duel, c: CardId) -> bool {
    card::is_attack_pos(d, c) && card::is_can_change_position(d, c)
}
/// `aux.FaceupFilter(Card.IsSetCard, SET_SUPER_QUANT)`
fn faceup_super_quant(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_SUPER_QUANT as u16])
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_SUPER_QUANT as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
        && {
            let attr = card::get_original_attribute(d, c);
            !duel::is_existing_matching_card(
                d,
                Some(&|d, cc| card::is_faceup(d, cc) && card::is_original_attribute(d, cc, attr)),
                tp,
                LOCATION_MZONE,
                0,
                1,
                NO_EX,
            )
        }
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_SUPER_QUANT as u16]) && card::is_trap(d, c) && card::is_s_setable(d, c, false, None)
}
/// `s.effcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn effcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    effect::set_label(d, a.e, &[-100]);
    let b1 = !duel::has_flag_effect(d, tp, ID, None)
        && duel::is_existing_target(d, Some(&attack_change), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    let b2 = !duel::has_flag_effect(d, tp, ID + 1, None)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&faceup_super_quant), tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    let b3 = !duel::has_flag_effect(d, tp, ID + 2, None)
        && (card::is_location(d, effect::get_handler(d, a.e).nil(), LOCATION_SZONE)
            || duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 >= 2)
        && duel::is_existing_matching_card(d, Some(&setfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    if a.chk == Some(0) {
        return b1 || b2 || b3;
    }
    true
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return effect::get_label(d, a.e) == 1
            && card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_attack_pos(d, chkc)
            && card::is_can_change_position(d, chkc);
    }
    let cost_skip = effect::get_label(d, a.e) != -100;
    let b1 = (cost_skip || !duel::has_flag_effect(d, tp, ID, None))
        && duel::is_existing_target(d, Some(&attack_change), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    let b2 = (cost_skip
        || (!duel::has_flag_effect(d, tp, ID + 1, None)
            && duel::is_existing_matching_card(d, Some(&faceup_super_quant), tp, LOCATION_MZONE, 0, 1, NO_EX)))
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    let b3 = (cost_skip || !duel::has_flag_effect(d, tp, ID + 2, None))
        && (!effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE)
            || card::is_location(d, effect::get_handler(d, a.e).nil(), LOCATION_SZONE)
            || duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 >= 2)
        && duel::is_existing_matching_card(d, Some(&setfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    if a.chk == Some(0) {
        effect::set_label(d, a.e, &[0]);
        return b1 || b2 || b3;
    }
    // Duel.SelectEffect is not cancelable and only returns nil when no option is enabled
    let op = duel::select_effect(
        d,
        tp,
        &[
            (b1, aux::stringid(ID, 1)),
            (b2, aux::stringid(ID, 2)),
            (b3, aux::stringid(ID, 3)),
        ],
    )
    .unwrap_or(0) as i64;
    effect::set_label(d, a.e, &[op]);
    if op == 1 {
        effect::set_category(d, a.e, CATEGORY_POSITION);
        effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
        if !cost_skip {
            duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        }
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_POSCHANGE as u64);
        let g = duel::select_target(d, tp, Some(&attack_change), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
        duel::set_operation_info(d, 0, CATEGORY_POSITION, g, 1, tp, 0);
    } else if op == 2 {
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        effect::set_property(d, a.e, 0, 0);
        if !cost_skip {
            duel::register_flag_effect(d, tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
        }
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    } else if op == 3 {
        effect::set_category(d, a.e, CATEGORY_SET);
        effect::set_property(d, a.e, 0, 0);
        if !cost_skip {
            duel::register_flag_effect(d, tp, ID + 2, RESET_PHASE | PHASE_END, 0, 1, 0);
        }
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let op = effect::get_label(d, a.e);
    if op == 1 {
        // Change 1 Attack Position monster on the field to Defense Position
        let tc = duel::first_target(d).nil();
        if card::is_relate_to_effect(d, tc, a.e) {
            duel::change_position(d, CardSet::from_card(tc), POS_FACEUP_DEFENSE as u8, None, None, None, false);
        }
    } else if op == 2 {
        // Special Summon 1 "Super Quant" monster from your Deck in Defense Position, with a different original Attribute from the monsters you control
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        if !g.is_empty() {
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
        }
    } else if op == 3 {
        // Set 1 "Super Quant" Trap from your Deck
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
        let sel = duel::select_matching_card(d, tp, Some(&setfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        if let Some(sc) = group::get_first(d, &sel) {
            if duel::s_set(d, tp, CardSet::from_card(sc), None, true) > 0 {
                // It can be activated this turn
                let h = effect::get_handler(d, a.e).nil();
                let e1 = effect::create_effect(d, h);
                effect::set_description(d, e1, aux::stringid(ID, 4));
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE, 0);
                effect::set_code(d, e1, EFFECT_TRAP_ACT_IN_SET_TURN);
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                card::register_effect(d, sc, e1, false);
            }
        }
    }
}
