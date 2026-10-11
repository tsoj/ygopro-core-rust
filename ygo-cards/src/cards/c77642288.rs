//! Gladiator Beast Secutor
use super::prelude::*;

pub const ID: u32 = 77642288;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GLADIATOR], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_PHASE | PHASE_BATTLE);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    aux::gbspcon().call(d, &[Value::Effect(a.e)]).truthy() && card::get_battled_group_count(d, h) > 0
}
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    !card::is_code(d, c, &[ID])
        && card::is_set_card(d, c, &[SET_GLADIATOR as u16])
        && card::is_can_be_special_summoned(d, c, e, 106, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, LOCATION_DECK as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 2 {
        return;
    }
    if !duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, 2, NO_EX) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| filter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, 2, 2, false, NO_EX).nil();
    let _ = group::get_first(d, &g);
    for tc in group::iter(d, &g) {
        let _ = duel::special_summon_step(d, tc, 106, tp, tp, false, false, POS_FACEUP as u8, None);
        let code = card::get_original_code(d, tc);
        card::register_flag_effect(d, tc, code, RESET_EVENT | RESETS_STANDARD_DISABLE, 0, 0, 0, 0);
    }
    duel::special_summon_complete(d);
}
