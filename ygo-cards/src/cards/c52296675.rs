//! Anchamoufrite
use super::prelude::*;

pub const ID: u32 = 52296675;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    pendulum::add_procedure(d, c, None, None);
    // draw
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_PZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
    // spsummon condition
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e2, EFFECT_SPSUMMON_CONDITION);
    card::register_effect(d, c, e2, false);
    // special summon
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_code(d, e3, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_count_limit(d, e3, 1, ID, 1, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_range(d, e3, LOCATION_EXTRA);
    effect::set_condition(d, e3, Some(lua_fn(spcon)));
    card::register_effect(d, c, e3, false);
    // redirect
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e4, EFFECT_LEAVE_FIELD_REDIRECT);
    effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_condition(d, e4, Some(lua_fn(recon)));
    effect::set_value(d, e4, LOCATION_REMOVED as i64);
    card::register_effect(d, c, e4, false);
}
/// `s.condition(e)`
fn cond_check(d: &mut Duel, e: EffectId) -> bool {
    duel::get_matching_group_count(d, Some(&|d, c| !card::is_code(d, c, &[ID])), effect::get_handler_player(d, e), LOCATION_EXTRA, 0, NO_EX) == 0
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    cond_check(d, a.e)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1);
    }
    duel::set_target_player(d, a.tp);
    duel::set_target_param(d, 1);
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from(h)), 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let (p, dd) = (info[0].int() as u8, info[1].int() as u32);
    if duel::destroy(d, h, REASON_EFFECT, None, None) > 0 {
        duel::break_effect(d);
        duel::draw(d, p, dd, REASON_EFFECT);
    }
}
fn spcon(d: &mut Duel, a: Args) -> bool {
    let Some(c) = a.card(1) else { return true };
    let e = a.e(0);
    let tp = effect::get_handler_player(d, e);
    cond_check(d, e) && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
}
fn recon(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_faceup(d, h) && card::is_location(d, h, LOCATION_MZONE)
}
