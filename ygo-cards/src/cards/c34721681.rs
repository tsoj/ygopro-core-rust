//! Heavy Forward
use super::prelude::*;

pub const ID: u32 = 34721681;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_INFINITRACK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Activate 1 of these effects
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_target(d, e2, Some(cost_target(efftg)));
    effect::set_operation(d, e2, Some(operation(effop)));
    card::register_effect(d, c, e2, false);
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_INFINITRACK as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // Search 1 "Infinitrack" monster
    let g = duel::get_matching_group(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, NO_EX);
    if !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0)) {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        duel::send_to_hand(d, &sg, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - a.tp, &sg);
    }
}
fn tgfilter(d: &mut Duel, c: CardId, mc: CardId, tp: u8) -> bool {
    card::is_faceup(d, c)
        && card::is_type(d, c, TYPE_XYZ)
        && card::is_race(d, c, RACE_MACHINE)
        && (card::is_can_change_position(d, c) || card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), Some(REASON_EFFECT)))
}
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let label = effect::get_label(d, a.e);
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_controler(d, chkc, a.tp)
            && tgfilter(d, chkc, c, a.tp)
            && ((label == 1 && card::is_can_change_position(d, chkc))
                || (label == 2 && card::is_can_be_xyz_material(d, c, Some(chkc), Some(a.tp), Some(REASON_EFFECT))));
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| tgfilter(d, x, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local tc=Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,0,1,1,nil,c,tp):GetFirst()
    // (ygoai override lua_fixes/c34721681.lua: the original passed no c,tp to s.tgfilter here, so the
    // filter indexed a nil mc for a monster that cannot change position, a Lua error in the core)
    let g = duel::select_target(d, a.tp, Some(&|d, x| tgfilter(d, x, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g).nil();
    let b1 = card::is_can_change_position(d, tc);
    let b2 = card::is_can_be_xyz_material(d, c, Some(tc), Some(a.tp), Some(REASON_EFFECT));
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]).unwrap_or(0) as i64;
    effect::set_label(d, a.e, &[op]);
    if op == 1 {
        effect::set_category(d, a.e, CATEGORY_POSITION);
        duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(tc.into()), 1, a.tp, 0);
    } else {
        effect::set_category(d, a.e, 0);
    }
    true
}
fn effop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if !card::is_relate_to_effect(d, tc, a.e) || card::is_immune_to_effect(d, tc, a.e) {
        return;
    }
    let op = effect::get_label(d, a.e);
    let c = effect::get_handler(d, a.e).nil();
    if op == 1 {
        // Change its battle position
        duel::change_position(d, tc, POS_FACEUP_DEFENSE as u8, Some(0), Some(POS_FACEUP_ATTACK as u8), Some(0), false);
    } else if op == 2 && card::is_relate_to_effect(d, c, a.e) && card::is_can_be_xyz_material(d, c, Some(tc), Some(a.tp), Some(REASON_EFFECT)) {
        // Attach this card to it as material
        duel::overlay(d, tc, c, false);
    }
}
