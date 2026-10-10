//! Burst Stream of Destruction
use super::prelude::*;

pub const ID: u32 = 17655904;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_BLUEEYES_W_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    duel::add_custom_activity_counter(d, ID, ACTIVITY_ATTACK, lua_fn(counterfilter));
}
fn counterfilter(d: &mut Duel, a: Args) -> bool {
    let c = a.c(0);
    !card::is_code(d, c, &[CARD_BLUEEYES_W_DRAGON])
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[CARD_BLUEEYES_W_DRAGON]))), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_custom_activity_count(d, ID, a.tp, ACTIVITY_ATTACK) == 0;
    }
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    effect::set_property(d, e1, EFFECT_FLAG_OATH + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_code(d, c, &[CARD_BLUEEYES_W_DRAGON]))));
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
    let e2 = effect::create_effect(d, h);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    effect::set_target_range(d, e2, 1, 0);
    duel::register_effect(d, e2, a.tp);
    true
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, None, a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    let sg = duel::get_matching_group(d, None, a.tp, 0, LOCATION_MZONE, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(sg.clone()), sg.len() as u8, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let sg = duel::get_matching_group(d, None, a.tp, 0, LOCATION_MZONE, NO_EX);
    duel::destroy(d, sg, REASON_EFFECT, None, None);
}
