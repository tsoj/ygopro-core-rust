//! Evilswarm Nightmare
use super::prelude::*;

pub const ID: u32 = 359563;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    xyz::add_procedure(d, c, Some(mat_filter(|d, x, sc, st, tp| card::is_attribute_ctx(d, x, ATTRIBUTE_DARK, sc, st, Some(tp)))), Some(4), 2, None, None, None, None, false, None);
    card::enable_revive_limit(d, c);
    // pos
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_POSITION + CATEGORY_SET);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId, e: Option<EffectId>, tp: u8) -> bool {
    card::is_faceup(d, c) && card::is_can_turn_set(d, c) && card::is_summon_player(d, c, 1 - tp) && match e {
        None => true,
        Some(e) => card::is_relate_to_effect(d, c, e),
    }
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    if a.chk == Some(0) {
        return group::is_exists(d, &eg, &|d, c| filter(d, c, None, a.tp), 1, NO_EX);
    }
    duel::set_target_card(d, eg.clone());
    let n = eg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(eg), n, a.tp, POS_FACEDOWN_DEFENSE as i32);
    true
}
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let g = group::filter(d, &a.eg.clone().nil(), &|d, c| filter(d, c, Some(a.e), a.tp), NO_EX);
    duel::change_position(d, g, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
}
