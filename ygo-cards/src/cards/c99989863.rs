//! Fiendsmith in Paradise
use super::prelude::*;

pub const ID: u32 = 99989863;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FIENDSMITH], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some((TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E) as u32));
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Send 1 "Fiendsmith" monster from your Deck or Extra Deck to the GY
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(e2cond)));
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    effect::set_operation(d, e2, Some(operation(tgop)));
    card::register_effect(d, c, e2, false);
}
fn trgtfilter(d: &mut Duel, c: CardId, rc: CardId) -> bool {
    card::is_level_above(d, c, 7)
        && card::is_attribute(d, c, ATTRIBUTE_LIGHT)
        && card::is_race(d, c, RACE_FIEND)
        && card::is_faceup(d, c)
        && {
            let mut ex = CardSet::from_card(c);
            group::add_card(&mut ex, &CardSet::from_card(rc));
            duel::is_existing_matching_card(d, None, 0, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, (&ex).into())
        }
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && trgtfilter(d, chkc, c);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| trgtfilter(d, x, c)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    let g = duel::select_target(d, tp, Some(&|d, x| trgtfilter(d, x, c)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let mut ex = g.clone();
    group::add_card(&mut ex, &CardSet::from_card(c));
    let tgg = duel::get_matching_group(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, (&ex).into());
    let n = tgg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(tgg), n, tp, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    let h = effect::get_handler(d, a.e).nil();
    let mut ex = CardSet::from_card(tc);
    group::add_card(&mut ex, &CardSet::from_card(h));
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, (&ex).into());
    if !g.is_empty() {
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_FIENDSMITH as u16]) && card::is_monster(d, c) && card::is_able_to_grave(d, c, None)
}
/// `function(e,tp,eg) return eg:IsExists(Card.IsSummonPlayer,1,nil,1-tp) end`
fn e2cond(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| card::is_summon_player(d, c, 1 - tp), 1, NO_EX)
}
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, (LOCATION_DECK | LOCATION_EXTRA) as i32);
    true
}
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
