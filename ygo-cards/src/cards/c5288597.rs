//! Transmodify
use super::prelude::*;

pub const ID: u32 = 5288597;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_label(d, e1, &[0]);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    effect::set_label(d, a.e, &[100]);
    true
}
fn cfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32) -> bool {
    let lv = card::get_original_level(d, c);
    let mut rc = card::get_original_race(d, c);
    let mut att = card::get_original_attribute(d, c);
    let eff4064256 = duel::get_player_effect(d, tp, 4064256);
    for te in eff4064256 {
        if let Value::Fn(val) = effect::get_value(d, te) {
            if val.call(d, &[Value::Effect(te), Value::Card(c), Value::Effect(e), Value::Int(0)]).truthy() {
                rc = val.call(d, &[Value::Effect(te), Value::Card(c), Value::Effect(e), Value::Int(1)]).int() as u64;
            }
        }
    }
    let eff12644061 = duel::get_player_effect(d, tp, CARD_ADVANCED_DARK);
    for te in eff12644061 {
        if let Value::Fn(val) = effect::get_value(d, te) {
            if val.call(d, &[Value::Effect(te), Value::Card(c), Value::Effect(e), Value::Int(0)]).truthy() {
                att = val.call(d, &[Value::Effect(te), Value::Card(c), Value::Effect(e), Value::Int(1)]).int() as u32;
            }
        }
    }
    card::is_monster_card(d, c)
        && lv > 0
        && card::is_faceup(d, c)
        && card::is_able_to_grave_as_cost(d, c)
        && (ft > 0 || card::get_sequence(d, c) < 5)
        && duel::is_existing_matching_card(d, Some(&|d, cc| spfilter(d, cc, lv + 1, rc, att, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
fn spfilter(d: &mut Duel, c: CardId, lv: u32, rc: u64, att: u32, e: EffectId, tp: u8) -> bool {
    card::is_level(d, c, &[lv])
        && card::is_race(d, c, rc)
        && card::is_attribute(d, c, att)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    if a.chk == Some(0) {
        if effect::get_label(d, a.e) != 100 {
            return false;
        }
        effect::set_label(d, a.e, &[0]);
        return ft > -1 && duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c, a.e, tp, ft)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    effect::set_label(d, a.e, &[0]);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| cfilter(d, c, a.e, tp, ft)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g).nil();
    duel::send_to_grave(d, CardSet::from_card(tc), REASON_COST, None, None);
    duel::set_target_card(d, tc);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    let Some(tc) = duel::first_target(d) else { return };
    if !card::is_location(d, tc, LOCATION_GRAVE) || !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let lv = card::get_level(d, tc) + 1;
    let rc = card::get_race(d, tc);
    let att = card::get_attribute(d, tc);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, lv, rc, att, a.e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, &g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
