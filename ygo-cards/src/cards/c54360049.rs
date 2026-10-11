//! Catoblepas and the Witch of Fate
use super::prelude::*;

pub const ID: u32 = 54360049;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // destroy
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::get_text_attack(d, c) == -2 && card::is_able_to_remove_as_cost(d, c, None) && card::is_monster(d, c) && aux::sp_elim_filter(d, c, true, false)
}
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&cfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::get_summon_player(d, c) != tp
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    if a.chk == Some(0) {
        return group::is_exists(d, &eg, &|d, c| filter(d, c, a.tp), 1, NO_EX);
    }
    let g = group::filter(d, &eg, &|d, c| filter(d, c, a.tp), NO_EX);
    duel::set_target_card(d, eg.clone());
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), g.len() as u8, 0, 0);
    true
}
fn dfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    !card::is_summon_player(d, c, tp) && card::is_relate_to_effect(d, c, e)
}
fn op(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg();
    let g = group::filter(d, &eg, &|d, c| dfilter(d, c, a.e, a.tp), NO_EX);
    if !g.is_empty() {
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
