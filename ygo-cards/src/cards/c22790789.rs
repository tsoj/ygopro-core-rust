//! B.E.S. Crystal Core
use super::prelude::*;

pub const ID: u32 = 22790789;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_counter_permit(d, c, 0x1f, None, None);
    // summon success
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_COUNTER);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE | EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_target(d, e1, Some(cost_target(addct)));
    effect::set_operation(d, e1, Some(operation(addc)));
    card::register_effect(d, c, e1, false);
    // battle indestructable
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_value(d, e2, 1);
    card::register_effect(d, c, e2, false);
    // remove counter
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE | EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e3, EVENT_DAMAGE_STEP_END);
    effect::set_condition(d, e3, Some(condition(rctcon)));
    effect::set_operation(d, e3, Some(operation(rctop)));
    card::register_effect(d, c, e3, false);
    // destroy
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 2));
    effect::set_category(d, e4, CATEGORY_DESTROY);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE | EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e4, EVENT_DAMAGE_STEP_END);
    effect::set_condition(d, e4, Some(condition(descon)));
    effect::set_target(d, e4, Some(cost_target(destg)));
    effect::set_operation(d, e4, Some(operation(desop)));
    card::register_effect(d, c, e4, false);
    // pos
    let e5 = effect::create_effect(d, c);
    effect::set_description(d, e5, aux::stringid(ID, 3));
    effect::set_category(d, e5, CATEGORY_POSITION);
    effect::set_property(d, e5, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e5, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e5, LOCATION_MZONE);
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    effect::set_target(d, e5, Some(cost_target(postg)));
    effect::set_operation(d, e5, Some(operation(posop)));
    card::register_effect(d, c, e5, false);
}
fn addct(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, None, 3, 0, 0x1f);
    true
}
fn addc(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        card::add_counter(d, c, 0x1f, 3, false);
    }
}
fn rctcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::get_counter(d, c, 0x1f) != 0
}
fn rctop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        card::remove_counter(d, c, a.tp, 0x1f, 1, REASON_EFFECT);
    }
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::get_counter(d, c, 0x1f) == 0
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        duel::destroy(d, c, REASON_EFFECT, None, None);
    }
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_position(d, c, POS_FACEUP_ATTACK as u32) && card::is_can_change_position(d, c)
}
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_POSCHANGE as u64);
    let g = duel::select_target(d, tp, Some(&filter), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_POSITION, g, 1, 0, 0);
    true
}
fn posop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d);
    if let Some(tc) = tc {
        if card::is_position(d, tc, POS_FACEUP_ATTACK as u32) && card::is_relate_to_effect(d, tc, a.e) {
            duel::change_position(d, tc, POS_FACEUP_DEFENSE as u8, None, None, None, false);
        }
    }
}
