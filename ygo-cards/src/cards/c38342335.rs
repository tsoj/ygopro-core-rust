//! Knightmare Unicorn
use super::prelude::*;

pub const ID: u32 = 38342335;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_KNIGHTMARE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    link::add_procedure(d, c, None, 2, Some(99), Some(group_check(lcheck)), None, None);
    card::enable_revive_limit(d, c);
    // shuffle card into the deck
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(tdcon)));
    effect::set_cost(d, e1, Some(cost_target(tdcost)));
    effect::set_target(d, e1, Some(cost_target(tdtg)));
    effect::set_operation(d, e1, Some(operation(tdop)));
    card::register_effect(d, c, e1, false);
    // Change draw count
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_DRAW_COUNT);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_target_range(d, e2, 1, 0);
    effect::set_value_fn(d, e2, lua_fn(drval));
    card::register_effect(d, c, e2, false);
}
/// `s.lcheck(g,lc,sumtype,tp)`: `g:CheckDifferentProperty(Card.GetCode,lc,sumtype,tp)`
fn lcheck(d: &mut Duel, g: &CardSet, _lc: CardId, _sumtype: u64, _tp: u8) -> bool {
    group::check_different_property(d, g, &code_list)
}
fn code_list(d: &mut Duel, c: CardId) -> Vec<i64> {
    card::get_code(d, c).iter().map(|x| *x as i64).collect()
}
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, h)
}
fn discardable(d: &mut Duel, c: CardId) -> bool {
    card::is_discardable(d, c, None, None)
}
fn tdcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&discardable), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::discard_hand(d, a.tp, Some(&discardable), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_able_to_deck(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&card::is_able_to_deck), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    let g = duel::select_target(d, a.tp, Some(&card::is_able_to_deck), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), 1, 0, 0);
    let h = effect::get_handler(d, a.e).nil();
    if card::get_mutual_linked_group_count(d, h) > 0 {
        effect::set_label(d, a.e, &[1]);
    } else {
        effect::set_label(d, a.e, &[0]);
    }
    let mut cat = CATEGORY_TODECK;
    if effect::get_label(d, a.e) == 1 {
        cat += CATEGORY_DRAW;
    }
    effect::set_category(d, a.e, cat);
    true
}
fn tdop(d: &mut Duel, a: &EffArgs) {
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e)
            && duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0
            && card::is_location(d, tc, LOCATION_DECK | LOCATION_EXTRA)
            && effect::get_label(d, a.e) == 1
            && duel::is_player_can_draw(d, a.tp, 1)
            && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1))
        {
            duel::break_effect(d);
            if card::is_location(d, tc, LOCATION_DECK) && card::is_controler(d, tc, a.tp) {
                duel::shuffle_deck(d, a.tp);
            }
            duel::draw(d, a.tp, 1, REASON_EFFECT);
        }
    }
}
fn drfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_KNIGHTMARE as u16]) && card::get_mutual_linked_group_count(d, c) > 0
}
fn drval(d: &mut Duel, a: Args) -> i64 {
    let tp = effect::get_handler_player(d, a.e(0));
    let g = duel::get_matching_group(d, Some(&drfilter), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    if g.is_empty() {
        return 1;
    }
    group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) as i64
}
