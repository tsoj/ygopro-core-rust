//! De-Spell
use super::prelude::*;

pub const ID: u32 = 19159413;

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
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_facedown(d, c) || card::is_spell(d, c)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        let h = effect::get_handler(d, a.e);
        return card::is_location(d, chkc, LOCATION_SZONE) && filter(d, chkc) && Some(chkc) != h;
    }
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_SZONE, LOCATION_SZONE, 1, h.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    let h = effect::get_handler(d, a.e).nil();
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_SZONE, LOCATION_SZONE, 1, 1, false, h.into()).nil();
    let tc = group::get_first(d, &g).nil();
    if card::is_faceup(d, tc) {
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    }
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        if card::is_facedown(d, tc) {
            duel::confirm_cards(d, a.tp, tc);
        }
        if card::is_spell(d, tc) {
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
