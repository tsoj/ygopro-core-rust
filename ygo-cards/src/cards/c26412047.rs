//! Hammer Shot
use super::prelude::*;

pub const ID: u32 = 26412047;

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
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_position(d, c, POS_FACEUP_ATTACK)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    let g = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    let tg = group::get_max_group(d, &g, &|d, c| card::get_attack(d, c) as i64).map(|(tg, _)| tg);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, tg, 1, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    if !g.is_empty() {
        let (tg, _) = group::get_max_group(d, &g, &|d, c| card::get_attack(d, c) as i64).nil();
        if tg.len() > 1 {
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
            let sg = group::select(d, &tg, a.tp, 1, 1, false, NO_EX).nil();
            duel::hint_selection(d, sg.clone(), true);
            duel::destroy(d, sg, REASON_EFFECT, None, None);
        } else {
            duel::destroy(d, tg, REASON_EFFECT, None, None);
        }
    }
}
