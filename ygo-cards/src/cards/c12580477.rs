//! Raigeki
use super::prelude::*;

pub const ID: u32 = 12580477;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, None, a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    let sg = duel::get_matching_group(d, None, a.tp, 0, LOCATION_MZONE, NO_EX);
    let n = sg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(sg), n, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let sg = duel::get_matching_group(d, None, a.tp, 0, LOCATION_MZONE, NO_EX);
    duel::destroy(d, sg, REASON_EFFECT, None, None);
}
