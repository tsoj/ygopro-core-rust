//! Psychic Eraser Laser
use super::prelude::*;

pub const ID: u32 = 9322133;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_RECOVER);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_summon_location(d, c, LOCATION_EXTRA) && card::is_able_to_grave(d, c, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, 1 - tp, LOCATION_MZONE as i32);
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, 1 - tp, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&filter), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    let Some(tg) = group::get_first(d, &g) else { return };
    if duel::send_to_grave(d, tg, REASON_EFFECT, None, None) > 0 && card::is_location(d, tg, LOCATION_GRAVE) {
        let rec = card::get_base_attack(d, tg).max(card::get_base_defense(d, tg)) as i64;
        if rec == 0 {
            return;
        }
        duel::break_effect(d);
        duel::recover(d, 1 - tp, rec, REASON_EFFECT, false, None);
    }
}
