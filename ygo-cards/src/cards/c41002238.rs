//! Kaiser Glider - Golden Burst
use super::prelude::*;

pub const ID: u32 = 41002238;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card from your hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Copy the targeted monster's ATK
    let e2 = effect::create_effect(d, c);
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE | EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET | EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    effect::set_target(d, e2, Some(cost_target(atktg)));
    effect::set_operation(d, e2, Some(operation(atkop)));
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e3, false);
}
fn spcfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&|d, x| spcfilter(d, x, tp)), 1, 1, false, None, NO_EX);
    }
    let sg = duel::select_release_group_cost(d, tp, Some(&|d, x| spcfilter(d, x, tp)), 1, 1, false, None, NO_EX);
    duel::release(d, sg, REASON_COST, None);
    true
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, LOCATION_HAND as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
fn atkfilter(d: &mut Duel, c: CardId, atk: i32) -> bool {
    card::is_faceup(d, c) && !card::is_attack(d, c, &[atk as u32])
}
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    let atk = card::get_attack(d, c);
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && chkc != c && atkfilter(d, chkc, atk);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| atkfilter(d, x, atk)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    duel::select_target(d, tp, Some(&|d, x| atkfilter(d, x, atk)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let c = effect::get_handler(d, e).nil();
    if !(card::is_relate_to_effect(d, c, e) && card::is_faceup(d, c)) {
        return;
    }
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, e) && card::is_faceup(d, tc) {
        // Change ATK
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        let atk = card::get_attack(d, tc) as i64;
        effect::set_value(d, e1, atk);
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        card::register_effect(d, c, e1, false);
    }
}
