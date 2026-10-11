//! Shamanite Shamanknight
use super::prelude::*;

pub const ID: u32 = 16237004;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2+ Level 3 monsters
    xyz::add_procedure(d, c, None, Some(3), 2, None, None, Some(xyz::INFINITE_MATS), None, false, None);
    // If this card is Xyz Summoned, or if a Trap you own is banished: You can target 1 of your banished Traps; attach it to this card
    let e1a = effect::create_effect(d, c);
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1a, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1a, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e1a, 1, ID, 0, 0);
    effect::set_condition(d, e1a, Some(condition(|d, a| card::is_xyz_summoned(d, effect::get_handler(d, a.e).nil()))));
    effect::set_target(d, e1a, Some(cost_target(attachtg)));
    effect::set_operation(d, e1a, Some(operation(attachop)));
    card::register_effect(d, c, e1a, false);
    let e1b = effect::clone(d, e1a);
    effect::set_type(d, e1b, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1b, EVENT_REMOVE);
    effect::set_range(d, e1b, LOCATION_MZONE);
    effect::set_condition(d, e1b, Some(condition(attachcon)));
    card::register_effect(d, c, e1b, false);
    // You can detach any number of materials from this card, then target 1 DARK monster in your GY or banishment with a Level equal to the number detached; Special Summon it
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_cost(
        d,
        e2,
        Some(cost::detach_choice_from_self(
            std::sync::Arc::new(|d: &mut Duel, e: EffectId, tp: u8| {
                let g = duel::get_target_group(d, Some(&|d, c| spfilter(d, c, e, tp, None)), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, NO_EX);
                group::get_class(d, &g, &|d, c| card::get_level(d, c) as i64)
            }),
            None,
        )),
    );
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}
fn attachfilter(d: &mut Duel, c: CardId, xyzc: CardId, tp: u8) -> bool {
    card::is_trap(d, c) && card::is_faceup(d, c) && card::is_can_be_xyz_material(d, c, Some(xyzc), Some(tp), Some(REASON_EFFECT))
}
fn attachtg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_REMOVED) && attachfilter(d, chkc, c, a.tp);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, cc| attachfilter(d, cc, c, a.tp)), a.tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATTACH as u64);
    duel::select_target(d, a.tp, Some(&|d, cc| attachfilter(d, cc, c, a.tp)), a.tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX);
    true
}
fn attachop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d);
    if card::is_relate_to_effect(d, c, a.e) && card::is_relate_to_effect(d, tc.nil(), a.e) && card::is_can_be_xyz_material(d, tc.nil(), Some(c), Some(a.tp), Some(REASON_EFFECT)) {
        duel::overlay(d, c, tc.nil(), false);
    }
}
fn trapfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_trap(d, c) && card::is_faceup(d, c) && card::is_owner(d, c, tp)
}
fn attachcon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| trapfilter(d, c, a.tp), 1, NO_EX)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, level: Option<i64>) -> bool {
    card::is_attribute(d, c, ATTRIBUTE_DARK)
        && card::has_level(d, c)
        && level.map_or(true, |l| card::is_level(d, c, &[l as u32]))
        && card::is_faceup(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    if let Some(chkc) = a.chkc() {
        let level = detached_count(d, e);
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_GRAVE | LOCATION_REMOVED) && spfilter(d, chkc, e, a.tp, Some(level));
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, e, a.tp, None)), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    let level = detached_count(d, e);
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, a.tp, Some(&|d, c| spfilter(d, c, e, a.tp, Some(level))), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, a.tp, 0);
    true
}
/// `#e:GetChainData().cost_detached_materials`
fn detached_count(d: &Duel, e: EffectId) -> i64 {
    chain::effect_data(d, e, None, chain::field::COST_DETACHED_MATERIALS).as_group().cloned().nil().len() as i64
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
