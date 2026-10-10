//! Codebreaker Virus Swordsman
use super::prelude::*;

pub const ID: u32 = 84121193;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CODEBREAKER], listed_names: &[8662794], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    card::enable_revive_limit(d, c);
    // Link summon procedure
    link::add_procedure(d, c, Some(mat_filter(|d, x, sc, st, tp| card::is_type_ctx(d, x, TYPE_EFFECT, sc, st, Some(tp)))), 2, Some(2), None, None, None);
    // Special summon 1 "Codebreaker Zero Day" to either field from your deck
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
    // Register the fact it was sent to GY
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_operation(d, e2, Some(operation(tgop)));
    card::register_effect(d, c, e2, false);
    // Special summon itself from GY
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_range(d, e3, LOCATION_GRAVE);
    effect::set_condition(d, e3, Some(condition(spcon)));
    effect::set_target(d, e3, Some(cost_target(sptg)));
    effect::set_operation(d, e3, Some(operation(spop)));
    card::register_effect(d, c, e3, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::get_mutual_linked_group_count(d, c) > 0
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, p: u8, zones: u32) -> bool {
    card::is_code(d, c, &[8662794]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(p), Some(zones))
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let zones_tp = aux::get_mm_zones_pointed_to(d, tp, None, None, None, None);
    let zones_opp = aux::get_mm_zones_pointed_to(d, 1 - tp, None, None, None, None);
    let loc = LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp, tp, zones_tp)), tp, loc, 0, 1, NO_EX)
            || duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp, 1 - tp, zones_opp)), tp, loc, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, loc as i32);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    let zt = aux::get_mm_zones_pointed_to(d, tp, None, None, None, None);
    let zo = aux::get_mm_zones_pointed_to(d, 1 - tp, None, None, None, None);
    let zone_of = |p: u8| if p == tp { zt } else { zo };
    let loc = LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g1 = duel::get_matching_group(
        d,
        Some(&|d, c| aux::necro_valley_filter(|d, x| spfilter(d, x, e, tp, tp, zt))(d, c)),
        tp,
        loc,
        0,
        NO_EX,
    );
    let g2 = duel::get_matching_group(
        d,
        Some(&|d, c| aux::necro_valley_filter(|d, x| spfilter(d, x, e, tp, 1 - tp, zo))(d, c)),
        tp,
        loc,
        0,
        NO_EX,
    );
    let mut g = g1;
    group::add_card(&mut g, &g2);
    let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
    if let Some(tc) = group::get_first(d, &sel) {
        let p = if spfilter(d, tc, e, tp, tp, zt) && spfilter(d, tc, e, tp, 1 - tp, zo) {
            if duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
                1 - tp
            } else {
                tp
            }
        } else if spfilter(d, tc, e, tp, tp, zt) {
            tp
        } else {
            1 - tp
        };
        duel::special_summon(d, tc, 0, tp, p, false, false, POS_FACEUP as u8, Some(zone_of(p)));
    }
}
fn tgop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_reason(d, c, REASON_DESTROY) && a.rp != a.tp {
        card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    }
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, c, ID) > 0
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) != 0 {
        // Banish it if it leaves the field
        let e1 = effect::create_effect(d, c);
        effect::set_description(d, e1, 3300);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
        effect::set_value(d, e1, LOCATION_REMOVED as i64);
        card::register_effect(d, c, e1, true);
    }
}
