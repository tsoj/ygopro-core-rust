//! Shining Elf
use super::prelude::*;

pub const ID: u32 = 97170107;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    xyz::add_procedure(d, c, None, Some(2), 2, None, None, None, None, false, None);
    card::enable_revive_limit(d, c);
    // atkdown
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    card::register_effect(d, c, e2, false);
}
/// `s.filter(c,e,tp)`: `e` is nil in the target check
fn filter(d: &mut Duel, c: CardId, e: Option<EffectId>, tp: u8) -> bool {
    card::is_faceup(d, c)
        && card::is_controler(d, c, 1 - tp)
        && card::get_attack(d, c) > 0
        && match e {
            None => true,
            Some(e) => card::is_relate_to_effect(d, c, e),
        }
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    if a.chk == Some(0) {
        return group::is_exists(d, &eg, &|d, c| filter(d, c, None, a.tp), 1, NO_EX);
    }
    duel::set_target_card(d, eg);
    true
}
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| filter(d, c, Some(a.e), a.tp), NO_EX);
    let _ = group::get_first(d, &g);
    let c = effect::get_handler(d, a.e).nil();
    for tc in group::iter(d, &g) {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_value(d, e1, -500);
        effect::set_reset(d, e1, RESET_EVENT + RESETS_STANDARD, 1);
        card::register_effect(d, tc, e1, false);
    }
}
