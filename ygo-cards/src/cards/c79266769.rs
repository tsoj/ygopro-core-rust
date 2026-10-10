//! Darkness Metal, the Dragon of Dark Steel
use super::prelude::*;

pub const ID: u32 = 79266769;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    card::enable_revive_limit(d, c);
    // Link Summon procedure
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_EFFECT),2,nil,s.spcheck)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_EFFECT, sc, st, Some(tp)))), 2, None, Some(group_check(spcheck)), None, None);
    // Special Summon 1 of your monsters that is banished or in GY
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
}
/// `s.spcheck(g,lc,sumtype,tp)`
fn spcheck(d: &mut Duel, g: &CardSet, _lc: CardId, _sumtype: u64, _tp: u8) -> bool {
    group::check_same_property(d, g, &|d, c| card::get_race(d, c)).0 && group::check_same_property(d, g, &|d, c| card::get_attribute(d, c) as u64).0
}
/// `s.filter(c,e,tp,zone)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, zone: u32) -> bool {
    (card::is_faceup(d, c) || !card::is_location(d, c, LOCATION_REMOVED))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), Some(tp), Some(zone))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    let zone = card::get_linked_zone(d, h, Some(a.tp)) & ZONES_MMZ;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE | LOCATION_REMOVED) && card::is_controler(d, chkc, a.tp) && filter(d, chkc, a.e, a.tp, zone);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, a.e, a.tp, zone)), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, a.tp, Some(&|d, c| filter(d, c, a.e, a.tp, zone)), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d);
    let zone = card::get_linked_zone(d, c, Some(a.tp)) & ZONES_MMZ;
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && zone != 0 && duel::special_summon_step(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, Some(zone)) {
            // Negate its effects
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_DISABLE);
            effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
            card::register_effect(d, tc, e1, false);
            let e2 = effect::create_effect(d, c);
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            effect::set_reset(d, e2, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
            card::register_effect(d, tc, e2, false);
            // Place on bottom of deck if it leaves the field
            let e3 = effect::create_effect(d, c);
            effect::set_description(d, e3, 3301);
            effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e3, EFFECT_LEAVE_FIELD_REDIRECT);
            effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
            effect::set_reset(d, e3, RESET_EVENT | RESETS_REDIRECT, 1);
            effect::set_value(d, e3, LOCATION_DECKBOT as i64);
            card::register_effect(d, tc, e3, false);
            // Cannot special summon link monsters for rest of turn
            let e4 = effect::create_effect(d, c);
            effect::set_description(d, e4, aux::stringid(ID, 1));
            effect::set_type(d, e4, EFFECT_TYPE_FIELD);
            effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
            effect::set_code(d, e4, EFFECT_CANNOT_SPECIAL_SUMMON);
            effect::set_target_range(d, e4, 1, 0);
            effect::set_target(d, e4, Some(lua_fn(splimit)));
            effect::set_reset(d, e4, RESET_PHASE | PHASE_END, 1);
            duel::register_effect(d, e4, a.tp);
        }
    }
    duel::special_summon_complete(d);
}
/// `s.splimit(e,c,sump,sumtype,sumpos,targetp,se)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    if card::is_monster(d, c) {
        card::is_type(d, c, TYPE_LINK)
    } else {
        card::is_original_type(d, c, TYPE_LINK)
    }
}
