//! Morphtronic Clocken
use super::prelude::*;

pub const ID: u32 = 91607976;

pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[0x8], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_counter_permit(d, c, 0x8, Some(LOCATION_MZONE), None);
    // atk
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_condition(d, e1, Some(lua_fn(cona)));
    effect::set_value_fn(d, e1, effect_card_value(vala));
    card::register_effect(d, c, e1, false);
    // counter
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_COUNTER);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_condition(d, e2, Some(condition(cond1)));
    effect::set_operation(d, e2, Some(operation(opd1)));
    card::register_effect(d, c, e2, false);
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_DAMAGE);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_condition(d, e3, Some(condition(cond2)));
    effect::set_cost(d, e3, Some(cost_target(costd2)));
    effect::set_target(d, e3, Some(cost_target(tgd2)));
    effect::set_operation(d, e3, Some(operation(opd2)));
    card::register_effect(d, c, e3, false);
}
fn cona(d: &mut Duel, a: Args) -> bool {
    let c = effect::get_handler(d, a.e(0)).nil();
    card::is_attack_pos(d, c)
}
/// `function(e,c) return c:GetCounter(0x8)*500 end`
fn vala(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    (card::get_counter(d, c, 0x8) * 500) as i64
}
fn cond1(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    !card::is_disabled(d, h) && card::is_defense_pos(d, h)
}
fn opd1(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) {
        card::add_counter(d, h, 0x8, 1, false);
    }
}
fn cond2(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    !card::is_disabled(d, h) && card::is_defense_pos(d, h)
}
fn costd2(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_releasable(d, h, None, None);
    }
    let n = card::get_counter(d, h, 0x8) * 1000;
    effect::set_label(d, a.e, &[n as i64]);
    duel::release(d, h, REASON_COST, None);
    true
}
fn tgd2(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::get_counter(d, h, 0x8) > 0;
    }
    let label = effect::get_label(d, a.e);
    duel::set_target_player(d, 1 - a.tp);
    duel::set_target_param(d, label as u32);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, label as i32);
    true
}
fn opd2(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
