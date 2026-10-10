//! I:P Masquerena
use super::prelude::*;

pub const ID: u32 = 65741786;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    card::enable_revive_limit(d, c);
    // Link summon procedure
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| matfilter(d, c, sc, st, tp))), 2, Some(2), None, None, None);
    // Link summon 1 link monster during opponent's main phase
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_MAIN_END));
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // A link monster using this card cannot be destroyed by opponent's card effects
    let e2 = effect::create_effect(d, c);
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_BE_MATERIAL);
    effect::set_condition(d, e2, Some(condition(indcon)));
    effect::set_operation(d, e2, Some(operation(indop)));
    card::register_effect(d, c, e2, false);
}
/// `s.matfilter(c,lc,st,tp)`
fn matfilter(d: &mut Duel, c: CardId, lc: Option<CardId>, st: u64, tp: u8) -> bool {
    !card::is_type_ctx(d, c, TYPE_LINK, lc, st, Some(tp))
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp) && duel::is_main_phase(d, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_link_summonable(d, c, Some(CardSet::from_card(h)), None, 0, 0)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) || card::is_controler(d, c, 1 - a.tp) {
        return;
    }
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_link_summonable(d, x, Some(CardSet::from_card(c)), None, 0, 0)), a.tp, LOCATION_EXTRA, 0, NO_EX);
    if !g.is_empty() {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &sg).nil();
        // Duel.LinkSummon(tp,sg:GetFirst(),c): c is the card that must be used
        duel::link_summon(d, a.tp, tc, Some(CardSet::from_card(c)), None, 0, 0);
    }
}
/// `s.indcon(e,tp,eg,ep,ev,re,r,rp)`
fn indcon(_d: &mut Duel, a: &EffArgs) -> bool {
    a.r == REASON_LINK
}
/// `s.indop(e,tp,eg,ep,ev,re,r,rp)`
fn indop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let rc = card::get_reason_card(d, c);
    // Cannot be destroyed by opponent's card effects
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, 3060);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    effect::set_value_fn(d, e1, lua_fn(indval));
    effect::set_owner_player(d, e1, a.ep);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, rc.nil(), e1, true);
}
/// `s.indval(e,re,rp)`: `rp==1-e:GetOwnerPlayer()`
fn indval(d: &mut Duel, a: Args) -> bool {
    let owner = effect::get_owner_player(d, a.e(0)) as i64;
    a.int(2) == 1 - owner
}
