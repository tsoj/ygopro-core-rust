//! Enemy Controller
use super::prelude::*;

pub const ID: u32 = 98045062;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, TIMING_BATTLE_PHASE | TIMING_STANDBY_PHASE, Some(TIMING_BATTLE_PHASE));
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(9)
    effect::set_label(d, a.e, &[9]);
    true
}
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return Duel.GetMZoneCount(tp,c,tp,LOCATION_REASON_CONTROL)>0
    //     and Duel.IsExistingTarget(aux.FaceupFilter(Card.IsControlerCanBeChanged,true),tp,0,LOCATION_MZONE,1,c)
    duel::get_m_zone_count(d, tp, c.into(), Some(tp), Some(LOCATION_REASON_CONTROL), None).0 > 0
        && duel::is_existing_target(
            d,
            Some(&aux::faceup_filter(|d, c| card::is_controler_can_be_changed(d, c, true, None))),
            tp,
            0,
            LOCATION_MZONE,
            1,
            c.into(),
        )
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if let Some(chkc) = a.chkc() {
        if !(card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc) && card::is_controler(d, chkc, 1 - tp)) {
            return false;
        }
        if effect::get_label(d, e) == 1 {
            return card::is_can_change_position(d, chkc);
        } else {
            return card::is_controler_can_be_changed(d, chkc, false, None);
        }
    }
    let b1 = duel::is_existing_target(d, Some(&aux::faceup_filter(|d, c| card::is_can_change_position(d, c))), tp, 0, LOCATION_MZONE, 1, NO_EX);
    let b2;
    if effect::get_label(d, e) == 9 {
        b2 = aux::check_release_group_cost(d, tp, Some(&|d, c| cfilter(d, c, tp)), 1, 1, false, None, NO_EX);
    } else {
        b2 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, Some(tp), Some(LOCATION_REASON_CONTROL), None).0 > 0
            && duel::is_existing_target(d, Some(&aux::faceup_filter(|d, c| card::is_controler_can_be_changed(d, c, false, None))), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    if a.chk == Some(0) {
        effect::set_label(d, e, &[0]);
        return b1 || b2;
    }
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
    if op == Some(1) {
        effect::set_category(d, e, CATEGORY_POSITION);
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_POSCHANGE as u64);
        let g = duel::select_target(d, tp, Some(&aux::faceup_filter(|d, c| card::is_can_change_position(d, c))), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
        duel::set_operation_info(d, 0, CATEGORY_POSITION, g, 1, 0, 0);
    } else {
        effect::set_category(d, e, CATEGORY_CONTROL);
        if effect::get_label(d, e) == 9 {
            let rg = aux::select_release_group_cost(d, tp, Some(&|d, c| cfilter(d, c, tp)), 1, 1, false, None, NO_EX);
            duel::release(d, rg, REASON_COST, None);
        }
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
        let g = duel::select_target(d, tp, Some(&aux::faceup_filter(|d, c| card::is_controler_can_be_changed(d, c, false, None))), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
        duel::set_operation_info(d, 0, CATEGORY_CONTROL, g, 1, 0, 0);
    }
    // e:SetLabel(op)
    effect::set_label(d, e, &[op.unwrap_or(0) as i64]);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not (tc:IsRelateToEffect(e) and tc:IsFaceup() and tc:IsControler(1-tp)) then return end
    if !(card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && card::is_controler(d, tc, 1 - a.tp)) {
        return;
    }
    if effect::get_label(d, a.e) == 1 {
        // Change that target's battle position
        duel::change_position(d, tc, POS_FACEUP_DEFENSE as u8, Some(0), Some(POS_FACEUP_ATTACK as u8), Some(0), false);
    } else {
        // Take control of that target until the End Phase
        duel::get_control(d, tc, a.tp, PHASE_END as u16, 1, None, None);
    }
}
