//! Mystic Box
use super::prelude::*;

pub const ID: u32 = 25774450;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    duel::get_m_zone_count(d, tp, c.into(), Some(tp), Some(LOCATION_REASON_CONTROL), None).0 > 0
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chkc().is_some() {
        return false;
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, 1 - tp)), tp, 0, LOCATION_MZONE, 1, NO_EX)
            && duel::is_existing_target(d, Some(&|d, c| card::is_able_to_change_controler(d, c)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    let g1 = duel::select_target(d, tp, Some(&|d, c| filter(d, c, 1 - tp)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
    let g2 = duel::select_target(d, tp, Some(&|d, c| card::is_able_to_change_controler(d, c)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g1, 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, g2, 1, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let dg = duel::get_operation_info(d, 0, CATEGORY_DESTROY).and_then(|x| x.op_cards);
    let cg = duel::get_operation_info(d, 0, CATEGORY_CONTROL).and_then(|x| x.op_cards);
    let dc = group::get_first(d, &dg.nil());
    let cc = group::get_first(d, &cg.nil());
    if card::is_relate_to_effect(d, dc.nil(), a.e) && duel::destroy(d, dc.nil(), REASON_EFFECT, None, None) > 0 && card::is_relate_to_effect(d, cc.nil(), a.e) {
        duel::break_effect(d);
        duel::get_control(d, cc.nil(), 1 - tp, 0, 0, None, None);
    }
}
