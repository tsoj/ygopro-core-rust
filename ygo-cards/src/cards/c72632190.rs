//! Triple-Headed Behemoth
use super::prelude::*;

pub const ID: u32 = 72632190;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card from your hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e1, EVENT_DESTROYED);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(selfspcon)));
    effect::set_target(d, e1, Some(cost_target(selfsptg)));
    effect::set_operation(d, e1, Some(operation(selfspop)));
    card::register_effect(d, c, e1, false);
    // During the End Phase of this turn, Special Summon 1 WIND monster from your GY, but its ATK/DEF become 1000
    let e2a = effect::create_effect(d, c);
    effect::set_description(d, e2a, aux::stringid(ID, 1));
    effect::set_category(d, e2a, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2a, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2a, EVENT_SUMMON_SUCCESS);
    effect::set_count_limit(d, e2a, 1, ID, 1, EFFECT_COUNT_CODE_DUEL as u8);
    effect::set_target(d, e2a, Some(cost_target(efftg)));
    effect::set_operation(d, e2a, Some(operation(effop)));
    card::register_effect(d, c, e2a, false);
    let e2b = effect::clone(d, e2a);
    effect::set_code(d, e2b, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e2b, false);
}
fn selfspconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_reason(d, c, REASON_EFFECT)
        && card::is_previous_location(d, c, LOCATION_ONFIELD)
        && card::is_previous_position(d, c, POS_FACEUP)
        && card::is_previous_controler(d, c, tp)
}
fn selfspcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| selfspconfilter(d, c, tp), 1, NO_EX)
}
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
fn selfspop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if card::is_relate_to_effect(d, c, e) {
        duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // During the End Phase of this turn, Special Summon 1 WIND monster from your GY, but its ATK/DEF become 1000
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_operation(d, e1, Some(operation(gyspop)));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, tp);
}
fn gyspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_attribute(d, c, ATTRIBUTE_WIND) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn gyspop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(|d, c| gyspfilter(d, c, e, tp))), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    if let Some(sc) = sc {
        if duel::special_summon_step(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
            // Its ATK/DEF become 1000
            let h = effect::get_handler(d, e).nil();
            let e1a = effect::create_effect(d, h);
            effect::set_type(d, e1a, EFFECT_TYPE_SINGLE);
            effect::set_property(d, e1a, EFFECT_FLAG_CANNOT_DISABLE, 0);
            effect::set_code(d, e1a, EFFECT_SET_ATTACK);
            effect::set_value(d, e1a, 1000);
            effect::set_reset(d, e1a, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, sc, e1a, false);
            let e1b = effect::clone(d, e1a);
            effect::set_code(d, e1b, EFFECT_SET_DEFENSE);
            card::register_effect(d, sc, e1b, false);
        }
    }
    duel::special_summon_complete(d);
}
