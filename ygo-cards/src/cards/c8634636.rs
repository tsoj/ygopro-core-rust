//! Dark Cat with White Tail
use super::prelude::*;

pub const ID: u32 = 8634636;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // flip
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chkc().is_some() {
        return false;
    }
    if a.chk == Some(0) {
        return true;
    }
    let is_able_to_hand = |d: &mut Duel, c: CardId| card::is_able_to_hand(d, c, None);
    if duel::is_existing_target(d, Some(&is_able_to_hand), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::is_existing_target(d, Some(&is_able_to_hand), a.tp, 0, LOCATION_MZONE, 2, NO_EX)
    {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
        let g1 = duel::select_target(d, a.tp, Some(&is_able_to_hand), a.tp, 0, LOCATION_MZONE, 2, 2, false, NO_EX).nil();
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
        let g2 = duel::select_target(d, a.tp, Some(&is_able_to_hand), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
        let mut g1 = g1;
        group::add_card(&mut g1, &g2);
        let n = g1.len() as u8;
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g1), n, 0, 0);
    }
    true
}
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned();
    if let Some(g) = g {
        let sg = group::filter(d, &g, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
        duel::send_to_hand(d, sg, None, REASON_EFFECT, None);
    }
}
