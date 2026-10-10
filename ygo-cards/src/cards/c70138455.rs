//! Blast Juggler
use super::prelude::*;

pub const ID: u32 = 70138455;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // destroy
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_TRIGGER_O + EFFECT_TYPE_FIELD);
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_cost(d, e1, Some(cost::self_tribute()));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    a.tp == duel::get_turn_player(d)
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_attack_below(d, c, 1000)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc);
    }
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, LOCATION_MZONE, 2, c.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    let g = duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, LOCATION_MZONE, 2, 2, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 2, 0, 0);
    true
}
fn desfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    card::is_relate_to_effect(d, c, e) && card::is_faceup(d, c) && card::is_attack_below(d, c, 1000)
}
fn op(d: &mut Duel, a: &EffArgs) {
    let g = chain::get_target_cards(d, 0).as_group().cloned().nil();
    let sg = group::filter(d, &g, &|d, x| desfilter(d, x, a.e), NO_EX);
    duel::destroy(d, sg, REASON_EFFECT, None, None);
}
