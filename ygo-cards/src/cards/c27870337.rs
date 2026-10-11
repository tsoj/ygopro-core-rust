//! Solfachord Elegance
//! Scripted by Hel
use super::prelude::*;

pub const ID: u32 = 27870337;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SOLFACHORD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_LIMIT_ZONE, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    effect::set_value_fn(d, e1, lua_fn(|d, a| zones(d, a)));
    card::register_effect(d, c, e1, false);
}
fn zones(d: &mut Duel, a: Args) -> i64 {
    let e = a.e(0);
    let tp = a.p(1);
    let mut zone: i64 = 0xff; // all S/T zones
    let place_g = duel::get_matching_group(d, Some(&plfilter), tp, LOCATION_DECK, 0, NO_EX);
    let left_pend = duel::check_location(d, tp, LOCATION_PZONE, 0);
    let right_pend = duel::check_location(d, tp, LOCATION_PZONE, 1);
    let b1 = duel::check_pendulum_zones(d, tp) && !place_g.is_empty();
    let b2 = duel::is_existing_matching_card(d, Some(&plfilter), tp, LOCATION_HAND, 0, 1, NO_EX) && left_pend && right_pend && select_ok(d, &place_g, e, tp);
    if b1 && !b2 && !(left_pend && right_pend) {
        zone = 0xe; // S/T zones without the right and left most ones
    }
    zone
}
fn plfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_SOLFACHORD as u16]) && card::is_type(d, c, TYPE_PENDULUM) && !card::is_forbidden(d, c)
}
fn notforbidden(d: &mut Duel, c: CardId) -> bool {
    !card::is_forbidden(d, c)
}
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let even = group::is_exists(d, sg, &|d, c| card::is_even_scale(d, c), 1, NO_EX);
    let odd = group::is_exists(d, sg, &|d, c| card::is_odd_scale(d, c), 1, NO_EX);
    (even && odd, false)
}
/// `aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0,tp,nil)` as a boolean (chk == 0)
fn select_ok(d: &mut Duel, g: &CardSet, e: EffectId, tp: u8) -> bool {
    let rs: aux::Rescon = &rescon;
    matches!(aux::select_unselect_group(d, g, Some(e), tp, Some(2), Some(2), Some(rs), 0, tp, None, None, None, false), Err(true))
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let place_g = duel::get_matching_group(d, Some(&plfilter), tp, LOCATION_DECK, 0, NO_EX);
    let toextra_g = duel::get_matching_group(d, Some(&notforbidden), tp, LOCATION_PZONE, 0, NO_EX);
    let b1 = duel::check_pendulum_zones(d, tp) && !place_g.is_empty();
    let b2 = duel::is_existing_matching_card(d, Some(&plfilter), tp, LOCATION_HAND, 0, 1, NO_EX)
        && duel::check_location(d, tp, LOCATION_PZONE, 0)
        && duel::check_location(d, tp, LOCATION_PZONE, 1)
        && select_ok(d, &place_g, e, tp);
    let b3 = duel::is_player_can_draw(d, tp, 2) && select_ok(d, &toextra_g, e, tp);
    if a.chk == Some(0) {
        return b1 || b2 || b3;
    }
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2)), (b3, aux::stringid(ID, 3))]).nil();
    effect::set_label(d, e, &[op as i64]);
    if op == 1 {
        effect::set_category(d, e, 0);
    } else if op == 2 {
        effect::set_category(d, e, CATEGORY_TOEXTRA);
        duel::set_operation_info(d, 0, CATEGORY_TOEXTRA, None, 1, tp, LOCATION_HAND as i32);
    } else if op == 3 {
        effect::set_category(d, e, CATEGORY_TOEXTRA + CATEGORY_DRAW);
        duel::set_target_player(d, tp);
        duel::set_target_param(d, 2);
        duel::set_operation_info(d, 0, CATEGORY_TOEXTRA, None, 2, tp, LOCATION_PZONE as i32);
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 2);
    }
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let op = effect::get_label(d, e);
    if op == 1 {
        // Place 1 "Solfachord" Pendulum Monster from your Deck in your Pendulum Zone
        if !duel::check_pendulum_zones(d, tp) {
            return;
        }
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
        let g = duel::select_matching_card(d, tp, Some(&plfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        if let Some(sc) = group::get_first(d, &g) {
            duel::move_to_field(d, sc, tp, tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
        }
    } else if op == 2 {
        // Place 2 "Solfachord" Pendulum Monsters from your Deck in your Pendulum Zone
        duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 4));
        let g = duel::select_matching_card(d, tp, Some(&plfilter), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        if !(duel::send_to_extra_p(d, &g, Some(tp), REASON_EFFECT, None) > 0 && duel::check_location(d, tp, LOCATION_PZONE, 0) && duel::check_location(d, tp, LOCATION_PZONE, 1)) {
            return;
        }
        let place_g = duel::get_matching_group(d, Some(&plfilter), tp, LOCATION_DECK, 0, NO_EX);
        if place_g.len() < 2 {
            return;
        }
        let rs: aux::Rescon = &rescon;
        let sg = aux::select_unselect_group(d, &place_g, Some(e), tp, Some(2), Some(2), Some(rs), 1, tp, Some(HINTMSG_TOFIELD as u64), None, None, false).unwrap_or_default();
        if sg.len() == 2 {
            duel::break_effect(d);
            let order = group::iter(d, &sg);
            duel::move_to_field(d, order[0], tp, tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
            duel::move_to_field(d, order[1], tp, tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
        }
    } else if op == 3 {
        // Add 2 Pendulum Scales to the Extra Deck face-up and draw 2 cards
        let toextra_g = duel::get_matching_group(d, Some(&notforbidden), tp, LOCATION_PZONE, 0, NO_EX);
        if toextra_g.len() == 2 && select_ok(d, &toextra_g, e, tp) && duel::send_to_extra_p(d, &toextra_g, Some(tp), REASON_EFFECT, None) > 0 {
            let og = duel::get_operated_group(d);
            if group::filter_count(d, &og, &|d, c| card::is_location(d, c, LOCATION_EXTRA), NO_EX) == 2 {
                let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
                duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
            }
        }
    }
}
