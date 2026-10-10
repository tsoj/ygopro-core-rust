//! Pot of Prosperity
use super::prelude::*;

pub const ID: u32 = 84211599;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // excavate and add
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
    aux::global_check(d, ID, |d| {
        let ge1 = effect::create_effect(d, c);
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, ge1, EVENT_DRAW);
        effect::set_operation(d, ge1, Some(operation(checkop)));
        duel::register_effect(d, ge1, 0);
    });
}
fn checkop(d: &mut Duel, a: &EffArgs) {
    if (a.r & REASON_EFFECT) > 0 {
        duel::register_flag_effect(d, a.ep, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    }
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_flag_effect(d, a.tp, ID) == 0;
    }
    // cannot draw
    let e1 = effect::create_effect(d, effect::get_handler(d, a.e).nil());
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CANNOT_DRAW);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT + EFFECT_FLAG_OATH, 0);
    effect::set_description(d, e1, aux::stringid(ID, 2));
    effect::set_target_range(d, e1, 1, 0);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
    true
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove_as_cost(d, c, Some(POS_FACEDOWN as u8))), a.tp, LOCATION_EXTRA, 0, NO_EX);
    let dg = duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0);
    let t3 = g.len() >= 3 && dg >= 3 && {
        let top = duel::get_decktop_group(d, a.tp, 3);
        group::filter_count(d, &top, &|d, c| card::is_able_to_hand(d, c, None), NO_EX) > 0
    };
    let t6 = g.len() >= 6 && dg >= 6 && {
        let top = duel::get_decktop_group(d, a.tp, 6);
        group::filter_count(d, &top, &|d, c| card::is_able_to_hand(d, c, None), NO_EX) > 0
    };
    if a.chk == Some(0) {
        return t3 || t6;
    }
    let mut ann = Vec::new();
    if t3 {
        ann.push(3);
    }
    if t6 {
        ann.push(6);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 0));
    let (num, _) = duel::announce_number(d, a.tp, &ann).nil();
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    let rg = group::select(d, &g, a.tp, num as u16, num as u16, false, NO_EX).nil();
    duel::remove(d, rg, POS_FACEDOWN as u8, REASON_COST, None, None);
    effect::set_label(d, a.e, &[num as i64]);
    duel::set_target_player(d, a.tp);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, 0, LOCATION_DECK as i32);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER]);
    let p = info[0].int() as u8;
    let n = effect::get_label(d, a.e);
    duel::confirm_decktop(d, p, n as u32);
    let mut g = duel::get_decktop_group(d, p, n as u32);
    if !g.is_empty() {
        duel::disable_shuffle_check(d, true);
        duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_ATOHAND as u64);
        let sel = group::select(d, &g, p, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sel).nil();
        if card::is_able_to_hand(d, sc, None) {
            duel::send_to_hand(d, sc, None, REASON_EFFECT, None);
            duel::confirm_cards(d, 1 - p, sc);
            duel::shuffle_hand(d, p);
            group::remove_card(&mut g, &CardSet::from_card(sc));
            duel::move_to_deck_bottom(d, &g);
            duel::sort_deckbottom(d, p, p, g.len() as u16);
        } else {
            duel::send_to_grave(d, sc, REASON_RULE, None, None);
        }
    }
    // halve battle damage
    let e1 = effect::create_effect(d, effect::get_handler(d, a.e).nil());
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CHANGE_DAMAGE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e1, 0, 1);
    effect::set_value_fn(d, e1, lua_fn(damval));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
    // (aux.RegisterClientHint returns at once when the card is nil)
    if let Some(h) = effect::get_handler(d, a.e) {
        aux::register_client_hint(d, h, 0, a.tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
    }
}
fn damval(_d: &mut Duel, a: Args) -> i64 {
    // math.floor(val/2)
    (a.int(2) as f64 / 2.0).floor() as i64
}
