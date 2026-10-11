//! Rebirth of the Seventh Emperors
use super::prelude::*;

pub const ID: u32 = 83888009;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NUMBER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 of your banished Xyz Monsters
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_LEAVE_GRAVE + CATEGORY_DAMAGE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn xyz_filter(d: &mut Duel, c: CardId) -> bool {
    card::is_type(d, c, TYPE_XYZ)
}
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    effect::set_label(d, a.e, &[1]);
    let sg = duel::get_matching_group(d, Some(&xyz_filter), a.tp, LOCATION_MZONE, 0, NO_EX);
    if a.chk == Some(0) {
        return !sg.is_empty() && group::filter_count(d, &sg, &|d, c| card::is_releasable(d, c, None, None), NO_EX) == sg.len();
    }
    let ct = duel::release(d, sg, REASON_COST, None);
    duel::set_target_param(d, (ct + 1) as u32);
    true
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_REMOVED) && spfilter(d, chkc, e, tp);
    }
    if a.chk == Some(0) {
        let sg = duel::get_matching_group(d, Some(&xyz_filter), tp, LOCATION_MZONE, 0, NO_EX);
        let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
        if effect::get_label(d, e) == 1 {
            ft = duel::get_m_zone_count(d, tp, (&sg).into(), None, None, None).0;
        }
        effect::set_label(d, e, &[0]);
        return ft > 0 && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    effect::set_label(d, e, &[0]);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, tp, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, PLAYER_ALL as u8, 300);
    true
}
/// `s.attachfilter(c)`
fn attachfilter(d: &mut Duel, c: CardId) -> bool {
    let no = crate::card_info(d, c).xyz_number;
    card::is_set_card(d, c, &[SET_NUMBER as u16]) && card::is_type(d, c, TYPE_XYZ) && matches!(no, Some(n) if (101..=107).contains(&n)) && card::is_faceup(d, c)
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let tc_opt = duel::first_target(d);
    let ct = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
    let ex = match tc_opt {
        Some(t) => t.into(),
        None => NO_EX,
    };
    let nf = aux::necro_valley_filter(attachfilter);
    let g = duel::get_matching_group(d, Some(&nf), tp, LOCATION_REMOVED | LOCATION_GRAVE, 0, ex);
    let tc = tc_opt.nil();
    if card::is_relate_to_effect(d, tc, e)
        && duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0
        && ct > 0
        && !g.is_empty()
        && !card::is_immune_to_effect(d, tc, e)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
    {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_XMATERIAL as u64);
        let mg = group::select(d, &g, tp, 1, ct as u16, false, NO_EX).nil();
        if !mg.is_empty() {
            duel::hint_selection(d, mg.clone(), true);
            duel::break_effect(d);
            duel::overlay(d, tc, mg, false);
        }
    }
    if !effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // During the End Phase of the turn you activated this card, each player takes 300 damage for each card in their hand
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_operation(d, e1, Some(operation(damop)));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, tp);
}
fn damop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    let n1 = duel::get_field_group_count(d, tp, LOCATION_HAND, 0);
    duel::damage(d, tp, n1 as i64 * 300, REASON_EFFECT, true, None);
    let n2 = duel::get_field_group_count(d, 1 - tp, LOCATION_HAND, 0);
    duel::damage(d, 1 - tp, n2 as i64 * 300, REASON_EFFECT, true, None);
    duel::rd_complete(d);
}
