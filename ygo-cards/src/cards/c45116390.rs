//! Heavy Armored Knight Babeldecker
use super::prelude::*;

pub const ID: u32 = 45116390;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can Normal Summon this card without Tributing
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_SUMMON_PROC);
    effect::set_condition(d, e1, Some(lua_fn(ntcon)));
    card::register_effect(d, c, e1, false);
    // Special Summon 1 EARTH Machine monster from your hand
    let e2a = effect::create_effect(d, c);
    effect::set_description(d, e2a, aux::stringid(ID, 1));
    effect::set_category(d, e2a, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE | EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2a, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2a, EVENT_SUMMON_SUCCESS);
    effect::set_count_limit(d, e2a, 1, ID, 0, 0);
    effect::set_target(d, e2a, Some(cost_target(handsptg)));
    effect::set_operation(d, e2a, Some(operation(handspop)));
    card::register_effect(d, c, e2a, false);
    let e2b = effect::clone(d, e2a);
    effect::set_code(d, e2b, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e2b, false);
    // Special Summon 1 Rank 10 EARTH Machine Xyz Monster from your Extra Deck using this card you control as material (this is treated as an Xyz Summon)
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_condition(d, e3, Some(condition(|d, a| duel::get_custom_activity_count(d, ID, 1 - a.tp, ACTIVITY_CHAIN) > 0)));
    effect::set_target(d, e3, Some(cost_target(extrasptg)));
    effect::set_operation(d, e3, Some(operation(extraspop)));
    card::register_effect(d, c, e3, false);
    duel::add_custom_activity_counter(d, ID, ACTIVITY_CHAIN, lua_fn(|_d: &mut Duel, _a: Args| false));
}
fn ntcon(d: &mut Duel, a: Args) -> bool {
    let Some(c) = a.card(1) else { return true };
    a.int(2) == 0 && card::is_level_above(d, c, 4) && duel::get_location_count(d, card::get_controler(d, c), LOCATION_MZONE as u8, None, None, None).0 > 0
}
fn handspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_attribute(d, c, ATTRIBUTE_EARTH) && card::is_race(d, c, RACE_MACHINE) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn handsptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| handspfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
fn handspop(d: &mut Duel, a: &EffArgs) {
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| handspfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
fn extraspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId) -> bool {
    card::is_type_ctx(d, c, TYPE_XYZ, Some(c), SUMMON_TYPE_XYZ as u64, Some(tp))
        && card::is_rank(d, c, &[10])
        && card::is_attribute(d, c, ATTRIBUTE_EARTH)
        && card::is_race(d, c, RACE_MACHINE)
        && card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ, tp, false, false, None, None, None)
}
fn extrasptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        let pg = aux::get_must_be_material_group_opt(d, a.tp, Some(&CardSet::from_card(c)), a.tp, None, None, REASON_XYZ);
        return (pg.is_empty() || (pg.len() == 1 && group::is_contains(&pg, c)))
            && duel::is_existing_matching_card(d, Some(&|d, x| extraspfilter(d, x, a.e, a.tp, c)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
fn extraspop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_facedown(d, c) || !card::is_relate_to_effect(d, c, a.e) || card::is_controler(d, c, 1 - a.tp) || card::is_immune_to_effect(d, c, a.e) {
        return;
    }
    let pg = aux::get_must_be_material_group_opt(d, a.tp, Some(&CardSet::from_card(c)), a.tp, None, None, REASON_XYZ);
    if pg.len() > 1 || (pg.len() == 1 && !group::is_contains(&pg, c)) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let sc = duel::select_matching_card(d, a.tp, Some(&|d, x| extraspfilter(d, x, a.e, a.tp, c)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let Some(xc) = group::get_first(d, &sc) else { return };
    card::set_material(d, xc, CardSet::from_card(c));
    duel::overlay(d, xc, c, false);
    if duel::special_summon(d, xc, SUMMON_TYPE_XYZ, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
        card::complete_procedure(d, xc);
    }
}
