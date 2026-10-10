//! Ojamatch
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 38395123;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_OJAMA, SET_ARMED_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // draw
    let e2 = effect::create_effect(d, c);
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_DRAW);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(drtg)));
    effect::set_operation(d, e2, Some(operation(drop)));
    card::register_effect(d, c, e2, false);
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    effect::set_label(d, a.e, &[1]);
    true
}
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    let cd = card::get_code(d, c)[0];
    card::is_set_card(d, c, &[SET_OJAMA as u16])
        && (card::is_faceup(d, c) || !card::is_on_field(d, c))
        && card::is_able_to_grave_as_cost(d, c)
        && duel::is_existing_matching_card(d, Some(&|d, cc| filter1(d, cc, cd)), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, c.into())
        && duel::is_existing_matching_card(d, Some(&filter2), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, c.into())
}
fn filter1(d: &mut Duel, c: CardId, cd: u32) -> bool {
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_OJAMA as u16]) && !card::is_code(d, c, &[cd]) && card::is_able_to_hand(d, c, None)
}
fn filter2(d: &mut Duel, c: CardId) -> bool {
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_ARMED_DRAGON as u16]) && card::is_able_to_hand(d, c, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        if effect::get_label(d, a.e) != 1 {
            return false;
        }
        effect::set_label(d, a.e, &[0]);
        return duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c, a.tp)), a.tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, effect::get_handler(d, a.e).into());
    }
    effect::set_label(d, a.e, &[0]);
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| cfilter(d, c, a.tp)), a.tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, 1, false, effect::get_handler(d, a.e).into()).nil();
    let first = group::get_first(d, &g);
    effect::set_label_object(d, a.e, first.map_or(Value::Nil, Value::from));
    duel::send_to_grave(d, g, REASON_COST, None, None);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 2, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let sc = effect::get_label_object(d, a.e).as_card().nil();
    let sc_code = card::get_code(d, sc)[0];
    let g1 = duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, c| filter1(d, c, sc_code))), tp, LOCATION_DECK | LOCATION_GRAVE, 0, sc.into());
    let g2 = duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, c| filter2(d, c))), tp, LOCATION_DECK | LOCATION_GRAVE, 0, sc.into());
    if g1.is_empty() || g2.is_empty() {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let mut g = group::select(d, &g1, tp, 1, 1, false, NO_EX).nil();
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let gg = group::select(d, &g2, tp, 1, 1, false, NO_EX).nil();
    group::add_card(&mut g, &gg);
    if g.len() == 2 && duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) > 0 {
        duel::confirm_cards(d, 1 - tp, g.clone());
        let opg = duel::get_operated_group(d);
        let og = group::filter(d, &opg, &|d, c| card::is_summonable(d, c, true, None, 0, None), NO_EX);
        if !og.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
            duel::break_effect(d);
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SUMMON as u64);
            let sg = group::select(d, &og, tp, 1, 1, false, NO_EX).nil();
            let tc = group::get_first(d, &sg).nil();
            duel::summon(d, tp, tc, true, None, 0, None);
        }
    }
}
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_monster(d, c) && card::is_set_card(d, c, &[SET_OJAMA as u16]) && card::is_able_to_deck(d, c)
}
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_REMOVED) && card::is_controler(d, chkc, a.tp) && tdfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1) && duel::is_existing_target(d, Some(&tdfilter), a.tp, LOCATION_REMOVED, 0, 3, NO_EX);
    }
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, desc);
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    let g = duel::select_target(d, a.tp, Some(&tdfilter), a.tp, LOCATION_REMOVED, 0, 3, 3, false, NO_EX).nil();
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
fn drop(d: &mut Duel, a: &EffArgs) {
    let tg = duel::get_target_cards(d, a.e).unwrap_or_default();
    if tg.is_empty() {
        return;
    }
    duel::send_to_deck(d, tg, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
    let g = duel::get_operated_group(d);
    if group::is_exists(d, &g, &|d, c| card::is_location(d, c, LOCATION_DECK), 1, NO_EX) {
        duel::shuffle_deck(d, a.tp);
    }
    let ct = group::filter_count(d, &g, &|d, c| card::is_location(d, c, LOCATION_DECK | LOCATION_EXTRA), NO_EX);
    if ct > 0 {
        duel::break_effect(d);
        duel::draw(d, a.tp, 1, REASON_EFFECT);
    }
}
