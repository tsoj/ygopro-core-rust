//! The Return to the Normal
use super::prelude::*;

pub const ID: u32 = 56001930;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e2, false);
}
fn cond(_d: &mut Duel, a: &EffArgs) -> bool {
    a.eg.as_ref().map_or(false, |g| g.len() == 1)
}
fn filter(d: &mut Duel, c: CardId, atk: i32) -> bool {
    card::is_faceup(d, c) && card::get_attack(d, c) < atk
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let ec = group::get_first(d, &a.eg.clone().nil()).nil();
    let atk = card::get_attack(d, ec);
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, atk)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, ec.into());
    }
    card::create_effect_relation(d, ec, a.e);
    let g = duel::get_matching_group(d, Some(&|d, c| filter(d, c, atk)), a.tp, LOCATION_MZONE, LOCATION_MZONE, ec.into());
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), g.len() as u8, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let ec = match group::get_first(d, &a.eg.clone().nil()) {
        Some(ec) => ec,
        None => return,
    };
    if !(card::is_relate_to_effect(d, ec, a.e) && card::is_faceup(d, ec)) {
        return;
    }
    let atk = card::get_attack(d, ec);
    let g = duel::get_matching_group(d, Some(&|d, c| filter(d, c, atk)), a.tp, LOCATION_MZONE, LOCATION_MZONE, ec.into());
    if !g.is_empty() {
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
