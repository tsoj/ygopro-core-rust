//! Flash of the Forbidden Spell
use super::prelude::*;

pub const ID: u32 = 39956951;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If your opponent controls monsters in all of their Main Monster Zones: Destroy all monsters your opponent controls
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
/// `TOTAL_MMZ_COUNT=Duel.IsDuelType(DUEL_3_COLUMNS_FIELD) and 3 or 5`
fn total_mmz_count(d: &Duel) -> usize {
    if duel::is_duel_type(d, DUEL_3_COLUMNS_FIELD) {
        3
    } else {
        5
    }
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetMatchingGroupCount(nil,tp,0,LOCATION_MMZONE,nil)==TOTAL_MMZ_COUNT
    duel::get_matching_group_count(d, None, a.tp, 0, LOCATION_MMZONE, NO_EX) == total_mmz_count(d)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, 0, LOCATION_MZONE) > 0;
    }
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_MZONE);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), g.len() as u8, a.tp, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_MZONE);
    if !g.is_empty() {
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
