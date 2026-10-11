//! Descending Lost Star
use super::prelude::*;

pub const ID: u32 = 42079445;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 Synchro Monster from your GY in Defense Position, but its effects are negated, its Level is reduced by 1, its DEF becomes 0, also its battle position cannot be changed
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    card::register_effect(d, c, e1, false);
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    if !card::is_synchro_monster(d, c) {
        return false;
    }
    let lv = card::get_level(d, c) as i64;
    card::assume_property(d, c, ASSUME_LEVEL, (lv - 1) as u64);
    card::assume_property(d, c, ASSUME_DEFENSE, 0);
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && spfilter(d, chkc, e, tp);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let tc = duel::first_target(d).nil();
    if !card::is_relate_to_effect(d, tc, e) {
        return;
    }
    let lv = card::get_level(d, tc) as i64;
    card::assume_property(d, tc, ASSUME_LEVEL, (lv - 1) as u64);
    card::assume_property(d, tc, ASSUME_DEFENSE, 0);
    if duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None) {
        let c = effect::get_handler(d, e).nil();
        // Its effects are negated
        card::negate_effects(d, tc, c, None, false, None);
        // Its Level is reduced by 1
        let e1 = effect::create_effect(d, c);
        effect::set_description(d, e1, 3313);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
        effect::set_value(d, e1, -1);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e1, false);
        // Its DEF becomes 0
        let e2 = effect::clone(d, e1);
        effect::set_code(d, e2, EFFECT_SET_DEFENSE_FINAL);
        effect::set_value(d, e2, 0);
        card::register_effect(d, tc, e2, false);
        // Also its battle position cannot be changed
        let e3 = effect::clone(d, e1);
        effect::set_code(d, e3, EFFECT_CANNOT_CHANGE_POSITION);
        card::register_effect(d, tc, e3, false);
    }
    duel::special_summon_complete(d);
}
