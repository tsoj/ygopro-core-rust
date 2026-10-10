//! Shaddoll Zefracore
use super::prelude::*;

pub const ID: u32 = 95401059;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_SHADDOLL, SET_ZEFRA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pendulum summon
    pendulum::add_procedure(d, c, None, None);
    // splimit
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_range(d, e2, LOCATION_PZONE);
    effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CANNOT_NEGATE, 0);
    effect::set_target_range(d, e2, 1, 0);
    effect::set_target(d, e2, Some(lua_fn(splimit)));
    card::register_effect(d, c, e2, false);
    // spsummon
    let e3 = effect::create_effect(d, c);
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    effect::set_condition(d, e3, Some(condition(condition1)));
    effect::set_target(d, e3, Some(cost_target(target)));
    effect::set_operation(d, e3, Some(operation(operation_fn)));
    card::register_effect(d, c, e3, false);
    let e4 = effect::clone(d, e3);
    effect::set_code(d, e4, EVENT_TO_GRAVE);
    effect::set_condition(d, e4, Some(condition(|_d, _a| true)));
    card::register_effect(d, c, e4, false);
}
/// `s.splimit(e,c,sump,sumtype,sumpos,targetp)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    if card::is_set_card(d, c, &[SET_SHADDOLL as u16]) || card::is_set_card(d, c, &[SET_ZEFRA as u16]) {
        return false;
    }
    let sumtype = a.int(3) as u32;
    (sumtype & SUMMON_TYPE_PENDULUM) == SUMMON_TYPE_PENDULUM
}
fn condition1(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_pendulum_summoned(d, h)
}
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_ZEFRA as u16]) && !card::is_code(d, c, &[ID]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_PZONE) && card::is_controler(d, chkc, tp) && filter(d, chkc, e, tp);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_PZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_PZONE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
