//! Eldlixir of Scarlet Sanguine
//! Scripted by AlphaKretin
use super::prelude::*;

pub const ID: u32 = 20612097;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ELDLICH, SET_GOLDEN_LAND], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 Zombie monster from your Deck or GY
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Set 1 "Golden Land" Spell/Trap from your Deck
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SET);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER_E));
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(settg)));
    effect::set_operation(d, e2, Some(operation(setop)));
    card::register_effect(d, c, e2, false);
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, eldlich: bool) -> bool {
    (eldlich || card::is_set_card(d, c, &[SET_ELDLICH as u16])) && card::is_race(d, c, RACE_ZOMBIE) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(tp), Some(0xff))
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let eldlich = duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x) && card::is_set_card(d, x, &[SET_ELDLICH as u16])), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, a.tp, eldlich)), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        // Cannot Special Summon monsters, except Zombie monsters
        let c = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, c);
        effect::set_description(d, e1, aux::stringid(ID, 2));
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
        effect::set_target_range(d, e1, 1, 0);
        effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| !card::is_race(d, c, RACE_ZOMBIE))));
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        duel::register_effect(d, e1, a.tp);
    }
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    let eldlich = duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x) && card::is_set_card(d, x, &[SET_ELDLICH as u16])), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&aux::necro_valley_filter(|d, x| spfilter(d, x, a.e, a.tp, eldlich))),
        a.tp,
        LOCATION_DECK | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    if !g.is_empty() {
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_GOLDEN_LAND as u16]) && card::is_s_setable(d, c, false, None)
}
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
fn setop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::s_set(d, a.tp, g, None, true);
    }
}
