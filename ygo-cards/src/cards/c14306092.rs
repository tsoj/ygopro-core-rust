//! Shark Caesar
use super::prelude::*;

pub const ID: u32 = 14306092;

pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[0x2e], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(0x2e)
    card::enable_counter_permit(d, c, 0x2e, None, None);
    // xyz summon
    // Xyz.AddProcedure(c,nil,3,3,nil,nil,5)
    xyz::add_procedure(d, c, None, Some(3), 3, None, None, Some(xyz::MaxCt::N(5)), None, false, None);
    card::enable_revive_limit(d, c);
    // counter
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_COUNTER);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    effect::set_operation(d, e1, Some(operation(ctop)));
    card::register_effect(d, c, e1, false);
    // atkup
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_condition(d, e2, Some(condition(atkcon)));
    effect::set_value_fn(d, e2, effect_card_value(atkval));
    card::register_effect(d, c, e2, false);
}
fn ctop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        card::add_counter(d, c, 0x2e, 1, false);
    }
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let ph = duel::get_current_phase(d);
    let h = effect::get_handler(d, a.e).nil();
    (ph == PHASE_DAMAGE || ph == PHASE_DAMAGE_CAL) && (duel::get_attacker(d) == Some(h) || duel::get_attack_target(d) == Some(h))
}
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    card::get_counter(d, c, 0x2e) as i64 * 1000
}
