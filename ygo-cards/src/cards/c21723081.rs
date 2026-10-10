//! XYZ Hyper Cannon
use super::prelude::*;

pub const ID: u32 = 21723081;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e0, false);
    // Activate the appropriate effect based on whose turn it is
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    effect::set_condition(d, e1, Some(condition(effcon)));
    effect::set_target(d, e1, Some(cost_target(efftg)));
    effect::set_operation(d, e1, Some(operation(effop)));
    card::register_effect(d, c, e1, false);
}

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[91998119],
    listed_card_types: &[TYPE_UNION],
    ..ScriptInfo::DEFAULT
};

fn effconfilter(d: &mut Duel, c: CardId) -> bool {
    (card::is_code(d, c, &[91998119])
        || (card::is_type(d, c, TYPE_FUSION) && card::is_monster(d, c) && card::lists_code_as_material(d, c, &[91998119])))
        && card::is_faceup(d, c)
}
fn effcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&effconfilter), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_type(d, c, TYPE_UNION) && card::is_faceup(d, c) && card::is_able_to_deck(d, c)
}
/// `Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,e:GetHandler())`, evaluated
/// in the same order as the Lua `b2` expression.
fn b2_check(d: &mut Duel, a: &EffArgs) -> bool {
    let h: Exclude = effect::get_handler(d, a.e).nil().into();
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), a.tp, LOCATION_HAND, 0, 1, h)
}
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        let op = effect::get_label(d, a.e);
        if op == 1 {
            return card::is_location(d, chkc, LOCATION_REMOVED) && card::is_controler(d, chkc, a.tp) && tdfilter(d, chkc);
        } else if op == 2 {
            return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - a.tp);
        }
    }
    let b1 = duel::is_turn_player(d, a.tp)
        && duel::is_player_can_draw(d, a.tp, 1)
        && duel::is_existing_target(d, Some(&tdfilter), a.tp, LOCATION_REMOVED, 0, 1, NO_EX);
    let b2 = duel::is_turn_player(d, 1 - a.tp) && b2_check(d, a) && duel::is_existing_target(d, None, a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    if a.chk == Some(0) {
        return b1 || b2;
    }
    let op = if b1 { 1 } else { 2 };
    effect::set_label(d, a.e, &[op]);
    if op == 1 {
        effect::set_category(d, a.e, CATEGORY_TODECK + CATEGORY_DRAW);
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
        let g = duel::select_target(d, a.tp, Some(&tdfilter), a.tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX);
        duel::set_target_player(d, a.tp);
        duel::set_target_param(d, 1);
        duel::set_operation_info(d, 0, CATEGORY_TODECK, g, 1, a.tp, 0);
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    } else if op == 2 {
        effect::set_category(d, a.e, CATEGORY_DESTROY);
        let target_ct = duel::get_target_count(d, None, a.tp, 0, LOCATION_ONFIELD, NO_EX);
        let discard_ct = duel::discard_hand(
            d,
            a.tp,
            Some(&|d, c| card::is_discardable(d, c, None, None)),
            1,
            target_ct as u16,
            REASON_COST | REASON_DISCARD,
            NO_EX,
        );
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
        let g = duel::select_target(d, a.tp, None, a.tp, 0, LOCATION_ONFIELD, discard_ct as u16, discard_ct as u16, false, NO_EX);
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, discard_ct as u8, a.tp, 0);
    }
    true
}
fn effop(d: &mut Duel, a: &EffArgs) {
    let op = effect::get_label(d, a.e);
    if op == 1 {
        // Place 1 of your banished Union monsters on the bottom of your Deck, and if you do, draw 1 card
        let tc = duel::first_target(d).nil();
        if card::is_relate_to_effect(d, tc, a.e)
            && duel::send_to_deck(d, tc, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None) > 0
            && card::is_location(d, tc, LOCATION_DECK | LOCATION_EXTRA)
        {
            let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
            let p = info.first().and_then(|v| if let Value::Int(x) = v { Some(*x) } else { None }).nil() as u8;
            let n = info.get(1).and_then(|v| if let Value::Int(x) = v { Some(*x) } else { None }).nil() as u32;
            duel::draw(d, p, n, REASON_EFFECT);
        }
    } else if op == 2 {
        // Destroy cards your opponent controls equal to the number of cards you discarded
        if let Some(tg) = duel::get_target_cards(d, a.e) {
            if !tg.is_empty() {
                duel::destroy(d, tg, REASON_EFFECT, None, None);
            }
        }
    }
}
