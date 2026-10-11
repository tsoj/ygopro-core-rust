//! BF - Twin Shadow
use super::prelude::*;

pub const ID: u32 = 69973414;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_BLACK_WINGED_DRAGON],
    listed_series: &[SET_BLACKWING],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Black-Winged Dragon" or "Blackwing" Synchro monster from the Extra Deck
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Can be activated from the hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 2));
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_TRAP_ACT_IN_HAND);
    effect::set_condition(d, e2, Some(lua_fn(handcon)));
    card::register_effect(d, c, e2, false);
}
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_BLACKWING as u16]) && card::has_level(d, c) && card::is_faceup(d, c) && card::is_able_to_deck_or_extra_as_cost(d, c)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: i64) -> bool {
    duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_type(d, c, TYPE_SYNCHRO)
        && (card::is_set_card(d, c, &[SET_BLACKWING as u16]) || card::is_code(d, c, &[CARD_BLACK_WINGED_DRAGON]))
        && card::is_level(d, c, &[lv as u32])
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_SYNCHRO, tp, false, false, None, None, None)
}
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let Some(e) = e else { return (false, false) };
    let res = sg.len() == 2
        && group::filter_count(d, sg, &|d, c| card::is_type(d, c, TYPE_TUNER), NO_EX) == 1
        && {
            let lv = group::get_sum(d, sg, &|d, c| card::get_level(d, c) as i64);
            duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp, lv)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
        };
    (res, false)
}
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    effect::set_label(d, a.e, &[1]);
    let _ = d;
    true
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::new()), tp, None, None, REASON_SYNCHRO);
    let g = duel::get_matching_group(d, Some(&tdfilter), tp, LOCATION_REMOVED | LOCATION_GRAVE, 0, NO_EX);
    let rs: aux::Rescon = &rescon;
    if a.chk == Some(0) {
        return pg.is_empty() && matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(rs), 0, tp, None, None, None, false), Err(true));
    }
    let dg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(rs), 1, tp, Some(HINTMSG_TODECK as u64), None, None, false).unwrap_or_default();
    let lv = group::get_sum(d, &dg, &|d, c| card::get_level(d, c) as i64);
    effect::set_label(d, e, &[lv]);
    duel::confirm_cards(d, 1 - tp, dg.clone());
    duel::send_to_deck(d, dg, None, SEQ_DECKSHUFFLE as i32, REASON_COST, None);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let lv = effect::get_label(d, e);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, lv)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() && duel::special_summon(d, g.clone(), SUMMON_TYPE_SYNCHRO, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        let f = group::get_first(d, &g).nil();
        card::complete_procedure(d, f);
    }
}
/// `s.handcon(e)`
fn handcon(d: &mut Duel, a: Args) -> bool {
    let p = effect::get_handler_player(d, a.e(0));
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_BLACKWING as u16])), p, LOCATION_MZONE, 0, 2, NO_EX)
}
