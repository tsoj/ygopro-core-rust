//! Chaosrider Gustaph
use super::prelude::*;

pub const ID: u32 = 47829960;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Banish up to 2 Spells and increase ATK
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_REMOVE | CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    effect::set_operation(d, e1, Some(operation(rmop)));
    card::register_effect(d, c, e1, false);
}
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_spell(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&rmfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - a.tp, LOCATION_GRAVE as i32);
    true
}
fn rmop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&rmfilter), a.tp, LOCATION_GRAVE, 0, 1, 2, false, NO_EX).nil();
    if g.is_empty() {
        return;
    }
    duel::hint_selection(d, g.clone(), true);
    let ct = duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    let c = effect::get_handler(d, a.e).nil();
    if ct > 0 && card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // Increase ATK
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_value(d, e1, ct as i64 * 300);
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END | RESET_OPPO_TURN, 1);
        card::register_effect(d, c, e1, false);
    }
}
