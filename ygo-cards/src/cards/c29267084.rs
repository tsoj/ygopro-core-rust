//! Shadow Spell
use super::prelude::*;

pub const ID: u32 = 29267084;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    aux::add_persistent_procedure(
        d,
        c,
        Some(1),
        Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_faceup(d, c))),
        Some(CATEGORY_POSITION),
        Some(EFFECT_FLAG_DAMAGE_STEP),
        Some(TIMING_DAMAGE_STEP),
        Some(TIMING_DAMAGE_STEP),
        Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))),
        None,
        None,
        None,
        false,
    );
    // eff
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CANNOT_CHANGE_POSITION);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e1, Some(aux::persistent_target_filter()));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_CANNOT_ATTACK);
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e1);
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    effect::set_value(d, e3, -700);
    card::register_effect(d, c, e3, false);
    // Destroy
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    effect::set_range(d, e4, LOCATION_SZONE);
    effect::set_code(d, e4, EVENT_LEAVE_FIELD);
    effect::set_condition(d, e4, Some(condition(descon)));
    effect::set_operation(d, e4, Some(operation(desop)));
    card::register_effect(d, c, e4, false);
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsStatus(STATUS_DESTROY_CONFIRMED) then return false end
    if card::is_status(d, c, STATUS_DESTROY_CONFIRMED) {
        return false;
    }
    // local tc=c:GetFirstCardTarget()
    // return tc and eg:IsContains(tc)
    match card::get_first_card_target(d, c) {
        Some(tc) => {
            let eg = a.eg.clone().nil();
            group::is_contains(&eg, tc)
        }
        None => false,
    }
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    duel::destroy(d, c, REASON_EFFECT, None, None);
}
