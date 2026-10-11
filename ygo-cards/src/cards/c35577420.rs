//! Light Spiral
use super::prelude::*;

pub const ID: u32 = 35577420;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LIGHTSWORN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // discard deck
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_category(d, e2, CATEGORY_REMOVE);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_condition(d, e2, Some(condition(rmcon)));
    effect::set_target(d, e2, Some(cost_target(rmtg)));
    effect::set_operation(d, e2, Some(operation(rmop)));
    card::register_effect(d, c, e2, false);
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::get_previous_location(d, c) == LOCATION_DECK
}
/// `function(e,tp,eg,ep,ev,re,r,rp) ... end`
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    let Some(re) = a.re else { return false };
    let rc = effect::get_handler(d, re).nil();
    a.rp == a.tp
        && (a.r & REASON_EFFECT) != 0
        && card::is_set_card(d, rc, &[SET_LIGHTSWORN as u16])
        && card::is_monster(d, rc)
        && group::is_exists(d, &a.eg.clone().nil(), &cfilter, 1, NO_EX)
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk) if chk==0 then return true end ... end`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn rmop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    if duel::get_field_group_count(d, a.tp, 0, LOCATION_DECK) == 0 {
        return;
    }
    let g = duel::get_decktop_group(d, 1 - a.tp, 1);
    duel::disable_shuffle_check(d, true);
    duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
