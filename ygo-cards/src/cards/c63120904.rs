//! Orca Mega-Fortress of Darkness
use super::prelude::*;

pub const ID: u32 = 63120904;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // destroy
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn rfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[90337190, 95614612])
}
fn dfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_spell_trap(d, c)
}
fn is_fish(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[90337190])
}
fn is_cannonball(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[95614612])
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        if effect::get_label(d, a.e) == 90337190 {
            return card::is_location(d, chkc, LOCATION_MZONE);
        } else {
            return card::is_on_field(d, chkc) && dfilter(d, chkc);
        }
    }
    let b1 = duel::check_release_group_cost(d, a.tp, Some(&is_fish), 1, 1, false, None, NO_EX)
        && duel::is_existing_target(d, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    let b2 = duel::check_release_group_cost(d, a.tp, Some(&is_cannonball), 1, 1, false, None, NO_EX)
        && duel::is_existing_target(d, Some(&dfilter), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    if a.chk == Some(0) {
        return b1 || b2;
    }
    let code: u32;
    if b1 && b2 {
        let rg = duel::select_release_group_cost(d, a.tp, Some(&rfilter), 1, 1, false, None, NO_EX);
        let first = group::get_first(d, &rg).nil();
        code = card::get_code(d, first)[0];
        duel::release(d, rg, REASON_COST, None);
    } else if b1 {
        let rg = duel::select_release_group_cost(d, a.tp, Some(&is_fish), 1, 1, false, None, NO_EX);
        code = 90337190;
        duel::release(d, rg, REASON_COST, None);
    } else {
        let rg = duel::select_release_group_cost(d, a.tp, Some(&is_cannonball), 1, 1, false, None, NO_EX);
        code = 95614612;
        duel::release(d, rg, REASON_COST, None);
    }
    effect::set_label(d, a.e, &[code as i64]);
    if code == 90337190 {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
        let g = duel::select_target(d, a.tp, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    } else {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
        let g = duel::select_target(d, a.tp, Some(&dfilter), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    }
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) {
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
