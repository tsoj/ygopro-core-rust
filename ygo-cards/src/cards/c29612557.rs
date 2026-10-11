//! Cyclone Boomerang
use super::prelude::*;

pub const ID: u32 = 29612557;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[86188410], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsCode,86188410))
    aux::add_equip_procedure(d, c, None, Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_code(d, c, &[86188410]))), None, None, None, None, None, None);
    // equip effect
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    effect::set_value(d, e3, 500);
    card::register_effect(d, c, e3, false);
    // destroy
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 0));
    effect::set_category(d, e4, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e4, EVENT_LEAVE_FIELD);
    effect::set_condition(d, e4, Some(condition(descon)));
    effect::set_target(d, e4, Some(cost_target(destg)));
    effect::set_operation(d, e4, Some(operation(desop)));
    card::register_effect(d, c, e4, false);
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_reason(d, h, REASON_LOST_TARGET) {
        return false;
    }
    let ec = card::get_previous_equip_target(d, h).nil();
    card::is_location(d, ec, LOCATION_GRAVE) && (card::get_reason(d, ec) & (REASON_DESTROY | REASON_EFFECT)) == (REASON_DESTROY | REASON_EFFECT)
}
fn dfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_spell_trap(d, c)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let g = duel::get_matching_group(d, Some(&dfilter), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    let n = group::get_count(&g);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n as u8, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, (n * 100) as i32);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_matching_group(d, Some(&dfilter), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    let ct = duel::destroy(d, g, REASON_EFFECT, None, None);
    duel::damage(d, 1 - a.tp, ct as i64 * 100, REASON_EFFECT, false, None);
}
