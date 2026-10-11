//! Dharc the Dark Charmer
use super::prelude::*;

pub const ID: u32 = 19327348;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // flip
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_CONTROL);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_DARK) && card::is_controler_can_be_changed(d, c, false, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return true;
    }
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_status(d, h, STATUS_BATTLE_DESTROYED) {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONTROL as u64);
        let g = duel::select_target(d, a.tp, Some(&filter), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(g.clone()), g.len() as u8, 0, 0);
    }
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d);
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        if let Some(tc) = tc {
            if card::is_relate_to_effect(d, tc, a.e) && !card::is_immune_to_effect(d, tc, a.e) {
                card::set_card_target(d, c, tc);
                let e1 = effect::create_effect(d, c);
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                effect::set_code(d, e1, EFFECT_SET_CONTROL);
                effect::set_value(d, e1, a.tp as i64);
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                effect::set_condition(d, e1, Some(condition(ctcon)));
                card::register_effect(d, tc, e1, false);
            }
        }
    }
}
fn ctcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetOwner()
    // local h=e:GetHandler()
    // return c:IsHasCardTarget(h)
    let c = effect::get_owner(d, a.e).nil();
    let h = effect::get_handler(d, a.e).nil();
    card::is_has_card_target(d, c, h)
}
