//! Two-Pronged Attack
use super::prelude::*;

pub const ID: u32 = 83887306;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, LOCATION_MZONE, 0, 2, NO_EX) && duel::is_existing_target(d, None, tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    let mut g1 = duel::select_target(d, tp, None, tp, LOCATION_MZONE, 0, 2, 2, false, NO_EX).nil();
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    let g2 = duel::select_target(d, tp, None, tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    group::add_card(&mut g1, &g2);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g1), 3, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    let tg = group::filter(d, &g, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
    if tg.len() == 3 {
        duel::destroy(d, tg, REASON_EFFECT, None, None);
    }
}
