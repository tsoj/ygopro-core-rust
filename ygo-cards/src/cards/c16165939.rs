//! Supernatural Danger Zone
use super::prelude::*;

pub const ID: u32 = 16165939;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Destroy
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(descon)));
    effect::set_target(d, e2, Some(cost_target(destg)));
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
    // Special Summon
    let e3 = effect::create_effect(d, c);
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e3, EVENT_DESTROYED);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_condition(d, e3, Some(condition(spcon)));
    effect::set_target(d, e3, Some(cost_target(sptg)));
    effect::set_operation(d, e3, Some(operation(spop)));
    card::register_effect(d, c, e3, false);
}
fn descfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_non_effect_monster(d, c)
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &descfilter, 1, NO_EX)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - a.tp);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    let g = duel::select_target(d, a.tp, None, a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let tc = duel::first_target(d);
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None);
        }
    }
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    a.rp == 1 - a.tp
        && card::is_reason(d, c, REASON_EFFECT)
        && card::is_previous_controler(d, c, a.tp)
        && card::is_previous_location(d, c, LOCATION_SZONE)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_non_effect_monster(d, c) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp))),
        a.tp,
        LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    if !g.is_empty() {
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
