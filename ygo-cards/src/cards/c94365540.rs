//! Tindangle Base Gardna
use super::prelude::*;

pub const ID: u32 = 94365540;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TINDANGLE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_SPSUM_PARAM, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_target_range(d, e1, POS_FACEUP_DEFENSE, 0);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_condition(d, e1, Some(lua_fn(sp_condition)));
    card::register_effect(d, c, e1, false);
    // Special summon
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_cost(d, e2, Some(cost::self_tribute()));
    effect::set_condition(d, e2, Some(condition(spcon)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e3, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_facedown(d, c)
}
/// `s.condition(e,c)`: `c` is nil when the summon is only checked for the effect's existence.
fn sp_condition(d: &mut Duel, a: Args) -> bool {
    let Some(c) = a.card(1) else { return true };
    let p = card::get_controler(d, c);
    duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0 > 0 && duel::is_existing_matching_card(d, Some(&filter), p, LOCATION_MZONE, 0, 1, NO_EX)
}
fn lkfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_link_monster(d, c)
}
fn cfilter(d: &mut Duel, c: CardId, tp: u8, zone: u32) -> bool {
    let mut seq = card::get_sequence(d, c);
    if card::is_controler(d, c, tp) {
        seq += 16;
    }
    (zone >> seq) & 1 != 0
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let mut zone: u32 = 0;
    let lg = duel::get_matching_group(d, Some(&lkfilter), a.tp, 0, LOCATION_MZONE, NO_EX);
    for tc in group::iter(d, &lg) {
        zone |= card::get_linked_zone(d, tc, None);
    }
    let h = effect::get_handler(d, a.e).nil();
    let eg = a.eg.clone().nil();
    !group::is_contains(&eg, h) && group::is_exists(d, &eg, &|d, c| cfilter(d, c, a.tp, zone), 1, NO_EX)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_TINDANGLE as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some((POS_FACEUP_ATTACK | POS_FACEDOWN_DEFENSE) as u8), None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > -1
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_DECK | LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_DECK | LOCATION_HAND) as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let f = aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, tp));
    let g = duel::select_matching_card(d, tp, Some(&f), tp, LOCATION_DECK | LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    if let Some(tc) = tc {
        if duel::special_summon(d, tc, 0, tp, tp, false, false, (POS_FACEUP_ATTACK | POS_FACEDOWN_DEFENSE) as u8, None) != 0 && card::is_facedown(d, tc) {
            duel::confirm_cards(d, 1 - tp, tc);
        }
    }
}
