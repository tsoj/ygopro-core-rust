//! Magical Marionette
use super::prelude::*;

pub const ID: u32 = 8034697;

pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[COUNTER_SPELL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_counter_permit(d, c, COUNTER_SPELL as u16, None, None);
    // add counter
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e0, EVENT_CHAINING);
    effect::set_range(d, e0, LOCATION_MZONE);
    effect::set_operation(d, e0, Some(aux::chainreg()));
    card::register_effect(d, c, e0, false);
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EVENT_CHAIN_SOLVING);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_operation(d, e1, Some(operation(acop)));
    card::register_effect(d, c, e1, false);
    // attackup
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_value_fn(d, e2, effect_card_value(attackup));
    card::register_effect(d, c, e2, false);
    // destroy
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_DESTROY);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_cost(d, e3, Some(cost_target(descost)));
    effect::set_target(d, e3, Some(cost_target(destarg)));
    effect::set_operation(d, e3, Some(operation(desop)));
    card::register_effect(d, c, e3, false);
}
fn acop(d: &mut Duel, a: &EffArgs) {
    let re = a.re.nil();
    let c = effect::get_handler(d, a.e).nil();
    if effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && effect::is_spell_effect(d, re) && card::get_flag_effect(d, c, 1) > 0 {
        card::add_counter(d, c, COUNTER_SPELL as u16, 1, false);
    }
}
fn attackup(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    card::get_counter(d, c, COUNTER_SPELL as u16) as i64 * 200
}
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_can_remove_counter(d, c, a.tp, COUNTER_SPELL as u16, 2, REASON_COST);
    }
    card::remove_counter(d, c, a.tp, COUNTER_SPELL as u16, 2, REASON_COST);
    true
}
fn destarg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    let g = duel::select_target(d, a.tp, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
