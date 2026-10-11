//! Fluffal Mouse
use super::prelude::*;

pub const ID: u32 = 6142488;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FRIGHTFUR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_property(d, e1, EFFECT_FLAG_NO_TURN_RESET, 0);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SPSUMMON, lua_fn(counterfilter));
}
fn counterfilter(d: &mut Duel, a: Args) -> bool {
    let c = a.c(0);
    card::is_set_card(d, c, &[SET_FRIGHTFUR as u16]) || !card::is_summon_location(d, c, LOCATION_EXTRA)
}
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_custom_activity_count(d, ID, a.tp, ACTIVITY_SPSUMMON) == 0;
    }
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_target(d, e1, Some(effect_card_fn(splimit)));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
    aux::register_client_hint(d, h, 0, a.tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
    // lizard check
    aux::add_temp_lizard_check(d, h, a.tp, Some(effect_card_fn(lizfilter)), None, None, None, None);
    true
}
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    !card::is_set_card(d, c, &[SET_FRIGHTFUR as u16]) && card::is_location(d, c, LOCATION_EXTRA)
}
fn lizfilter(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    !card::is_original_set_card(d, c, &[SET_FRIGHTFUR as u16])
}
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_code(d, c, &[ID]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    if ft <= 0 {
        return;
    }
    if ft > 2 {
        ft = 2;
    }
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, ft as u16, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
