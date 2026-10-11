//! Artmage Finmel
use super::prelude::*;

pub const ID: u32 = 34541940;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ARTMAGE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Your opponent cannot target Level 6 or lower "Artmage" monsters you control with card effects
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_level_below(d, c, 6) && card::is_set_card(d, c, &[SET_ARTMAGE as u16]))));
    effect::set_value_fn(d, e1, aux::tgoval());
    card::register_effect(d, c, e1, false);
    // Special Summon this card from your hand, then you can draw 1 card
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_DRAW);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
    // Negate the effects of all face-up monsters your opponent currently controls, also their current ATK become halved until the end of this turn
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_DISABLE + CATEGORY_ATKCHANGE);
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_hint_timing(d, e3, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    effect::set_condition(d, e3, Some(condition(e3_cond)));
    effect::set_target(d, e3, Some(cost_target(distg)));
    effect::set_operation(d, e3, Some(operation(disop)));
    card::register_effect(d, c, e3, false);
}
fn is_artmage_faceup(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_ARTMAGE as u16])
}
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&is_artmage_faceup), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_DRAW, None, 1, a.tp, 1);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e)
        && duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0
        && duel::is_player_can_draw(d, a.tp, 1)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2))
    {
        duel::break_effect(d);
        duel::draw(d, a.tp, 1, REASON_EFFECT);
    }
}
fn e3_cond(d: &mut Duel, a: &EffArgs) -> bool {
    if !duel::is_main_phase(d, None) {
        return false;
    }
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, 0, NO_EX);
    group::get_bin_class_count(d, &g, &|d, c| card::get_race(d, c) as i64) >= 3
}
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    true
}
fn disop(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, NO_EX);
    if g.is_empty() {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    for tc in group::iter(d, &g) {
        if card::is_negatable_monster(d, tc) {
            card::negate_effects(d, tc, c, None, false, None);
        }
        // Its ATK becomes halved until the end of this turn
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        let atk = card::get_attack(d, tc);
        effect::set_value(d, e1, lua_int(atk as f64 / 2.0));
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e1, false);
    }
}
