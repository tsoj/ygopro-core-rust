//! D/D/D Abyss King Gilgamesh
use super::prelude::*;

pub const ID: u32 = 9024198;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Link Summon
    link::add_procedure(d, c, Some(mat_filter(|d, x, sc, st, tp| card::is_set_card_ctx(d, x, &[SET_DD as u16], sc, st, Some(tp)))), 2, None, None, None, None);
    // Must be properly summoned before being revived
    card::enable_revive_limit(d, c);
    // Place 2 "D/D" monsters from your Deck to your Pendulum Zones
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(pltg)));
    effect::set_operation(d, e1, Some(operation(plop)));
    card::register_effect(d, c, e1, false);
    // Special summon "D/D" monster from face-up extra deck or GY
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_DESTROYED);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(spcon)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}
// Check for "D/D" pendulum monsters
fn pcfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_DD as u16]) && card::is_type(d, c, TYPE_PENDULUM) && !card::is_forbidden(d, c)
}
// Activation legality
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        let g = duel::get_matching_group(d, Some(&pcfilter), tp, LOCATION_DECK, 0, NO_EX);
        return duel::check_location(d, tp, LOCATION_PZONE, 0)
            && duel::check_location(d, tp, LOCATION_PZONE, 1)
            && group::get_class_count(d, &g, &|d, x| card::get_code(d, x)[0] as i64) >= 2;
    }
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, tp, 1000);
    true
}
// Performing the effect of placing 2 different "D/D" pendulums into the scales
fn plop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    // Cannot Special Summon, except "D/D" monsters
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 2));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_target(d, e1, Some(effect_card_fn(splimit)));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, tp);
    // Place 2 "D/D" monsters in your Pendulum Zones
    let g = duel::get_matching_group(d, Some(&pcfilter), tp, LOCATION_DECK, 0, NO_EX);
    if duel::check_location(d, tp, LOCATION_PZONE, 0)
        && duel::check_location(d, tp, LOCATION_PZONE, 1)
        && group::get_class_count(d, &g, &|d, x| card::get_code(d, x)[0] as i64) >= 2
    {
        let pg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&aux::dncheck()), 1, tp, Some(HINTMSG_TOFIELD as u64), None, None, false).unwrap_or_default();
        if pg.len() != 2 {
            return;
        }
        let order = group::iter(d, &pg);
        let (pc1, pc2) = (order[0], order[1]);
        if duel::move_to_field(d, pc1, tp, tp, LOCATION_PZONE as u16, POS_FACEUP as u8, false, None) {
            if duel::move_to_field(d, pc2, tp, tp, LOCATION_PZONE as u16, POS_FACEUP as u8, false, None) {
                duel::damage(d, tp, 1000, REASON_EFFECT, false, None);
                card::set_status(d, pc2, STATUS_EFFECT_ENABLED, true);
            }
            card::set_status(d, pc1, STATUS_EFFECT_ENABLED, true);
        }
    }
}
// Limit special summons to "D/D" monsters for rest of turn
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    !card::is_set_card(d, c, &[SET_DD as u16])
}
// Check reason why destroyed and if link summoned
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if !(card::is_previous_controler(d, c, tp) && card::is_previous_location(d, c, LOCATION_MZONE)) {
        return false;
    }
    let by_effect = card::is_reason(d, c, REASON_EFFECT);
    let by_battle = !by_effect && card::is_reason(d, c, REASON_BATTLE) && {
        let attacker = duel::get_attacker(d).nil();
        card::is_controler(d, attacker, 1 - tp)
    };
    (by_effect || by_battle) && card::is_link_summoned(d, c) && a.rp == 1 - tp
}
// Check for "D/D" monster in GY or face-up extra deck
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    if card::is_location(d, c, LOCATION_GRAVE) && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return false;
    }
    if card::is_location(d, c, LOCATION_EXTRA) && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 <= 0 {
        return false;
    }
    card::is_set_card(d, c, &[SET_DD as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
// Activation legality
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_GRAVE | LOCATION_EXTRA, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_GRAVE | LOCATION_EXTRA) as i32);
    true
}
// Performing the effect of special summoning a "D/D" monster from face-up extra deck or GY
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(|d, x| spfilter(d, x, e, tp))), tp, LOCATION_GRAVE | LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEDOWN_DEFENSE as u8, None);
    }
}
