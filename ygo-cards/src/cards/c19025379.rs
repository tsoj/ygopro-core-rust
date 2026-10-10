//! Lord of the Red
use super::prelude::*;

pub const ID: u32 = 19025379;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[45410988, ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // destroy monster
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(descon)));
    effect::set_target(d, e1, Some(cost_target(destg1)));
    effect::set_operation(d, e1, Some(operation(desop1)));
    card::register_effect(d, c, e1, false);
    // destroy spell & trap
    let e2 = effect::clone(d, e1);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_target(d, e2, Some(cost_target(destg2)));
    effect::set_operation(d, e2, Some(operation(desop2)));
    card::register_effect(d, c, e2, false);
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let re_h = effect::get_handler(d, a.re.nil()).nil();
    let h = effect::get_handler(d, a.e).nil();
    !card::is_code(d, re_h, &[ID]) && !card::is_status(d, h, STATUS_BATTLE_DESTROYED)
}
fn destg1(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, desc);
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    let g = duel::select_target(d, a.tp, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
fn desfilter2(d: &mut Duel, c: CardId) -> bool {
    card::is_spell_trap(d, c)
}
fn destg2(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && desfilter2(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&desfilter2), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, desc);
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    let g = duel::select_target(d, a.tp, Some(&desfilter2), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
fn desop1(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && card::is_monster(d, tc) {
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
fn desop2(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
