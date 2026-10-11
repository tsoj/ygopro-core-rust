//! Balloon Lizard
use super::prelude::*;

pub const ID: u32 = 39892082;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(0x29)
    card::enable_counter_permit(d, c, 0x29, None, None);
    // counter
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_COUNTER);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(addccon)));
    effect::set_target(d, e1, Some(cost_target(addct)));
    effect::set_operation(d, e1, Some(operation(addc)));
    card::register_effect(d, c, e1, false);
    // damage
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_LEAVE_FIELD_P);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_operation(d, e2, Some(operation(regop)));
    card::register_effect(d, c, e2, false);
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_DAMAGE);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e3, EVENT_DESTROYED);
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_condition(d, e3, Some(condition(damcon)));
    effect::set_target(d, e3, Some(cost_target(damtg)));
    effect::set_operation(d, e3, Some(operation(damop)));
    effect::set_label_object(d, e3, Value::Effect(e2));
    card::register_effect(d, c, e3, false);
}
/// `s.addccon(e,tp,...)`
fn addccon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, a.tp)
}
/// `s.addct(e,tp,...,chk)`
fn addct(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, None, 1, 0, 0x29);
    true
}
/// `s.addc(e,tp,...)`
fn addc(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) {
        card::add_counter(d, h, 0x29, 1, false);
    }
}
/// `s.regop(e,tp,...)`
fn regop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    let ct = card::get_counter(d, h, 0x29);
    effect::set_label(d, a.e, &[ct as i64]);
}
/// `s.damcon(e,tp,...)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    let ct = effect::get_label(d, lo);
    effect::set_label(d, a.e, &[ct]);
    ct > 0
}
/// `s.damtg(e,tp,...,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let v = effect::get_label(d, a.e) * 400;
    duel::set_target_player(d, a.rp);
    duel::set_target_param(d, v as u32);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, a.rp, v as i32);
    true
}
/// `s.damop(e,tp,...)`
fn damop(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
