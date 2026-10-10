//! Bazoo the Soul-Eater
use super::prelude::*;

pub const ID: u32 = 40133511;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atkup
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_monster(d, c) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, false, false)
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, c.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    let c = effect::get_handler(d, a.e).nil();
    let cg = duel::select_matching_card(d, a.tp, Some(&cfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 3, false, c.into()).nil();
    let count = cg.len() as i64;
    duel::remove(d, cg, POS_FACEUP as u8, REASON_COST, None, None);
    effect::set_label(d, a.e, &[count]);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let count = effect::get_label(d, a.e);
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_value(d, e1, count * 300);
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 2);
        card::register_effect(d, c, e1, false);
    }
}
