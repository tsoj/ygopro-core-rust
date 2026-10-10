//! Shattered Axe
use super::prelude::*;

pub const ID: u32 = 12117532;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddPersistentProcedure(c,nil,aux.FilterBoolFunction(Card.IsFaceup),CATEGORY_DISABLE,nil,nil,TIMING_END_PHASE,nil,nil,s.target)
    aux::add_persistent_procedure(
        d,
        c,
        None,
        Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_faceup(d, c))),
        Some(CATEGORY_DISABLE),
        None,
        None,
        Some(TIMING_END_PHASE),
        None,
        None,
        Some(std::sync::Arc::new(target)),
        None,
        false,
    );
    // Destroy
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_code(d, e2, EVENT_LEAVE_FIELD);
    effect::set_condition(d, e2, Some(condition(descon)));
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
    // atkdown
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e3, EVENT_PHASE | PHASE_STANDBY);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_condition(d, e3, Some(condition(atkcon)));
    effect::set_operation(d, e3, Some(operation(atkop)));
    card::register_effect(d, c, e3, false);
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    effect::set_code(d, e4, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e4, LOCATION_SZONE);
    effect::set_target_range(d, e4, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e4, Some(aux::persistent_target_filter()));
    effect::set_value_fn(d, e4, effect_card_value(val));
    card::register_effect(d, c, e4, false);
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
        Some(tc) => a.eg.clone().nil().contains(tc) && card::is_reason(d, tc, REASON_DESTROY),
        None => false,
    }
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    duel::destroy(d, c, REASON_EFFECT, None, None);
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, a.tp) && card::get_first_card_target(d, effect::get_handler(d, a.e).nil()).is_some()
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if let Some(tc) = card::get_first_card_target(d, c) {
        card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 0, 0, 0);
    }
}
fn val(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    card::get_flag_effect(d, c, ID) as i64 * -500
}
