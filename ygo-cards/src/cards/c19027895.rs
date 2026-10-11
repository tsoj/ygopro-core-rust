//! Salamangreat Revive
use super::prelude::*;

pub const ID: u32 = 19027895;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SALAMANGREAT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 FIRE monster from the GY
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Shuffle monster into the Deck and increase the ATK of another monster
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_ATKCHANGE);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_hint_timing(d, e2, TIMING_DAMAGE_STEP, Some(TIMING_DAMAGE_STEP | TIMINGS_CHECK_MONSTER_E));
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(cond)));
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    effect::set_operation(d, e2, Some(operation(tdop)));
    card::register_effect(d, c, e2, false);
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_attribute(d, c, ATTRIBUTE_FIRE) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && spfilter(d, chkc, a.e, tp);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    let n = g.as_ref().map_or(0, |x| x.len()) as u8;
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, n, tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    aux::stat_change_damage_step_condition(d)
}
fn atkfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_SALAMANGREAT as u16]) && {
        let code = card::get_code(d, c)[0];
        duel::is_existing_target(d, Some(&|d, cc| tdfilter(d, cc, code)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
    }
}
fn tdfilter(d: &mut Duel, c: CardId, code: u32) -> bool {
    card::is_monster(d, c) && card::is_code(d, c, &[code]) && card::is_able_to_deck(d, c)
}
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chkc().is_some() {
        return false;
    }
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| atkfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATKDEF as u64);
    let tc = duel::select_target(d, tp, Some(&|d, c| atkfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    let tcg = tc.clone().nil();
    let first = group::get_first(d, &tcg).nil();
    let code = card::get_code(d, first)[0];
    let tdc = duel::select_target(d, tp, Some(&|d, c| tdfilter(d, c, code)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_TODECK, tdc, 1, tp, 0);
    true
}
fn tdop(d: &mut Duel, a: &EffArgs) {
    let Some(g) = duel::get_target_cards(d, a.e) else {
        return;
    };
    if g.is_empty() {
        return;
    }
    let gy = group::filter(d, &g, &|d, c| card::is_location(d, c, LOCATION_GRAVE), NO_EX);
    let tc = group::get_first(d, &gy);
    let mz = group::filter(d, &g, &|d, c| card::is_location(d, c, LOCATION_MZONE), NO_EX);
    let atkc = group::get_first(d, &mz);
    if let Some(tc) = tc {
        if duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0 {
            if let Some(atkc) = atkc {
                if card::is_faceup(d, atkc) && card::is_controler(d, atkc, a.tp) {
                    let base = card::get_base_attack(d, atkc) as i64;
                    let h = effect::get_handler(d, a.e);
                    card::update_attack(d, atkc, base, Some(RESETS_STANDARD_PHASE_END), h, None);
                }
            }
        }
    }
}
