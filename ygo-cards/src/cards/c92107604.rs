//! Runick Fountain
use super::prelude::*;

pub const ID: u32 = 92107604;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RUNICK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Activate "Runick" Quick-Play Spells from your hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_QP_ACT_IN_NTPHAND);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_target_range(d, e2, LOCATION_HAND, 0);
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_RUNICK as u16]))));
    card::register_effect(d, c, e2, false);
    // Return up to 3 "Runick" Quick-Play Spells to the Deck and draw the same number
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_TODECK | CATEGORY_DRAW);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD | EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_DELAY | EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e3, EVENT_CHAINING);
    effect::set_range(d, e3, LOCATION_FZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_condition(d, e3, Some(condition(drcon)));
    effect::set_target(d, e3, Some(cost_target(drtg)));
    effect::set_operation(d, e3, Some(operation(drop_op)));
    card::register_effect(d, c, e3, false);
}
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    if a.rp != a.tp {
        return false;
    }
    let re = a.re.nil();
    let h = effect::get_handler(d, re).nil();
    card::is_set_card(d, h, &[SET_RUNICK as u16])
        && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
        && effect::is_spell_effect(d, re)
        && effect::is_active_type(d, re, TYPE_QUICKPLAY)
}
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_RUNICK as u16]) && card::is_quick_play_spell(d, c) && card::is_able_to_deck(d, c)
}
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && tdfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tdfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX) && duel::is_player_can_draw(d, tp, 0);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    let g = duel::select_target(d, tp, Some(&tdfilter), tp, LOCATION_GRAVE, 0, 1, 3, false, NO_EX).nil();
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, n as i32);
    true
}
fn drop_op(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_target_cards(d, a.e).nil();
    if !g.is_empty() {
        let ct = duel::send_to_deck(d, g, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None);
        if ct == 0 {
            return;
        }
        if ct > 1 {
            duel::sort_deckbottom(d, a.tp, a.tp, ct as u16);
        }
        duel::break_effect(d);
        duel::draw(d, a.tp, ct as u32, REASON_EFFECT);
    }
}
