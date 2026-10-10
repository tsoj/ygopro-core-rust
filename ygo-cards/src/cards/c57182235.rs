//! Token Thanksgiving
use super::prelude::*;

pub const ID: u32 = 57182235;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_RECOVER);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `Duel.GetMatchingGroup(Card.IsType,tp,LOCATION_MZONE,LOCATION_MZONE,nil,TYPE_TOKEN)`
fn token_group(d: &mut Duel, tp: u8) -> CardSet {
    duel::get_matching_group(d, Some(&|d, c| card::is_type(d, c, TYPE_TOKEN)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_type(d, c, TYPE_TOKEN)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    let g = token_group(d, a.tp);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), g.len() as u8, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, (g.len() * 800) as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let g = token_group(d, a.tp);
    let ct = duel::destroy(d, g, REASON_EFFECT, None, None);
    duel::recover(d, a.tp, ct as i64 * 800, REASON_EFFECT, false, None);
}
