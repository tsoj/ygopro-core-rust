//! Spellbinding Circle
use super::prelude::*;

pub const ID: u32 = 18807108;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddPersistentProcedure(c,1,nil,CATEGORY_POSITION,nil,nil,nil,nil,nil,nil,nil,true)
    aux::add_persistent_procedure(d, c, Some(1), None, Some(CATEGORY_POSITION), None, None, None, None, None, None, None, true);
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_CHANGE_POSITION);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e1, Some(aux::persistent_target_filter()));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_CANNOT_ATTACK);
    card::register_effect(d, c, e2, false);
    // Destroy
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_code(d, e3, EVENT_LEAVE_FIELD);
    effect::set_condition(d, e3, Some(condition(descon)));
    effect::set_operation(d, e3, Some(operation(desop)));
    card::register_effect(d, c, e3, false);
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_status(d, c, STATUS_DESTROY_CONFIRMED) {
        return false;
    }
    let tc = match card::get_first_card_target(d, c) {
        Some(tc) => tc,
        None => return false,
    };
    a.eg.clone().nil().contains(tc) && card::is_reason(d, tc, REASON_DESTROY) && card::is_relate_to_card(d, c, tc)
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    duel::destroy(d, c, REASON_EFFECT, None, None);
}
