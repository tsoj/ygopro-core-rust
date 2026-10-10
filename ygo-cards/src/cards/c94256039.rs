//! Tower of Babel
use super::prelude::*;

pub const ID: u32 = 94256039;

pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[COUNTER_SPELL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_counter_permit(d, c, COUNTER_SPELL as u16, None, None);
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // add counter
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e2, EVENT_CHAINING);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_operation(d, e2, Some(aux::chainreg()));
    card::register_effect(d, c, e2, false);
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    effect::set_code(d, e3, EVENT_CHAIN_SOLVED);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_operation(d, e3, Some(operation(acop)));
    card::register_effect(d, c, e3, false);
    // destroy&damage
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 0));
    effect::set_category(d, e4, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e4, EVENT_CUSTOM + ID);
    effect::set_target(d, e4, Some(cost_target(damtg)));
    effect::set_operation(d, e4, Some(operation(damop)));
    card::register_effect(d, c, e4, false);
    let e5 = effect::create_effect(d, c);
    effect::set_category(d, e5, CATEGORY_DESTROY);
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e5, EVENT_ADD_COUNTER + COUNTER_SPELL);
    effect::set_condition(d, e5, Some(condition(descon)));
    effect::set_target(d, e5, Some(cost_target(destg)));
    effect::set_operation(d, e5, Some(operation(desop)));
    card::register_effect(d, c, e5, false);
}
fn acop(d: &mut Duel, a: &EffArgs) {
    let p = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_PLAYER])[0].int() as u8;
    let c = effect::get_handler(d, a.e).nil();
    let re = a.re.nil();
    if effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && effect::is_spell_effect(d, re) && card::get_flag_effect(d, c, 1) > 0 {
        card::add_counter(d, c, COUNTER_SPELL as u16, 1, false);
        if card::get_counter(d, c, COUNTER_SPELL as u16) == 4 {
            duel::raise_single_event(d, c, EVENT_CUSTOM + ID, a.re, 0, 0, p, 0);
        }
    }
}
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_relate_to_effect(d, c, a.e);
    }
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, a.ep, 3000);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(c.into()), 1, 0, 0);
    true
}
fn damop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    if duel::destroy(d, c, REASON_EFFECT, None, None) != 0 {
        duel::damage(d, a.ep, 3000, REASON_EFFECT, false, None);
    }
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    let c = effect::get_handler(d, a.e).nil();
    effect::get_handler(d, re).nil() != c && card::get_counter(d, c, COUNTER_SPELL as u16) >= 4
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(c.into()), 1, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    duel::destroy(d, c, REASON_EFFECT, None, None);
}
