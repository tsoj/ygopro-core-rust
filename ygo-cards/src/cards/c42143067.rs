//! Doki Doki
use super::prelude::*;

pub const ID: u32 = 42143067;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
}
// function s.spcost(...) e:SetLabel(100) return true end
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    effect::set_label(d, a.e, &[100]);
    true
}
fn cfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    if !(card::is_race(d, c, RACE_ROCK) && card::is_discardable(d, c, None, None)) {
        return false;
    }
    let att = card::get_original_attribute(d, c);
    let lv = card::get_original_level(d, c);
    duel::is_existing_matching_card(d, Some(&|d, cc| spfilter(d, cc, e, tp, att, lv)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, att: u32, lv: u32) -> bool {
    card::is_race(d, c, RACE_ROCK)
        && card::is_original_attribute(d, c, att)
        && card::get_original_level(d, c) == lv
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some((POS_FACEUP_ATTACK | POS_FACEDOWN_DEFENSE) as u8), None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        if effect::get_label(d, e) != 100 {
            return false;
        }
        effect::set_label(d, e, &[0]);
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return false;
        }
        return duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DISCARD as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| cfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let first = group::get_first(d, &g);
    effect::set_label_object(d, e, first.map_or(Value::Nil, Value::Card));
    duel::send_to_grave(d, g, REASON_COST | REASON_DISCARD, None, None);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 0, LOCATION_DECK as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    let gc = effect::get_label_object(d, e).as_card();
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    let gc = gc.nil();
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let att = card::get_original_attribute(d, gc);
    let lv = card::get_original_level(d, gc);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, att, lv)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    if let Some(tc) = tc {
        if duel::special_summon(d, tc, 0, tp, tp, false, false, (POS_FACEUP_ATTACK | POS_FACEDOWN_DEFENSE) as u8, None) != 0 && card::is_facedown(d, tc) {
            duel::confirm_cards(d, 1 - tp, tc);
        }
    }
}
