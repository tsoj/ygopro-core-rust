//! Triamid Pulse
use super::prelude::*;

pub const ID: u32 = 96100333;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    card::register_effect(d, c, e1, false);
    // Destroy 1 face-up card on the field
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_count_limit(d, e2, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER_E));
    effect::set_cost(d, e2, Some(cost_target(cost_fn)));
    effect::set_target(d, e2, Some(cost_target(destg)));
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
    // Special Summon 1 Rock monster
    let e3 = effect::clone(d, e2);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    effect::set_cost(d, e3, Some(aux_true()));
    effect::set_target(d, e3, Some(cost_target(sptg)));
    effect::set_operation(d, e3, Some(operation(spop)));
    card::register_effect(d, c, e3, false);
    // Shuffle 3 Field Spells into the Deck
    let e4 = effect::clone(d, e2);
    effect::set_description(d, e4, aux::stringid(ID, 3));
    effect::set_category(d, e4, CATEGORY_TODECK + CATEGORY_DRAW);
    effect::set_cost(d, e4, Some(aux_true()));
    effect::set_target(d, e4, Some(cost_target(tdtg)));
    effect::set_operation(d, e4, Some(operation(tdop)));
    card::register_effect(d, c, e4, false);
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    (card::is_race(d, c, RACE_ROCK) || card::is_field_spell(d, c)) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 2, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&cfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 2, 2, false, NO_EX).nil();
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_faceup(d, chkc) && chkc != c;
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, cc| card::is_faceup(d, cc)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into());
    }
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, desc);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    let g = duel::select_target(d, tp, Some(&|d, cc| card::is_faceup(d, cc)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, c.into());
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_race(d, c, RACE_ROCK) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.rescon(sg,e,tp)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let e = e.nil();
    (duel::is_existing_matching_card(d, Some(&|d, cc| spfilter(d, cc, e, tp)), tp, LOCATION_GRAVE, 0, 1, sg.into()), false)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && spfilter(d, chkc, a.e, tp);
    }
    let cg = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_GRAVE, 0, NO_EX);
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && matches!(aux::select_unselect_group(d, &cg, Some(a.e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false), Err(true));
    }
    duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, effect::get_description(d, a.e));
    let rg = aux::select_unselect_group(d, &cg, Some(a.e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, false).unwrap_or_default();
    duel::remove(d, rg, POS_FACEUP as u8, REASON_COST, None, None);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, tp, Some(&|d, cc| spfilter(d, cc, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    let n = g.as_ref().map_or(0, |x| x.len()) as u8;
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, n, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_field_spell(d, c) && card::is_able_to_deck(d, c)
}
/// `s.rescon2(sg,e,tp)`
fn rescon2(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    (duel::is_existing_matching_card(d, Some(&tdfilter), tp, LOCATION_GRAVE, 0, 1, sg.into()), false)
}
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && tdfilter(d, chkc);
    }
    let cg = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_GRAVE, 0, NO_EX);
    let _sg = duel::get_matching_group(d, Some(&tdfilter), tp, LOCATION_GRAVE, 0, NO_EX);
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 1)
            && matches!(aux::select_unselect_group(d, &cg, Some(a.e), tp, Some(2), Some(2), Some(&rescon2), 0, tp, None, None, None, false), Err(true));
    }
    duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, effect::get_description(d, a.e));
    let rg = aux::select_unselect_group(d, &cg, Some(a.e), tp, Some(2), Some(2), Some(&rescon2), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, false).unwrap_or_default();
    duel::remove(d, rg, POS_FACEUP as u8, REASON_COST, None, None);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    let g = duel::select_target(d, tp, Some(&tdfilter), tp, LOCATION_GRAVE, 0, 1, 3, false, NO_EX).nil();
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    true
}
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let Some(tg) = duel::get_target_cards(d, a.e) else {
        return;
    };
    if tg.is_empty() {
        return;
    }
    duel::send_to_deck(d, tg, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
    let g = duel::get_operated_group(d);
    if group::is_exists(d, &g, &|d, cc| card::is_location(d, cc, LOCATION_DECK), 1, NO_EX) {
        duel::shuffle_deck(d, tp);
    }
    let ct = group::filter_count(d, &g, &|d, cc| card::is_location(d, cc, LOCATION_DECK | LOCATION_EXTRA), NO_EX);
    if ct > 0 {
        duel::break_effect(d);
        duel::draw(d, tp, 1, REASON_EFFECT);
    }
}
