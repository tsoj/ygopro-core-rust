//! Mimighoul Charm
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 55733143;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MIMIGHOUL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e0, false);
    // Special Summon 1 random face-down monster from your opponent's Extra Deck, or banish it
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON | CATEGORY_REMOVE);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_FLIP);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_count_limit(d, e1, 2, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(e1_condition)));
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
}
fn spconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_controler(d, c, 1 - tp) && card::is_set_card(d, c, &[SET_MIMIGHOUL as u16])
}
fn e1_condition(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    duel::is_main_phase(d, None) && group::is_exists(d, &eg, &|d, c| spconfilter(d, c, a.tp), 1, NO_EX)
}
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return !card::has_flag_effect(d, c, ID, None);
    }
    card::register_flag_effect(d, c, ID, RESET_CHAIN, 0, 1, 0, 0);
    true
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        let can = duel::is_player_can_special_summon(d, tp, None) || duel::is_player_can_remove(d, tp, None);
        return can && duel::is_existing_matching_card(d, Some(&|d, c| card::is_facedown(d, c)), tp, 0, LOCATION_EXTRA, 1, NO_EX);
    }
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 1 - tp, LOCATION_EXTRA as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - tp, LOCATION_EXTRA as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_facedown(d, c)), tp, 0, LOCATION_EXTRA, NO_EX);
    if g.is_empty() {
        return;
    }
    let rs = group::random_select(d, &g, tp, 1);
    let Some(tc) = group::get_first(d, &rs) else {
        return;
    };
    duel::confirm_cards(d, tp, tc);
    if duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(tc), None).0 > 0
        && card::is_can_be_special_summoned(d, tc, e, 0, tp, false, false, None, None, None)
        && duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None)
    {
        // It cannot activate its effects this turn
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_description(d, e1, 3302);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE | EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_code(d, e1, EFFECT_CANNOT_TRIGGER);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e1, false);
        duel::special_summon_complete(d);
    } else {
        duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
