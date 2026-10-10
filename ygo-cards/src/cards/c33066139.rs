//! Reaper of the Cards
use super::prelude::*;

pub const ID: u32 = 33066139;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // flip
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_facedown(d, c) || card::is_trap(d, c)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_SZONE) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return true;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    let c = effect::get_handler(d, a.e).nil();
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_SZONE, LOCATION_SZONE, 1, 1, false, c.into()).nil();
    if !g.is_empty() {
        let tc = group::get_first(d, &g).nil();
        if card::is_faceup(d, tc) {
            duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
        }
    }
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d);
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            if card::is_facedown(d, tc) {
                duel::confirm_cards(d, a.tp, tc);
            }
            if card::is_trap(d, tc) {
                duel::destroy(d, tc, REASON_EFFECT, None, None);
            }
        }
    }
}
