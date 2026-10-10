//! Ojama Delta Hurricane!!
use super::prelude::*;

pub const ID: u32 = 8251996;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[12482652, 42941100, 79335209], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[12482652]))), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[42941100]))), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[79335209]))), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_ONFIELD);
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, a.tp, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_ONFIELD);
    if !g.is_empty() {
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
