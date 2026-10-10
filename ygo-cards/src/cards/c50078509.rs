//! Fiendish Chain
use super::prelude::*;

pub const ID: u32 = 50078509;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddPersistentProcedure(c,nil,s.filter,CATEGORY_DISABLE,nil,nil,TIMINGS_CHECK_MONSTER,nil,nil,s.target)
    aux::add_persistent_procedure(
        d,
        c,
        None,
        Some(aux::equip_filter_fn(|d, c, _e, _tp| filter(d, c))),
        Some(CATEGORY_DISABLE),
        None,
        None,
        Some(TIMINGS_CHECK_MONSTER),
        None,
        None,
        Some(std::sync::Arc::new(target)),
        None,
        false,
    );
    // disable
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_code(d, e3, EFFECT_DISABLE);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e3, Some(aux::persistent_target_filter()));
    card::register_effect(d, c, e3, false);
    // cannot attack
    let e4 = effect::clone(d, e3);
    effect::set_code(d, e4, EFFECT_CANNOT_ATTACK);
    card::register_effect(d, c, e4, false);
    // Destroy
    let e5 = effect::create_effect(d, c);
    effect::set_type(d, e5, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    effect::set_range(d, e5, LOCATION_SZONE);
    effect::set_code(d, e5, EVENT_LEAVE_FIELD);
    effect::set_condition(d, e5, Some(condition(descon)));
    effect::set_operation(d, e5, Some(operation(desop)));
    card::register_effect(d, c, e5, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_EFFECT)
}
fn target(d: &mut Duel, _a: &EffArgs, tc: Option<CardId>, chk: i64) -> bool {
    if chk == 0 {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, tc.map(CardSet::from_card), 1, 0, 0);
    true
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_status(d, c, STATUS_DESTROY_CONFIRMED) {
        return false;
    }
    match card::get_first_card_target(d, c) {
        Some(tc) => {
            let eg = a.eg.clone().nil();
            group::is_contains(&eg, tc) && card::is_reason(d, tc, REASON_DESTROY)
        }
        None => false,
    }
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    duel::destroy(d, c, REASON_EFFECT, None, None);
}
