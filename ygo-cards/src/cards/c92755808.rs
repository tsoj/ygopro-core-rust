//! Element Saurus
use super::prelude::*;

pub const ID: u32 = 92755808;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atk
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_value(d, e1, 500);
    effect::set_condition(d, e1, Some(condition(atkcon)));
    card::register_effect(d, c, e1, false);
    // disable
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_BATTLED);
    effect::set_condition(d, e2, Some(condition(discon)));
    effect::set_operation(d, e2, Some(operation(disop)));
    card::register_effect(d, c, e2, false);
}
/// `s.filter(c,att)`
fn filter(d: &mut Duel, c: CardId, att: u32) -> bool {
    card::is_faceup(d, c) && card::is_attribute(d, c, att)
}
/// `s.atkcon(e)`
fn atkcon(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, ATTRIBUTE_FIRE)), 0, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
}
/// `s.discon(e,tp,...)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // return bc and bc:IsStatus(STATUS_BATTLE_DESTROYED) and not c:IsStatus(STATUS_BATTLE_DESTROYED) and Duel.IsExistingMatchingCard(...)
    match bc {
        None => false,
        Some(bc) => {
            card::is_status(d, bc, STATUS_BATTLE_DESTROYED)
                && !card::is_status(d, c, STATUS_BATTLE_DESTROYED)
                && duel::is_existing_matching_card(d, Some(&|d, cc| filter(d, cc, ATTRIBUTE_EARTH)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
        }
    }
}
/// `s.disop(e,...)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_DISABLE);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_EXC_GRAVE, 1);
    card::register_effect(d, bc, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD_EXC_GRAVE, 1);
    card::register_effect(d, bc, e2, false);
}
