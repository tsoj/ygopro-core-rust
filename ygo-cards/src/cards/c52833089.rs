//! Black Thunder
use super::prelude::*;

pub const ID: u32 = 52833089;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BLACKWING], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYED);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_BLACKWING as u16])
        && card::is_location(d, c, LOCATION_GRAVE)
        && card::is_previous_controler(d, c, tp)
        && (card::get_reason(d, c) & REASON_BATTLE) != 0
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    group::is_exists(d, &a.eg(), &|d, c| cfilter(d, c, tp), 1, NO_EX)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, 0, LOCATION_ONFIELD) > 0;
    }
    duel::set_target_player(d, 1 - a.tp);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 400);
    true
}
fn activate(d: &mut Duel, _a: &EffArgs) {
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    let dmg = duel::get_field_group_count(d, p, LOCATION_ONFIELD, 0) as i64 * 400;
    duel::damage(d, p, dmg, REASON_EFFECT, false, None);
}
