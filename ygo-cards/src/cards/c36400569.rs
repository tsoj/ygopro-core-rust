//! Service Puppet Play
use super::prelude::*;

pub const ID: u32 = 36400569;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GIMMICK_PUPPET], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Take control of your opponent's monster(s)
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_CONTROL);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Special Summon 1 Xyz Monster from either GY to either field in Defense Position
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(lua_fn(e2cond)));
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}

fn e2cond(d: &mut Duel, a: Args) -> bool {
    let exccon = aux::exccon().call(d, &[Value::Effect(a.e(0))]).truthy();
    exccon && duel::is_existing_matching_card(d, Some(&gpxyzfilter), a.p(1), LOCATION_MZONE, 0, 1, NO_EX)
}

fn gpxyzfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_GIMMICK_PUPPET as u16]) && card::is_type(d, c, TYPE_XYZ) && card::is_faceup(d, c)
}

fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_controler(d, chkc, 1 - a.tp)
            && card::is_controler_can_be_changed(d, chkc, false, None);
    }
    let mut ct = duel::get_matching_group_count(d, Some(&gpxyzfilter), a.tp, LOCATION_MZONE, 0, NO_EX) as i64;
    if a.chk == Some(0) {
        return ct > 0
            && duel::is_existing_target(d, Some(&|d, c| card::is_controler_can_be_changed(d, c, false, None)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    let zc = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, Some(a.tp), Some(LOCATION_REASON_CONTROL), None).0 as i64;
    ct = ct.min(zc);
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONTROL as u64);
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_controler_can_be_changed(d, c, false, None)), a.tp, 0, LOCATION_MZONE, 1, ct as u16, false, NO_EX)
        .nil();
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(g.clone()), g.len() as u8, a.tp, 0);
    true
}

fn activate(d: &mut Duel, a: &EffArgs) {
    let tg = duel::get_target_cards(d, a.e).nil();
    if !tg.is_empty() {
        duel::get_control(d, tg, a.tp, PHASE_END as u16, 1, None, None);
    }
}

fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ftpl: i64, ftopp: i64) -> bool {
    card::is_type(d, c, TYPE_XYZ)
        && ((ftpl > 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None))
            || (ftopp > 0
                && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), Some(1 - tp), None)))
}

fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let ftpl = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 as i64;
    let ftopp = duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, Some(a.tp), None, None).0 as i64;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && spfilter(d, chkc, a.e, a.tp, ftpl, ftopp);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, ftpl, ftopp)), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp, ftpl, ftopp)), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, a.tp, 0);
    true
}

fn spop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::get_first_target(d).first().copied().nil();
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    let b1 = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && card::is_can_be_special_summoned(d, tc, a.e, 0, a.tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None);
    let b2 = duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, Some(a.tp), None, None).0 > 0
        && card::is_can_be_special_summoned(d, tc, a.e, 0, a.tp, false, false, Some(POS_FACEUP_DEFENSE as u8), Some(1 - a.tp), None);
    if !(b1 || b2) {
        return;
    }
    let op = if b1 && b2 {
        duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))])
    } else if b1 {
        Some(1)
    } else {
        Some(2)
    };
    let target_player = if op == Some(1) { a.tp } else { 1 - a.tp };
    duel::special_summon(d, tc, 0, a.tp, target_player, false, false, POS_FACEUP_DEFENSE as u8, None);
}
