//! Star Seraph Sovereignty
use super::prelude::*;

pub const ID: u32 = 91110378;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_STAR_SERAPH], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz
    let e1 = effect::create_effect(d, c);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_STAR_SERAPH_SOVEREIGNTY);
    // 0x1 >, 0x2 =, 0x4 <, value == last digit(s)
    effect::set_value(d, e1, 0x30003);
    card::register_effect(d, c, e1, false);
    // spsummon
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_condition(d, e2, Some(condition(spcon)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e3, false);
}
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_STAR_SERAPH as u16]) && card::is_summon_player(d, c, tp)
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, tp), 1, NO_EX)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1)
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) != 0 {
        if duel::draw(d, a.tp, 1, REASON_EFFECT) == 0 {
            return;
        }
        let og = duel::get_operated_group(d);
        let dc = group::get_first(d, &og).nil();
        if card::is_set_card(d, dc, &[SET_STAR_SERAPH as u16])
            && card::is_can_be_special_summoned(d, dc, a.e, 0, a.tp, false, false, None, None, None)
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1))
        {
            duel::break_effect(d);
            duel::special_summon(d, dc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
